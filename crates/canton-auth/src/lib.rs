//! JWT/OIDC authentication for the Canton Ledger API.
//!
//! Provides the OAuth2 **client-credentials** flow with a [`TokenProvider`]
//! that caches the access token and refreshes it before expiry. Secrets are
//! redacted from `Debug` output.
//!
//! ```no_run
//! # async fn run() -> canton_core::Result<()> {
//! use canton_auth::{OidcConfig, TokenProvider};
//!
//! let provider = TokenProvider::new(OidcConfig::new(
//!     "http://localhost:8082/realms/AppProvider/protocol/openid-connect/token",
//!     "app-provider-backend",
//!     "…",
//! ));
//! let bearer = provider.token().await?;
//! # let _ = bearer;
//! # Ok(())
//! # }
//! ```

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use canton_core::{Error, Result, TokenSource};
use serde::Deserialize;
use tokio::sync::Mutex;

/// Refresh a token this many seconds before its stated expiry, to avoid racing
/// the deadline on in-flight requests.
const REFRESH_SKEW: u64 = 30;

/// Fallback token lifetime (seconds) when the endpoint omits `expires_in`, so a
/// missing/zero value does not collapse to a 1-second TTL that hammers the IdP.
const DEFAULT_TTL_SECS: u64 = 300;

/// Upper bound on a cached token's TTL (30 days). Caps an absurd or hostile
/// `expires_in` so `Instant::now() + Duration::from_secs(ttl)` can never overflow
/// (which panics); a real token re-fetches long before this.
const MAX_TTL_SECS: u64 = 30 * 24 * 60 * 60;

/// The cache TTL (seconds) for a token whose endpoint reported `expires_in`:
/// a missing/zero value uses the default lifetime, a refresh skew is subtracted,
/// and the result is clamped to `[1, MAX_TTL_SECS]` — the upper bound guarding
/// against an overflow panic when computing the cache deadline.
fn effective_ttl(expires_in: u64) -> u64 {
    let lifetime = if expires_in == 0 {
        DEFAULT_TTL_SECS
    } else {
        expires_in
    };
    lifetime.saturating_sub(REFRESH_SKEW).clamp(1, MAX_TTL_SECS)
}

/// Per-request bound on a token fetch: a hung IdP fails the fetch (retriable
/// [`Error::Connection`]) instead of blocking all token consumers behind the
/// cache lock.
const FETCH_TIMEOUT: Duration = Duration::from_secs(30);

/// OIDC client-credentials configuration for a token endpoint.
///
/// Construct with [`OidcConfig::new`]; `#[non_exhaustive]` so fields can be
/// added without a breaking change.
#[derive(Clone)]
#[non_exhaustive]
pub struct OidcConfig {
    token_url: String,
    client_id: String,
    // Deliberately private with no getter: the secret is write-only from the
    // caller's perspective (used internally for the token fetch, redacted from
    // `Debug`), so it cannot leak via `println!`/serialization by accident.
    client_secret: String,
    scope: Option<String>,
}

impl OidcConfig {
    /// Create a client-credentials configuration (no scope).
    #[must_use]
    pub fn new(
        token_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            token_url: token_url.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            scope: None,
        }
    }

    /// Set the OAuth2 scope.
    #[must_use]
    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// The OAuth2 token endpoint URL.
    #[must_use]
    pub fn token_url(&self) -> &str {
        &self.token_url
    }

    /// The OAuth2 client id.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Preset for **Keycloak** (also the Canton LocalNet IdP): builds the
    /// `{base_url}/realms/{realm}/protocol/openid-connect/token` endpoint.
    #[must_use]
    pub fn keycloak(
        base_url: impl AsRef<str>,
        realm: impl AsRef<str>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        let base = base_url.as_ref().trim_end_matches('/');
        Self::new(
            format!(
                "{base}/realms/{}/protocol/openid-connect/token",
                realm.as_ref()
            ),
            client_id,
            client_secret,
        )
    }

    /// Preset for **Auth0**: builds the `https://{domain}/oauth/token` endpoint.
    #[must_use]
    pub fn auth0(
        domain: impl AsRef<str>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        let domain = domain.as_ref().trim_end_matches('/');
        Self::new(
            format!("https://{domain}/oauth/token"),
            client_id,
            client_secret,
        )
    }

    /// Preset for **Okta**: builds the
    /// `https://{domain}/oauth2/{auth_server}/v1/token` endpoint (use
    /// `"default"` for the default authorization server).
    #[must_use]
    pub fn okta(
        domain: impl AsRef<str>,
        auth_server: impl AsRef<str>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        let domain = domain.as_ref().trim_end_matches('/');
        Self::new(
            format!("https://{domain}/oauth2/{}/v1/token", auth_server.as_ref()),
            client_id,
            client_secret,
        )
    }
}

impl fmt::Debug for OidcConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OidcConfig")
            .field("token_url", &self.token_url)
            .field("client_id", &self.client_id)
            .field("client_secret", &"<redacted>")
            .field("scope", &self.scope)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    expires_in: u64,
}

struct Cached {
    token: String,
    deadline: Instant,
}

/// Fetches and caches an OAuth2 bearer token via the client-credentials grant,
/// refreshing it shortly before expiry. Cloning shares the cache.
#[derive(Clone)]
pub struct TokenProvider {
    inner: Arc<Inner>,
}

struct Inner {
    config: OidcConfig,
    http: reqwest::Client,
    cache: Mutex<Option<Cached>>,
}

impl fmt::Debug for TokenProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenProvider")
            .field("config", &self.inner.config)
            .finish_non_exhaustive()
    }
}

impl TokenProvider {
    /// Create a provider for the given OIDC configuration.
    ///
    /// Token fetches are bounded by a per-request timeout (see
    /// [`Self::token`]), so a hung token endpoint can never block callers
    /// indefinitely (the fetch holds the cache lock while in flight).
    #[must_use]
    pub fn new(config: OidcConfig) -> Self {
        // The per-request timeout in `fetch` is the guarantee; the client-level
        // timeout here is belt-and-braces (a bare builder with a timeout does
        // not fail in practice, but the fallback stays bounded either way).
        let http = reqwest::Client::builder()
            .timeout(FETCH_TIMEOUT)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            inner: Arc::new(Inner {
                config,
                http,
                cache: Mutex::new(None),
            }),
        }
    }

    /// Drop the cached token so the next [`Self::token`] call fetches a fresh
    /// one. Call this after a server rejects a token as expired/invalid so the
    /// client can self-heal instead of replaying the stale token.
    pub async fn invalidate(&self) {
        *self.inner.cache.lock().await = None;
    }

    /// Return a valid bearer token, fetching or refreshing if the cached one is
    /// absent or within the refresh-skew window of expiry.
    ///
    /// Concurrent callers that arrive during a refresh serialize behind a single
    /// in-flight fetch (single-flight de-duplication), bounded by the HTTP
    /// client's timeout — a slow IdP therefore stalls concurrent token consumers
    /// for at most that timeout rather than triggering a fetch storm.
    ///
    /// # Errors
    /// Returns [`Error::Auth`] if the endpoint rejects the credentials
    /// (401/403, e.g. `invalid_client`); [`Error::Http`] for other non-success
    /// statuses (5xx/429 stay retriable); [`Error::Connection`] if the endpoint
    /// is unreachable or the fetch times out; [`Error::Json`] if the response
    /// cannot be parsed.
    pub async fn token(&self) -> Result<String> {
        let mut guard = self.inner.cache.lock().await;
        if let Some(cached) = guard.as_ref()
            && Instant::now() < cached.deadline
        {
            return Ok(cached.token.clone());
        }

        let response = self.fetch().await?;
        let ttl = effective_ttl(response.expires_in);
        let token = response.access_token;
        *guard = Some(Cached {
            token: token.clone(),
            deadline: Instant::now() + Duration::from_secs(ttl),
        });
        Ok(token)
    }

    async fn fetch(&self) -> Result<TokenResponse> {
        let config = &self.inner.config;
        let mut params = vec![
            ("grant_type", "client_credentials"),
            ("client_id", config.client_id.as_str()),
            ("client_secret", config.client_secret.as_str()),
        ];
        if let Some(scope) = &config.scope {
            params.push(("scope", scope.as_str()));
        }

        // A send failure means the IdP was unreachable — retriable transport,
        // not a credential rejection. The per-request timeout bounds the fetch
        // even if the client was built without one.
        let response = self
            .inner
            .http
            .post(&config.token_url)
            .timeout(FETCH_TIMEOUT)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                Error::Connection(format!("token request to {} failed: {e}", config.token_url))
            })?;

        // A credential rejection (401/403, e.g. `invalid_client`) is a definite
        // auth failure; other non-success statuses keep their code so 5xx/429
        // stay retriable via the shared error model.
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            if matches!(status, 401 | 403) {
                return Err(Error::Auth(format!(
                    "token endpoint rejected the credentials (http {status}): {body}"
                )));
            }
            return Err(Error::Http { status, body });
        }

        let body = response
            .text()
            .await
            .map_err(|e| Error::Connection(format!("reading token response failed: {e}")))?;
        serde_json::from_str::<TokenResponse>(&body).map_err(Error::from)
    }
}

/// Lets a [`TokenProvider`] back the SDK's shared [`canton_core::Auth`] without
/// `canton-core` depending on this crate: `Config::with_oidc(provider)` stores
/// it as an `Arc<dyn TokenSource>`.
impl TokenSource for TokenProvider {
    fn fetch_bearer(&self) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send + '_>> {
        Box::pin(async move { self.token().await.map(Some) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> OidcConfig {
        OidcConfig::new("http://idp.example/token", "my-client", "TOP-SECRET-VALUE")
    }

    #[test]
    fn effective_ttl_subtracts_skew_and_clamps() {
        assert_eq!(effective_ttl(3600), 3600 - REFRESH_SKEW);
        // Missing/zero expires_in uses the default lifetime.
        assert_eq!(effective_ttl(0), DEFAULT_TTL_SECS - REFRESH_SKEW);
        // A tiny lifetime never collapses below 1.
        assert_eq!(effective_ttl(10), 1);
    }

    #[test]
    fn huge_expires_in_is_clamped_and_never_overflows() {
        // A buggy/hostile IdP reporting a giant expires_in must not panic when
        // the cache deadline is computed.
        for expires_in in [u64::MAX, 1_000_000_000_000_000_000] {
            assert_eq!(
                effective_ttl(expires_in),
                MAX_TTL_SECS,
                "clamped to the max"
            );
        }
        // Whatever the input, the ttl is bounded and the deadline the client
        // computes never overflows (the bug this guards against).
        for expires_in in [0, 10, 3600, MAX_TTL_SECS + 1, u64::MAX] {
            let ttl = effective_ttl(expires_in);
            assert!(ttl <= MAX_TTL_SECS);
            let _deadline = std::time::Instant::now() + std::time::Duration::from_secs(ttl);
        }
    }

    #[test]
    fn debug_redacts_the_client_secret() {
        let rendered = format!("{:?}", sample_config());
        assert!(
            !rendered.contains("TOP-SECRET-VALUE"),
            "client_secret must never appear in Debug output: {rendered}"
        );
        assert!(rendered.contains("<redacted>"));
        assert!(rendered.contains("my-client"));
    }

    #[test]
    fn provider_debug_does_not_leak_the_secret() {
        let rendered = format!("{:?}", TokenProvider::new(sample_config()));
        assert!(!rendered.contains("TOP-SECRET-VALUE"), "{rendered}");
    }

    #[test]
    fn presets_build_the_expected_token_urls() {
        assert_eq!(
            OidcConfig::keycloak("http://kc:8082/", "AppProvider", "c", "s").token_url,
            "http://kc:8082/realms/AppProvider/protocol/openid-connect/token"
        );
        assert_eq!(
            OidcConfig::auth0("my.eu.auth0.com", "c", "s").token_url,
            "https://my.eu.auth0.com/oauth/token"
        );
        assert_eq!(
            OidcConfig::okta("my.okta.com", "default", "c", "s").token_url,
            "https://my.okta.com/oauth2/default/v1/token"
        );
    }

    #[test]
    fn with_scope_sets_the_scope() {
        let config = OidcConfig::new("http://idp/token", "c", "s").with_scope("openid profile");
        assert_eq!(config.scope.as_deref(), Some("openid profile"));
    }
}
