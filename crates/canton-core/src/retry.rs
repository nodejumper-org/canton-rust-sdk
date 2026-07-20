//! Opt-in retry with exponential backoff.
//!
//! Retries fire only on [`crate::Error::is_retriable`] errors, up to a bounded
//! number of attempts. Callers build the request once (so a retried command
//! keeps the same `command_id` and stays de-duplication-safe) and pass an
//! operation that re-runs the RPC.

use std::future::Future;
use std::time::Duration;

use crate::Result;

/// Retry policy for unary calls.
///
/// Start from [`RetryConfig::default`] and adjust with the fluent setters;
/// `#[non_exhaustive]` so fields can be added without a breaking change.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct RetryConfig {
    /// Maximum number of attempts (including the first). `1` disables retrying.
    pub max_attempts: u32,
    /// Backoff before the first retry.
    pub initial_backoff: Duration,
    /// Upper bound the backoff doubles towards.
    pub max_backoff: Duration,
    /// Optional per-attempt timeout. When set, an attempt that exceeds it is
    /// cancelled and treated as a retriable timeout (bounding a hung call
    /// independently of the channel-level timeout).
    pub attempt_timeout: Option<Duration>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(5),
            attempt_timeout: None,
        }
    }
}

impl RetryConfig {
    /// Set the maximum number of attempts (including the first).
    #[must_use]
    pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Set the initial backoff before the first retry.
    #[must_use]
    pub fn with_initial_backoff(mut self, initial_backoff: Duration) -> Self {
        self.initial_backoff = initial_backoff;
        self
    }

    /// Set the ceiling the backoff doubles towards.
    #[must_use]
    pub fn with_max_backoff(mut self, max_backoff: Duration) -> Self {
        self.max_backoff = max_backoff;
        self
    }

    /// Set a per-attempt timeout: an attempt exceeding it is cancelled and
    /// retried (as a timeout) rather than blocking on the channel-level timeout.
    #[must_use]
    pub fn with_attempt_timeout(mut self, attempt_timeout: Duration) -> Self {
        self.attempt_timeout = Some(attempt_timeout);
        self
    }
}

/// Run `op`, retrying on retriable errors per `config`. With no `config`, runs
/// `op` exactly once.
///
/// # Errors
/// Returns the last error from `op` once attempts are exhausted or a
/// non-retriable error is hit.
pub async fn run_with_retry<T, F, Fut>(config: Option<&RetryConfig>, mut op: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let Some(config) = config else {
        return op().await;
    };

    let mut attempt = 1u32;
    let mut backoff = config.initial_backoff;
    loop {
        // Bound the attempt if a per-attempt timeout is configured; a timeout is
        // a retriable outcome (the op is retried like any transient failure).
        let outcome = match config.attempt_timeout {
            Some(timeout) => match tokio::time::timeout(timeout, op()).await {
                Ok(result) => result,
                Err(_) => Err(crate::Error::Timeout),
            },
            None => op().await,
        };
        match outcome {
            Ok(value) => return Ok(value),
            Err(err) if err.is_retriable() && attempt < config.max_attempts => {
                tokio::time::sleep(with_jitter(backoff)).await;
                backoff = (backoff * 2).min(config.max_backoff);
                attempt += 1;
            }
            Err(err) => return Err(err),
        }
    }
}

/// Equal jitter (×0.5–1.5) on a backoff delay, so retries from many clients
/// failing at once do not re-arrive in lockstep (thundering herd). Uses the
/// clock's sub-second nanos as a cheap entropy source — no RNG dependency.
fn with_jitter(backoff: Duration) -> Duration {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let factor = 0.5 + f64::from(nanos % 1024) / 1024.0;
    backoff.mul_f64(factor)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::Error;
    use std::cell::Cell;

    fn fast() -> RetryConfig {
        RetryConfig::default()
            .with_initial_backoff(Duration::from_millis(1))
            .with_max_backoff(Duration::from_millis(1))
    }

    #[tokio::test]
    async fn retries_retriable_errors_then_succeeds() {
        let calls = Cell::new(0);
        let result: Result<u32> = run_with_retry(Some(&fast()), || {
            calls.set(calls.get() + 1);
            let n = calls.get();
            async move { if n < 3 { Err(Error::Timeout) } else { Ok(n) } }
        })
        .await;

        assert_eq!(result.unwrap(), 3);
        assert_eq!(calls.get(), 3);
    }

    #[tokio::test]
    async fn does_not_retry_non_retriable_errors() {
        let calls = Cell::new(0);
        let result: Result<u32> = run_with_retry(Some(&fast()), || {
            calls.set(calls.get() + 1);
            async move { Err(Error::InvalidRequest("nope".to_string())) }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(calls.get(), 1, "non-retriable errors are not retried");
    }

    #[tokio::test]
    async fn gives_up_after_max_attempts() {
        let calls = Cell::new(0);
        let result: Result<u32> = run_with_retry(Some(&fast()), || {
            calls.set(calls.get() + 1);
            async move { Err(Error::Timeout) }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(calls.get(), 3, "stops at max_attempts");
    }

    #[tokio::test]
    async fn attempt_timeout_bounds_a_hung_attempt() {
        let calls = Cell::new(0);
        let config = fast()
            .with_max_attempts(2)
            .with_attempt_timeout(Duration::from_millis(5));
        let result: Result<u32> = run_with_retry(Some(&config), || {
            calls.set(calls.get() + 1);
            async move {
                // Hangs well past the per-attempt timeout, so each attempt is
                // cancelled and retried as a timeout.
                tokio::time::sleep(Duration::from_secs(30)).await;
                Ok(1)
            }
        })
        .await;

        assert!(result.is_err(), "every attempt times out");
        assert_eq!(calls.get(), 2, "the hung attempt is bounded and retried");
    }

    #[tokio::test]
    async fn without_config_runs_exactly_once() {
        let calls = Cell::new(0);
        let result: Result<u32> = run_with_retry(None, || {
            calls.set(calls.get() + 1);
            async move { Err(Error::Timeout) }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(calls.get(), 1);
    }
}
