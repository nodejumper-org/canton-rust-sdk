//! Request authentication for gRPC channels.
//!
//! A tonic interceptor that injects a bearer token into request metadata, plus
//! [`intercepted`] which wraps a [`Channel`] with a freshly-resolved token for
//! a single call. The token itself comes from [`crate::Auth`] (static or a
//! dynamic [`crate::TokenSource`]).

use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::Result;
use crate::config::Auth;

/// A gRPC channel wrapped with a bearer-token interceptor.
pub type Intercepted = InterceptedService<Channel, AuthInterceptor>;

/// Injects an `authorization: Bearer <token>` header when a token is present.
#[derive(Clone, Debug, Default)]
pub struct AuthInterceptor {
    token: Option<String>,
}

impl AuthInterceptor {
    /// Create an interceptor that injects `token` if `Some`, or is a no-op if `None`.
    #[must_use]
    pub fn new(token: Option<String>) -> Self {
        Self { token }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(token) = &self.token {
            let value: MetadataValue<_> = format!("Bearer {token}")
                .parse()
                .map_err(|_| Status::unauthenticated("invalid bearer token"))?;
            request.metadata_mut().insert("authorization", value);
        }
        // Propagate W3C trace context (no-op without an active OTel context).
        #[cfg(feature = "otel")]
        crate::telemetry::otel::inject_trace_context_metadata(request.metadata_mut());
        Ok(request)
    }
}

/// Wrap `channel` with a fresh bearer token resolved from `auth`, ready to back
/// a generated gRPC client for one call.
///
/// # Errors
/// Propagates token-resolution errors from the [`Auth`] source.
pub async fn intercepted(channel: &Channel, auth: &Auth) -> Result<Intercepted> {
    let token = auth.bearer().await?;
    Ok(InterceptedService::new(
        channel.clone(),
        AuthInterceptor::new(token),
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn injects_bearer_header_when_token_present() {
        let mut interceptor = AuthInterceptor::new(Some("tok-123".to_string()));
        let req = interceptor.call(Request::new(())).unwrap();
        let value = req.metadata().get("authorization").unwrap();
        assert_eq!(value.to_str().unwrap(), "Bearer tok-123");
    }

    #[test]
    fn no_header_when_token_absent() {
        let mut interceptor = AuthInterceptor::new(None);
        let req = interceptor.call(Request::new(())).unwrap();
        assert!(req.metadata().get("authorization").is_none());
    }

    #[test]
    fn rejects_a_token_with_illegal_header_bytes() {
        let mut interceptor = AuthInterceptor::new(Some("bad\ntoken".to_string()));
        let result = interceptor.call(Request::new(()));
        assert!(result.is_err(), "a token with a newline must be rejected");
    }
}
