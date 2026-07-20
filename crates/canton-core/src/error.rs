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
    /// reasons (as opposed to a transport failure). Not retriable.
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
}

impl Error {
    /// The gRPC status code, if this error originates from a gRPC status.
    #[must_use]
    pub fn code(&self) -> Option<tonic::Code> {
        match self {
            Error::Status(status) => Some(status.code()),
            _ => None,
        }
    }

    /// Whether retrying the operation may succeed.
    ///
    /// Transient conditions are retriable: timeouts, transport/connection
    /// failures, the transient gRPC codes (`Unavailable`, `DeadlineExceeded`,
    /// `ResourceExhausted`, `Aborted`), and transient HTTP status codes
    /// (408, 429, 5xx). Everything else — invalid input, auth rejection,
    /// command rejection, `NotFound`/`AlreadyExists`, deserialization — is not.
    #[must_use]
    pub fn is_retriable(&self) -> bool {
        use tonic::Code::{Aborted, DeadlineExceeded, ResourceExhausted, Unavailable};
        match self {
            Error::Timeout | Error::Transport(_) | Error::Connection(_) => true,
            Error::Status(status) => matches!(
                status.code(),
                Unavailable | DeadlineExceeded | ResourceExhausted | Aborted
            ),
            Error::Http { status, .. } => matches!(status, 408 | 429 | 500 | 502 | 503 | 504),
            _ => false,
        }
    }

    /// The structured `google.rpc.ErrorInfo` carried by a gRPC status, when
    /// present. Canton populates this with the machine-readable error `reason`
    /// (e.g. `DUPLICATE_COMMAND`) plus context `metadata` — prefer it over
    /// string-matching [`Display`](std::fmt::Display) output. Returns `None` for
    /// non-status errors or statuses without an `ErrorInfo` detail.
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
            _ => None,
        }
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
    fn transient_http_codes_are_retriable_but_client_codes_are_not() {
        for status in [408, 429, 500, 502, 503, 504] {
            assert!(
                Error::Http {
                    status,
                    body: String::new()
                }
                .is_retriable(),
                "http {status} should be retriable"
            );
        }
        for status in [400, 401, 403, 404, 409] {
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
