//! The SDK-wide error type and [`Result`] alias.

/// The single error type for the whole Canton Rust SDK.
///
/// It is `#[non_exhaustive]` so new variants can be added without a breaking
/// change. Large upstream error types are boxed so that `Result<T, Error>`
/// stays cheap to move on the happy path.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// gRPC transport failure (DNS, TCP, TLS, HTTP/2). Retriable.
    #[error("transport error")]
    Transport(#[source] Box<tonic::transport::Error>),

    /// A non-gRPC connection failure (e.g. an HTTP/JSON or token-endpoint
    /// request that could not be sent). Retriable.
    #[error("connection error: {0}")]
    Connection(String),

    /// The server returned a gRPC status. The full [`tonic::Status`] is kept so
    /// callers can inspect the code, message, and metadata; see [`Error::code`].
    #[error("grpc status {}: {}", .0.code(), .0.message())]
    Status(#[source] Box<tonic::Status>),

    /// A non-success HTTP response from the JSON API or a token endpoint.
    /// Retriable for transient status codes (see [`Error::is_retriable`]).
    #[error("http {status}: {body}")]
    Http {
        /// The HTTP status code.
        status: u16,
        /// The response body (truncated by the caller if large).
        body: String,
    },

    /// JSON (de)serialization error.
    #[error("json error: {0}")]
    Json(#[source] Box<serde_json::Error>),

    /// A command was rejected by the ledger for business/interpretation
    /// reasons (as opposed to a transport failure). Not retriable: this is a
    /// terminal outcome of that submission read back from the completion
    /// stream, and re-submitting is an application decision — the automatic
    /// retry path (`submit*` RPC errors) surfaces rejections as
    /// [`Error::Status`] instead, with full category/retry-delay precision.
    #[error("command rejected ({code}): {message}")]
    CommandRejected {
        /// The rejection status code.
        code: String,
        /// The rejection message.
        message: String,
    },

    /// Authentication/authorization was rejected (bad or expired credentials).
    /// Not retriable — a token-transport failure surfaces as [`Error::Connection`]
    /// or [`Error::Http`] instead.
    #[error("authentication failed: {0}")]
    Auth(String),

    /// A request precondition or configuration value was invalid before send.
    #[error("invalid request: {0}")]
    InvalidRequest(String),

    /// The server's response was well-formed at the transport level but not
    /// what the protocol expects (e.g. a missing field, or a stream that ended
    /// unexpectedly). Not a caller-input error.
    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),

    /// The operation exceeded its configured deadline. Retriable.
    #[error("operation timed out")]
    Timeout,

    /// A typed payload failed to convert to or from the Ledger API `Value` —
    /// a `canton-daml` codec error. Not retriable: the shape will not change
    /// on a retry.
    // The cause is in the message *and* kept as `source`: every other variant
    // here prints its detail, and an application that logs `{err}` without
    // walking the chain would otherwise be told only that something failed.
    #[error("payload conversion failed: {0}")]
    Payload(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    /// The gRPC status code the participant answered with, on **either**
    /// transport.
    ///
    /// The JSON Ledger API reports it numerically as `grpcCodeValue`, so the
    /// same failure yields the same code whichever lane carried it — the HTTP
    /// status alone does not, since Canton maps several codes onto one status.
    /// `None` when there is no participant verdict to report: a transport
    /// failure, a timeout, or an HTTP body that is not a Canton error object
    /// (a proxy's error page, say).
    #[must_use]
    pub fn code(&self) -> Option<tonic::Code> {
        match self {
            Error::Status(status) => Some(status.code()),
            Error::Http { body, .. } => http_grpc_code(body),
            _ => None,
        }
    }

    /// Whether retrying the operation may succeed.
    ///
    /// For gRPC statuses, Canton's own verdict wins: every Ledger API error
    /// carries an [`ErrorCategory`] whose retryability is defined by the
    /// [error-code documentation], and retryable errors additionally carry a
    /// `google.rpc.RetryInfo` detail. Only when a status carries neither (a
    /// proxy in the middle, a non-Canton server) does the classification fall
    /// back to the transient gRPC codes (`Unavailable`, `DeadlineExceeded`,
    /// `ResourceExhausted`, `Aborted`).
    ///
    /// Beyond statuses, transient conditions are retriable: timeouts,
    /// transport/connection failures, and transient HTTP status codes
    /// (408, 429, 5xx). Everything else — invalid input, auth rejection,
    /// command rejection, `NotFound`/`AlreadyExists`, deserialization — is not.
    ///
    /// [error-code documentation]: https://docs.daml.com/canton/reference/error_codes.html
    #[must_use]
    pub fn is_retriable(&self) -> bool {
        use tonic::Code::{Aborted, DeadlineExceeded, ResourceExhausted, Unavailable};
        match self {
            Error::Timeout | Error::Transport(_) | Error::Connection(_) => true,
            Error::Status(status) => match status_category(status) {
                Some(category) => category.is_retriable(),
                // No category ⇒ not a Canton self-service error. A RetryInfo
                // detail is still an explicit "retry me"; else fall back to
                // the transient codes, plus a connection that died in flight.
                None => {
                    status_retry_delay(status).is_some()
                        || matches!(
                            status.code(),
                            Unavailable | DeadlineExceeded | ResourceExhausted | Aborted
                        )
                        || is_transport_death(status)
                }
            },
            // The JSON Ledger API carries the same verdict in the error body
            // (`errorCategory`, `retryInfo`); parse it before falling back to
            // the transient HTTP status codes.
            Error::Http { status, body } => match http_category(body) {
                Some(category) => category.is_retriable(),
                None => http_retry_delay(body).is_some() || matches!(status, 408 | 429 | 500..=599),
            },
            _ => false,
        }
    }

    /// The Canton [`ErrorCategory`] of this error, when it carries one: from
    /// `ErrorInfo.metadata["category"]` on a gRPC status, or the
    /// `errorCategory` field of a JSON API error body. This is the field the
    /// [error-code documentation] tells clients to base error handling on;
    /// [`Error::is_retriable`] already does.
    ///
    /// [error-code documentation]: https://docs.daml.com/canton/reference/error_codes.html
    #[must_use]
    pub fn category(&self) -> Option<ErrorCategory> {
        match self {
            Error::Status(status) => status_category(status),
            Error::Http { body, .. } => http_category(body),
            _ => None,
        }
    }

    /// The server-recommended delay before retrying, from the
    /// `google.rpc.RetryInfo` detail of a gRPC status or the `retryInfo`
    /// field of a JSON API error body. Canton attaches it to retryable
    /// errors; the retry helper ([`crate::retry::run_with_retry`]) already
    /// honours it.
    #[must_use]
    pub fn retry_delay(&self) -> Option<std::time::Duration> {
        match self {
            Error::Status(status) => status_retry_delay(status),
            Error::Http { body, .. } => http_retry_delay(body),
            _ => None,
        }
    }

    /// The correlation id of the failed request, from the
    /// `google.rpc.RequestInfo` detail of a gRPC status or the
    /// `correlationId`/`traceId` of a JSON API error body. Canton echoes it
    /// in every error; quote it when reporting a problem to the participant's
    /// operator, who can find the server-side trace by it.
    #[must_use]
    pub fn correlation_id(&self) -> Option<String> {
        match self {
            Error::Status(status) => {
                use tonic_types::StatusExt as _;
                status
                    .get_details_request_info()
                    .map(|info| info.request_id)
            }
            Error::Http { body, .. } => http_correlation_id(body),
            _ => None,
        }
    }

    /// The resources this error is about: which contract, package, party or
    /// synchronizer the participant is complaining of. Canton attaches these to
    /// the errors where "which one?" is the first question — `CONTRACT_NOT_FOUND`
    /// names the contract id, contention names the locked contracts.
    ///
    /// A `Vec` rather than an `Option` because the wire carries a list: the JSON
    /// Ledger API's `resources` is an array of `[type, name]` pairs, and one
    /// error can name several. The gRPC side yields at most one today — that is
    /// a limit of `tonic_types`, which models a single `google.rpc.ResourceInfo`
    /// detail, not of the protocol.
    #[must_use]
    pub fn resource_info(&self) -> Vec<ResourceInfo> {
        match self {
            Error::Status(status) => {
                use tonic_types::StatusExt as _;
                status
                    .get_details_resource_info()
                    .map(|info| ResourceInfo {
                        resource_type: info.resource_type,
                        resource_name: info.resource_name,
                        owner: info.owner,
                        description: info.description,
                    })
                    .into_iter()
                    .collect()
            }
            Error::Http { body, .. } => http_resource_info(body),
            _ => Vec::new(),
        }
    }

    /// The machine-readable identity of the failure: Canton's error `reason`
    /// (e.g. `DUPLICATE_COMMAND`) plus its context `metadata`. Prefer it over
    /// string-matching [`Display`](std::fmt::Display) output.
    ///
    /// Available on **either** transport. gRPC carries it as a
    /// `google.rpc.ErrorInfo` detail; the JSON Ledger API spells the same two
    /// things as `code` and `context`, and this reads whichever is there. That
    /// matters because the alternative on the JSON lane was the string matching
    /// this method exists to replace.
    ///
    /// `None` when the participant published no identity to report: a transport
    /// failure, a status without the detail, or a **redacted** error — Canton
    /// answers a security-sensitive failure with the literal `"NA"`, which is
    /// the absence of an error id rather than an error id.
    ///
    /// `domain` is empty on the JSON lane, and Canton leaves it empty on gRPC
    /// too.
    #[must_use]
    pub fn error_info(&self) -> Option<ErrorInfo> {
        match self {
            Error::Status(status) => {
                use tonic_types::StatusExt as _;
                status
                    .get_error_details()
                    .error_info()
                    .map(|info| ErrorInfo {
                        reason: info.reason.clone(),
                        domain: info.domain.clone(),
                        metadata: info.metadata.clone(),
                    })
            }
            Error::Http { body, .. } => http_error_info(body),
            _ => None,
        }
    }
}

/// The category of the status's `ErrorInfo`, when present and recognized.
/// Whether a status is a connection that died *mid-call*, rather than a verdict
/// the participant returned.
///
/// tonic maps a broken stream or a dropped unary connection to a `Status` with
/// code `Unknown` (message "transport error" / "h2 protocol error"), not to a
/// `transport::Error` — that variant is only for connection *establishment*. So
/// a real drop reaches [`Error::is_retriable`] as a category-less `Unknown` and,
/// without this, is treated as terminal: the resumable stream gives up on the
/// first blip and the retry pipeline never fires on the most ordinary failure
/// there is.
///
/// The signal is the source chain, not the message text (which is
/// tonic-version-specific). A client-side transport failure carries a `source`
/// — tonic built the status *from* the underlying h2/hyper/io error. A status
/// decoded from response trailers, which is how a genuine application `Unknown`
/// arrives, has none. So `Unknown` (or `Internal`, tonic's other transport-error
/// code) with a source present is a dropped connection; the same code without a
/// source is the server's own answer and stays terminal.
fn is_transport_death(status: &tonic::Status) -> bool {
    use std::error::Error as _;
    matches!(status.code(), tonic::Code::Unknown | tonic::Code::Internal)
        && status.source().is_some()
}

fn status_category(status: &tonic::Status) -> Option<ErrorCategory> {
    use tonic_types::StatusExt as _;
    let info = status.get_details_error_info()?;
    ErrorCategory::from_i32(info.metadata.get("category")?.parse().ok()?)
}

/// The `RetryInfo.retry_delay` of a status, when present.
fn status_retry_delay(status: &tonic::Status) -> Option<std::time::Duration> {
    use tonic_types::StatusExt as _;
    status.get_details_retry_info()?.retry_delay
}

/// The `errorCategory` of a JSON Ledger API error body, when present and
/// recognized. Non-JSON bodies (a proxy's HTML error page, a token endpoint)
/// simply yield `None`.
fn http_category(body: &str) -> Option<ErrorCategory> {
    let body: serde_json::Value = serde_json::from_str(body).ok()?;
    let id = body.get("errorCategory")?.as_i64()?;
    ErrorCategory::from_i32(i32::try_from(id).ok()?)
}

/// The `retryInfo` of a JSON Ledger API error body, when present. The field
/// is a human-readable duration (e.g. `"1 second"`, `"250 milliseconds"`).
fn http_retry_delay(body: &str) -> Option<std::time::Duration> {
    let body: serde_json::Value = serde_json::from_str(body).ok()?;
    parse_spelled_duration(body.get("retryInfo")?.as_str()?)
}

/// The `resources` of a JSON Ledger API error body: an array of `[type, name]`
/// pairs (e.g. `[["ErrorResource(CONTRACT_ID)", "00abc…"]]`). Entries that are
/// not a pair of strings are skipped rather than failing the whole read — a
/// diagnostic must not itself become an error.
fn http_resource_info(body: &str) -> Vec<ResourceInfo> {
    let Ok(body) = serde_json::from_str::<serde_json::Value>(body) else {
        return Vec::new();
    };
    let Some(resources) = body.get("resources").and_then(serde_json::Value::as_array) else {
        return Vec::new();
    };
    resources
        .iter()
        .filter_map(|entry| {
            let pair = entry.as_array()?;
            Some(ResourceInfo {
                resource_type: pair.first()?.as_str()?.to_string(),
                resource_name: pair.get(1)?.as_str()?.to_string(),
                owner: String::new(),
                description: String::new(),
            })
        })
        .collect()
}

/// The `grpcCodeValue` of a JSON Ledger API error body — the numeric gRPC code
/// the participant would have answered with over the other transport.
///
/// Only reported when the field is actually there: `tonic::Code::from` maps an
/// unknown number to `Unknown`, and manufacturing `Unknown` for a body that
/// never carried a code would claim a verdict the participant never gave.
fn http_grpc_code(body: &str) -> Option<tonic::Code> {
    let body: serde_json::Value = serde_json::from_str(body).ok()?;
    let value = body.get("grpcCodeValue")?.as_i64()?;
    Some(tonic::Code::from(i32::try_from(value).ok()?))
}

/// The `code` and `context` of a JSON Ledger API error body, as the
/// `ErrorInfo` the gRPC lane carries as a status detail.
///
/// `"NA"` is Canton's placeholder on a redacted (security-sensitive) error and
/// is treated as no id at all — reporting it would hand the caller a string
/// that looks like an error code and matches nothing.
///
/// A context value that is not a string keeps its JSON spelling rather than
/// being dropped: `metadata` is a string map, and losing a field silently is
/// worse than rendering it.
fn http_error_info(body: &str) -> Option<ErrorInfo> {
    let body: serde_json::Value = serde_json::from_str(body).ok()?;
    let reason = body.get("code")?.as_str()?;
    if reason.is_empty() || reason == "NA" {
        return None;
    }
    let metadata = body
        .get("context")
        .and_then(serde_json::Value::as_object)
        .map(|context| {
            context
                .iter()
                .map(|(key, value)| {
                    let value = match value.as_str() {
                        Some(text) => text.to_string(),
                        None => value.to_string(),
                    };
                    (key.clone(), value)
                })
                .collect()
        })
        .unwrap_or_default();
    Some(ErrorInfo {
        reason: reason.to_string(),
        // Canton sets no domain on either transport, and the JSON body has no
        // field for one. Empty here says the same thing gRPC says.
        domain: String::new(),
        metadata,
    })
}

/// The `correlationId` (or, failing that, `traceId`) of a JSON Ledger API
/// error body, when present.
fn http_correlation_id(body: &str) -> Option<String> {
    let body: serde_json::Value = serde_json::from_str(body).ok()?;
    ["correlationId", "traceId"]
        .iter()
        .find_map(|key| Some(body.get(key)?.as_str()?.to_string()))
}

/// Parse a `"<number> <unit>"` duration as the JSON API spells `retryInfo`
/// (Scala `Duration#toString`: `"1 second"`, `"5 seconds"`, …).
fn parse_spelled_duration(text: &str) -> Option<std::time::Duration> {
    let mut words = text.split_whitespace();
    let amount: f64 = words.next()?.parse().ok()?;
    let unit = words.next()?;
    if words.next().is_some() {
        return None;
    }
    let seconds = match unit.strip_suffix('s').unwrap_or(unit) {
        "day" => amount * 86_400.0,
        "hour" => amount * 3_600.0,
        "minute" => amount * 60.0,
        "second" => amount,
        "millisecond" => amount / 1e3,
        "microsecond" => amount / 1e6,
        "nanosecond" => amount / 1e9,
        _ => return None,
    };
    // `from_secs_f64` panics on a value a `Duration` cannot hold, and this
    // number arrives from the server: a `retryInfo` of `"1e300 seconds"` — or
    // an ordinary-looking `"1e15 days"` — would abort the caller's process
    // inside error classification, the one place that must stay infallible.
    // `try_from_secs_f64` rejects those the same way it rejects the NaN and
    // negative values this guarded against before.
    std::time::Duration::try_from_secs_f64(seconds).ok()
}

/// Canton's error categories — the coarse classification every Ledger API
/// error carries (`ErrorInfo.metadata["category"]`), which the [error-code
/// documentation] defines retryability on. `#[non_exhaustive]`: Canton may add
/// categories.
///
/// The variants are the categories of the Canton 3.x docs, by their stable
/// numeric ids (13 is a server-log-only warning that never reaches the API).
///
/// [error-code documentation]: https://docs.daml.com/canton/reference/error_codes.html
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorCategory {
    /// 1 — a service was momentarily unavailable; the request may or may not
    /// have been processed. Retry with backoff.
    TransientServerFailure,
    /// 2 — contention on shared resources (locks, rate limits, a locked
    /// contract). Retry with backoff.
    ContentionOnSharedResources,
    /// 3 — the request's deadline expired with its outcome unknown. Retry a
    /// bounded number of times, relying on command deduplication.
    DeadlineExceededRequestStateUnknown,
    /// 4 — a system-internal invariant was violated (implementation bug or
    /// data corruption). Not retriable; needs operator/vendor attention.
    SystemInternalAssumptionViolated,
    /// 5 — a potential attack or faulty peer was detected; details are
    /// deliberately withheld. Not retriable.
    SecurityAlert,
    /// 6 — missing or invalid authentication credentials. Not retriable
    /// until the credentials are fixed.
    AuthInterceptorInvalidAuthenticationCredentials,
    /// 7 — authenticated, but not permitted to perform the operation. Not
    /// retriable until permissions change.
    InsufficientPermission,
    /// 8 — the request is invalid regardless of system state (malformed
    /// arguments, size limits). Not retriable.
    InvalidIndependentOfSystemState,
    /// 9 — the current ledger state does not satisfy the request's
    /// preconditions (Daml interpretation failures land here). Not blindly
    /// retriable; needs an application-level strategy.
    InvalidGivenCurrentSystemStateOther,
    /// 10 — a referenced resource already exists (e.g. a duplicate command).
    /// Not retriable as-is.
    InvalidGivenCurrentSystemStateResourceExists,
    /// 11 — a referenced resource does not exist (contract, package, party).
    /// Not retriable as-is.
    InvalidGivenCurrentSystemStateResourceMissing,
    /// 12 — the request reads past the current ledger end. Retriable: the
    /// system may naturally progress to make it valid.
    InvalidGivenCurrentSystemStateSeekAfterEnd,
    /// 14 — the operation is not implemented / not enabled on this node. Not
    /// retriable.
    InternalUnsupportedOperation,
}

/// The std spelling of [`ErrorCategory::from_i32`], so the category can be
/// used with generic conversion bounds. The inherent method stays: it is the
/// documented name and the `Option` shape matches "unknown id" better than an
/// error type would.
impl TryFrom<i32> for ErrorCategory {
    type Error = i32;
    fn try_from(id: i32) -> std::result::Result<Self, i32> {
        Self::from_i32(id).ok_or(id)
    }
}

impl From<ErrorCategory> for i32 {
    fn from(category: ErrorCategory) -> i32 {
        category.as_i32()
    }
}

impl ErrorCategory {
    /// The category for Canton's numeric id, `None` when unrecognized.
    #[must_use]
    pub const fn from_i32(id: i32) -> Option<Self> {
        Some(match id {
            1 => Self::TransientServerFailure,
            2 => Self::ContentionOnSharedResources,
            3 => Self::DeadlineExceededRequestStateUnknown,
            4 => Self::SystemInternalAssumptionViolated,
            5 => Self::SecurityAlert,
            6 => Self::AuthInterceptorInvalidAuthenticationCredentials,
            7 => Self::InsufficientPermission,
            8 => Self::InvalidIndependentOfSystemState,
            9 => Self::InvalidGivenCurrentSystemStateOther,
            10 => Self::InvalidGivenCurrentSystemStateResourceExists,
            11 => Self::InvalidGivenCurrentSystemStateResourceMissing,
            12 => Self::InvalidGivenCurrentSystemStateSeekAfterEnd,
            14 => Self::InternalUnsupportedOperation,
            _ => return None,
        })
    }

    /// Canton's numeric id for this category.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::TransientServerFailure => 1,
            Self::ContentionOnSharedResources => 2,
            Self::DeadlineExceededRequestStateUnknown => 3,
            Self::SystemInternalAssumptionViolated => 4,
            Self::SecurityAlert => 5,
            Self::AuthInterceptorInvalidAuthenticationCredentials => 6,
            Self::InsufficientPermission => 7,
            Self::InvalidIndependentOfSystemState => 8,
            Self::InvalidGivenCurrentSystemStateOther => 9,
            Self::InvalidGivenCurrentSystemStateResourceExists => 10,
            Self::InvalidGivenCurrentSystemStateResourceMissing => 11,
            Self::InvalidGivenCurrentSystemStateSeekAfterEnd => 12,
            Self::InternalUnsupportedOperation => 14,
        }
    }

    /// Whether the error-code documentation classifies this category as
    /// retryable (transient failures, contention, unknown-outcome deadlines,
    /// and reads past the ledger end).
    #[must_use]
    pub const fn is_retriable(self) -> bool {
        matches!(
            self,
            Self::TransientServerFailure
                | Self::ContentionOnSharedResources
                | Self::DeadlineExceededRequestStateUnknown
                | Self::InvalidGivenCurrentSystemStateSeekAfterEnd
        )
    }
}

/// Structured `google.rpc.ErrorInfo` details from a gRPC status: the machine-
/// readable `reason`, its `domain`, and error `metadata`. `#[non_exhaustive]`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ErrorInfo {
    /// Machine-readable error reason (e.g. a Canton/Daml error code).
    pub reason: String,
    /// The logical grouping the `reason` belongs to.
    pub domain: String,
    /// Additional structured context for the error.
    pub metadata: std::collections::HashMap<String, String>,
}

/// A resource a failure is about, from a `google.rpc.ResourceInfo` detail on a
/// gRPC status or an entry of a JSON API error body's `resources`.
/// `#[non_exhaustive]`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ResourceInfo {
    /// What kind of thing it is, as Canton names it (e.g.
    /// `ErrorResource(CONTRACT_ID)`).
    pub resource_type: String,
    /// The identifier itself — a contract id, package name, party, …
    pub resource_name: String,
    /// The owner, when the server reports one. Empty on the JSON transport,
    /// whose `resources` entries carry only the type and the name.
    pub owner: String,
    /// A human-readable note, when the server reports one. Empty on the JSON
    /// transport for the same reason.
    pub description: String,
}

/// Most transport errors keep the sentence that matters in their source chain
/// and print only a generic outer line: `reqwest` says `error sending request
/// for url (…)` while `invalid peer certificate: UnknownIssuer` sits one level
/// down, and `tokio_postgres` says `db error` over `FATAL: password
/// authentication failed`. Reporting only the outer message hides the one
/// thing an operator needs.
///
/// Lives here because four crates were each losing it separately, and three of
/// them had grown their own private copy of this function before anyone noticed
/// the fourth still had none.
#[must_use]
pub fn chain(error: &dyn std::error::Error) -> String {
    let mut message = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}

impl From<tonic::Status> for Error {
    fn from(status: tonic::Status) -> Self {
        Error::Status(Box::new(status))
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(err: tonic::transport::Error) -> Self {
        Error::Transport(Box::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(Box::new(err))
    }
}

/// SDK-wide result alias. Re-exported by the facade as `canton::Result`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn error_info_is_extracted_from_a_status_and_absent_otherwise() {
        use tonic_types::{ErrorDetails, StatusExt as _};

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("resource".to_string(), "contract-1".to_string());
        let details = ErrorDetails::with_error_info("DUPLICATE_COMMAND", "canton", metadata);
        let status = tonic::Status::with_error_details(tonic::Code::AlreadyExists, "dup", details);

        let info = Error::from(status)
            .error_info()
            .expect("error info present");
        assert_eq!(info.reason, "DUPLICATE_COMMAND");
        assert_eq!(info.domain, "canton");
        assert_eq!(
            info.metadata.get("resource").map(String::as_str),
            Some("contract-1")
        );

        // A status without ErrorInfo, and a non-status error, yield None.
        assert!(
            Error::from(tonic::Status::not_found("x"))
                .error_info()
                .is_none()
        );
        assert!(Error::Timeout.error_info().is_none());
    }

    /// A synthetic Canton-style status: `ErrorInfo` with a `category` metadata
    /// entry, `RequestInfo` with the correlation id, and — when `delay` is set —
    /// a `RetryInfo` recommendation.
    fn canton_status(
        code: tonic::Code,
        category: i32,
        delay: Option<std::time::Duration>,
    ) -> tonic::Status {
        use tonic_types::{ErrorDetails, StatusExt as _};

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("category".to_string(), category.to_string());
        let mut details = ErrorDetails::with_error_info("SOME_ERROR_CODE", "participant", metadata);
        details.set_request_info("corr-1234", "");
        if let Some(delay) = delay {
            details.set_retry_info(Some(delay));
        }
        tonic::Status::with_error_details(code, "boom", details)
    }

    #[test]
    fn the_canton_category_decides_retryability_over_the_grpc_code() {
        use std::time::Duration;

        // ABORTED is transient by code — but category 10 (resource exists,
        // e.g. a duplicate change id) says no. The category wins.
        let err = Error::from(canton_status(tonic::Code::Aborted, 10, None));
        assert_eq!(
            err.category(),
            Some(ErrorCategory::InvalidGivenCurrentSystemStateResourceExists)
        );
        assert!(!err.is_retriable());

        // OUT_OF_RANGE is not transient by code — but category 12 (seek past
        // the ledger end) says retry. The category wins again.
        let err = Error::from(canton_status(
            tonic::Code::OutOfRange,
            12,
            Some(Duration::from_secs(1)),
        ));
        assert_eq!(
            err.category(),
            Some(ErrorCategory::InvalidGivenCurrentSystemStateSeekAfterEnd)
        );
        assert!(err.is_retriable());
        assert_eq!(err.retry_delay(), Some(Duration::from_secs(1)));
    }

    #[test]
    fn correlation_id_and_retry_delay_are_extracted() {
        use std::time::Duration;

        let err = Error::from(canton_status(
            tonic::Code::Unavailable,
            1,
            Some(Duration::from_millis(250)),
        ));
        assert_eq!(err.category(), Some(ErrorCategory::TransientServerFailure));
        assert_eq!(err.correlation_id().as_deref(), Some("corr-1234"));
        assert_eq!(err.retry_delay(), Some(Duration::from_millis(250)));

        // Plain statuses and non-status errors carry none of it.
        let plain = Error::from(tonic::Status::unavailable("x"));
        assert_eq!(plain.category(), None);
        assert_eq!(plain.correlation_id(), None);
        assert_eq!(plain.retry_delay(), None);
        assert_eq!(Error::Timeout.category(), None);
    }

    #[test]
    fn statuses_without_a_category_fall_back_to_code_classification() {
        use tonic_types::{ErrorDetails, StatusExt as _};

        // No details at all: the transient codes still classify.
        assert!(Error::from(tonic::Status::unavailable("x")).is_retriable());
        assert!(!Error::from(tonic::Status::not_found("x")).is_retriable());

        // An unrecognized category id is ignored (forward compatibility), and
        // the code fallback applies.
        let err = Error::from(canton_status(tonic::Code::Unavailable, 99, None));
        assert_eq!(err.category(), None);
        assert!(err.is_retriable());

        // A bare RetryInfo (no category) is an explicit "retry me", even on a
        // code the fallback would refuse.
        let mut details = ErrorDetails::new();
        details.set_retry_info(Some(std::time::Duration::from_secs(2)));
        let status =
            tonic::Status::with_error_details(tonic::Code::FailedPrecondition, "wait", details);
        assert!(Error::from(status).is_retriable());
    }

    #[test]
    fn json_api_error_bodies_classify_by_category() {
        // A real body captured from a LocalNet participant (JSON Ledger API):
        // category 12 (seek after end) is retryable although the HTTP status
        // (400) is not in the transient set — the category verdict wins.
        let body = r#"{
            "code": "OFFSET_AFTER_LEDGER_END",
            "cause": "Begin offset (999999999) is after ledger end (23577)",
            "correlationId": null,
            "traceId": "36a33702b2fa7908a7349be166ccfa38",
            "context": {"participant": "'app-provider'", "category": "12"},
            "resources": [],
            "errorCategory": 12,
            "grpcCodeValue": 11,
            "retryInfo": "1 second",
            "definiteAnswer": null
        }"#;
        let err = Error::Http {
            status: 400,
            body: body.to_string(),
        };
        assert_eq!(
            err.category(),
            Some(ErrorCategory::InvalidGivenCurrentSystemStateSeekAfterEnd)
        );
        assert!(err.is_retriable());
        assert_eq!(err.retry_delay(), Some(std::time::Duration::from_secs(1)));
        // correlationId is null — falls back to traceId.
        assert_eq!(
            err.correlation_id().as_deref(),
            Some("36a33702b2fa7908a7349be166ccfa38")
        );

        // The same failure over gRPC yields OutOfRange and an ErrorInfo naming
        // OFFSET_AFTER_LEDGER_END. The JSON body spells both — `grpcCodeValue`
        // and `code`/`context` — so both accessors must answer here too, or a
        // caller on this transport is left with the string matching
        // `error_info` exists to replace.
        assert_eq!(err.code(), Some(tonic::Code::OutOfRange));
        let info = err.error_info().expect("the body names the error");
        assert_eq!(info.reason, "OFFSET_AFTER_LEDGER_END");
        assert_eq!(
            info.metadata.get("category").map(String::as_str),
            Some("12")
        );
        assert_eq!(
            info.metadata.get("participant").map(String::as_str),
            Some("'app-provider'")
        );

        // A non-retryable category on a retryable-looking HTTP status: the
        // category still wins (e.g. a 503 whose body says "invalid argument").
        let err = Error::Http {
            status: 503,
            body: r#"{"errorCategory": 8}"#.to_string(),
        };
        assert_eq!(
            err.category(),
            Some(ErrorCategory::InvalidIndependentOfSystemState)
        );
        assert!(!err.is_retriable());
    }

    #[test]
    fn non_json_http_bodies_fall_back_to_status_code_classification() {
        let retriable = Error::Http {
            status: 503,
            body: "<html>Service Unavailable</html>".to_string(),
        };
        assert!(retriable.is_retriable());
        assert_eq!(retriable.category(), None);
        assert_eq!(retriable.retry_delay(), None);

        let terminal = Error::Http {
            status: 404,
            body: String::new(),
        };
        assert!(!terminal.is_retriable());
        assert_eq!(terminal.correlation_id(), None);
    }

    #[test]
    fn spelled_durations_parse_and_garbage_is_refused() {
        use std::time::Duration;
        for (text, expected) in [
            ("1 second", Duration::from_secs(1)),
            ("5 seconds", Duration::from_secs(5)),
            ("250 milliseconds", Duration::from_millis(250)),
            ("2 minutes", Duration::from_secs(120)),
            ("1 hour", Duration::from_secs(3600)),
            ("0.5 seconds", Duration::from_millis(500)),
            // The rarer units and the day arm: mutation testing showed these
            // conversion factors were never exercised, so a wrong multiplier
            // in day/microsecond/nanosecond would have passed unnoticed.
            ("2 days", Duration::from_secs(2 * 86_400)),
            ("1 day", Duration::from_secs(86_400)),
            ("500 microseconds", Duration::from_micros(500)),
            ("250 nanoseconds", Duration::from_nanos(250)),
        ] {
            assert_eq!(parse_spelled_duration(text), Some(expected), "{text}");
        }
        // A server-supplied number too large for a `Duration` is refused, not
        // panicked on: this runs while an error is being classified, and the
        // value is whatever the participant put in the body.
        for bad in [
            "",
            "soon",
            "1",
            "1 fortnight",
            "-1 second",
            "1 second ago",
            "1e300 seconds",
            "1e300 days",
            "NaN seconds",
            "inf seconds",
        ] {
            assert_eq!(parse_spelled_duration(bad), None, "{bad}");
        }
    }

    #[test]
    fn category_ids_round_trip_and_follow_the_docs_retryability() {
        for id in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14] {
            let category = ErrorCategory::from_i32(id).expect("known id");
            assert_eq!(category.as_i32(), id);
            // Per the error-code docs: 1, 2, 3 and 12 are the retryable ones.
            assert_eq!(category.is_retriable(), matches!(id, 1 | 2 | 3 | 12));
        }
        assert_eq!(ErrorCategory::from_i32(0), None);
        // 13 (BackgroundProcessDegradationWarning) never reaches the API.
        assert_eq!(ErrorCategory::from_i32(13), None);
        assert_eq!(ErrorCategory::from_i32(15), None);
    }

    /// Canton strips the detail from security-sensitive failures — auth,
    /// permission, internal — and tells the caller to ask the operator. Every
    /// accessor must degrade to "nothing" rather than mislead, and the one
    /// thing that survives must keep surviving: without the correlation id
    /// there is no way to ask.
    ///
    /// Both fixtures are the real shapes, taken from a live 3.5.7 participant
    /// answering with an invalid token.
    #[test]
    fn a_redacted_status_yields_nothing_except_the_correlation_id() {
        use tonic_types::{ErrorDetails, StatusExt as _};

        // gRPC: no ErrorInfo, no RetryInfo, no ResourceInfo — only RequestInfo.
        let mut details = ErrorDetails::new();
        details.set_request_info("93199811c5b2090c51cf45fe8c88060c", "");
        let status = tonic::Status::with_error_details(
            tonic::Code::Unauthenticated,
            "An error occurred. Please contact the operator and inquire about the request \
             93199811c5b2090c51cf45fe8c88060c",
            details,
        );
        let err = Error::from(status);

        assert_eq!(err.category(), None, "a redacted status has no category");
        assert_eq!(err.error_info(), None);
        assert!(err.resource_info().is_empty());
        assert_eq!(err.retry_delay(), None);
        assert_eq!(
            err.correlation_id().as_deref(),
            Some("93199811c5b2090c51cf45fe8c88060c"),
            "the correlation id is the only actionable thing left"
        );
        // Classification falls back to the code, which is the right answer:
        // bad credentials will not become good on a retry.
        assert!(!err.is_retriable());

        // JSON: the category is reported as -1 rather than omitted, which must
        // not be mistaken for a real category.
        let body = r#"{"code":"NA","cause":"An error occurred. Please contact the operator",
            "errorCategory":-1,"retryInfo":null,"resources":[],
            "correlationId":"41f217564e4e76f6cbc853a94a82fa80",
            "traceId":"41f217564e4e76f6cbc853a94a82fa80"}"#;
        let err = Error::Http {
            status: 401,
            body: body.to_string(),
        };

        assert_eq!(err.category(), None, "-1 is not a category");
        assert!(err.resource_info().is_empty());
        assert_eq!(err.retry_delay(), None);
        // `"NA"` is Canton saying it withheld the id, not an id spelled "NA".
        assert_eq!(err.error_info(), None, "\"NA\" is not an error id");
        assert_eq!(
            err.correlation_id().as_deref(),
            Some("41f217564e4e76f6cbc853a94a82fa80")
        );
        assert!(!err.is_retriable(), "401 is not transient");
    }

    #[test]
    fn resource_info_names_what_the_error_is_about() {
        use tonic_types::{ErrorDetails, StatusExt as _};

        // gRPC: one `google.rpc.ResourceInfo` detail is all tonic_types models.
        let mut details = ErrorDetails::new();
        details.set_resource_info("ErrorResource(CONTRACT_ID)", "00abc", "alice", "not found");
        let status = tonic::Status::with_error_details(tonic::Code::NotFound, "gone", details);
        let found = Error::from(status).resource_info();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].resource_type, "ErrorResource(CONTRACT_ID)");
        assert_eq!(found[0].resource_name, "00abc");
        assert_eq!(found[0].owner, "alice");

        // A status with no such detail yields nothing, not a default-filled entry.
        assert!(
            Error::from(tonic::Status::not_found("x"))
                .resource_info()
                .is_empty()
        );
        assert!(Error::Timeout.resource_info().is_empty());
    }

    #[test]
    fn resource_info_reads_the_json_apis_resources_array() {
        // The body shape is verbatim from a live Canton 3.5.7 participant
        // answering an exercise on a contract that does not exist.
        let body = r#"{"code":"CONTRACT_NOT_FOUND","cause":"…","errorCategory":11,
            "resources":[["ErrorResource(CONTRACT_ID)","00ababab"]],"retryInfo":null}"#;
        let err = Error::Http {
            status: 404,
            body: body.to_string(),
        };
        let found = err.resource_info();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].resource_type, "ErrorResource(CONTRACT_ID)");
        assert_eq!(found[0].resource_name, "00ababab");
        // The JSON pairs carry no owner or description; they are empty, not absent.
        assert!(found[0].owner.is_empty());

        // Several resources — contention names more than one, which is why this
        // returns a Vec and not an Option.
        let many = Error::Http {
            status: 409,
            body: r#"{"resources":[["A","1"],["B","2"]]}"#.to_string(),
        };
        assert_eq!(many.resource_info().len(), 2);

        // A malformed entry is skipped, and an empty/absent array is not an error:
        // a diagnostic accessor must never itself fail.
        let ragged = Error::Http {
            status: 500,
            body: r#"{"resources":[["A"],["B","2"],42,null]}"#.to_string(),
        };
        assert_eq!(ragged.resource_info().len(), 1);
        for body in [r#"{"resources":[]}"#, "{}", "not json at all", ""] {
            let err = Error::Http {
                status: 500,
                body: body.to_string(),
            };
            assert!(err.resource_info().is_empty(), "{body}");
        }
    }

    #[test]
    fn transient_conditions_are_retriable() {
        assert!(Error::Timeout.is_retriable());
        assert!(Error::Connection("reset".to_string()).is_retriable());
        assert!(Error::from(tonic::Status::unavailable("x")).is_retriable());
        assert!(Error::from(tonic::Status::deadline_exceeded("x")).is_retriable());
        assert!(Error::from(tonic::Status::resource_exhausted("x")).is_retriable());
        assert!(Error::from(tonic::Status::aborted("x")).is_retriable());
    }

    #[test]
    fn a_connection_that_dies_mid_call_is_retriable_but_a_server_unknown_is_not() {
        use std::io;

        // What tonic hands back when a stream or unary connection dies in
        // flight: code Unknown, built *from* the transport error, so the source
        // chain is populated. This is the ordinary dropped connection, and the
        // resumable stream and the retry pipeline both depend on it being
        // retriable — before this, they gave up on the first blip.
        let dropped = tonic::Status::from_error(Box::new(io::Error::new(
            io::ErrorKind::ConnectionReset,
            "h2 protocol error: error reading a body from connection",
        )));
        assert_eq!(
            dropped.code(),
            tonic::Code::Unknown,
            "from_error yields Unknown"
        );
        assert!(Error::from(dropped).is_retriable());

        // A genuine `Unknown` the participant returned is decoded from response
        // trailers and carries no local source. That is the server's verdict,
        // and retrying it would just re-ask the same losing question.
        let server_unknown = tonic::Status::new(tonic::Code::Unknown, "business rule failed");
        assert!(!Error::from(server_unknown).is_retriable());

        // `Internal` follows the same rule: transport-built is retriable, a
        // bare server Internal is not.
        assert!(
            Error::from(tonic::Status::from_error(Box::new(io::Error::other(
                "broken pipe"
            ))))
            .is_retriable()
        );
        assert!(!Error::from(tonic::Status::internal("server bug")).is_retriable());
    }

    #[test]
    fn transient_http_codes_are_retriable_but_client_codes_are_not() {
        // The whole 5xx range is transient (per the doc), plus 408/429 — not just
        // a hand-picked subset. 501/509/511/520 were previously missed.
        for status in [
            408, 429, 500, 501, 502, 503, 504, 507, 509, 511, 520, 527, 599,
        ] {
            assert!(
                Error::Http {
                    status,
                    body: String::new()
                }
                .is_retriable(),
                "http {status} should be retriable"
            );
        }
        // 4xx (incl. the JSON API's 413 too-large) stay non-retriable.
        for status in [400, 401, 403, 404, 409, 413, 422] {
            assert!(
                !Error::Http {
                    status,
                    body: String::new()
                }
                .is_retriable(),
                "http {status} should not be retriable"
            );
        }
    }

    #[test]
    fn definite_failures_are_not_retriable() {
        assert!(!Error::from(tonic::Status::not_found("x")).is_retriable());
        assert!(!Error::from(tonic::Status::already_exists("dup")).is_retriable());
        assert!(!Error::from(tonic::Status::invalid_argument("x")).is_retriable());
        assert!(!Error::InvalidRequest("x".to_string()).is_retriable());
        assert!(!Error::Auth("x".to_string()).is_retriable());
        assert!(
            !Error::CommandRejected {
                code: "GrpcStatus".to_string(),
                message: "boom".to_string()
            }
            .is_retriable()
        );
        assert!(!Error::UnexpectedResponse("x".to_string()).is_retriable());
    }

    #[test]
    fn code_is_exposed_only_for_status_errors() {
        assert_eq!(
            Error::from(tonic::Status::not_found("x")).code(),
            Some(tonic::Code::NotFound)
        );
        assert_eq!(Error::Timeout.code(), None);
        assert_eq!(Error::Connection("x".to_string()).code(), None);
        assert_eq!(
            Error::Http {
                status: 503,
                body: String::new()
            }
            .code(),
            None
        );
    }

    #[test]
    fn display_messages_are_lowercase_and_informative() {
        assert_eq!(Error::Timeout.to_string(), "operation timed out");
        assert_eq!(
            Error::InvalidRequest("bad uri".to_string()).to_string(),
            "invalid request: bad uri"
        );
        assert_eq!(
            Error::Auth("token expired".to_string()).to_string(),
            "authentication failed: token expired"
        );
        assert_eq!(
            Error::Http {
                status: 503,
                body: "down".to_string()
            }
            .to_string(),
            "http 503: down"
        );
        assert_eq!(
            Error::CommandRejected {
                code: "INVALID_ARGUMENT".to_string(),
                message: "nope".to_string()
            }
            .to_string(),
            "command rejected (INVALID_ARGUMENT): nope"
        );
    }
}
