//! Shared client connection configuration (the Option-B connection kernel).
//!
//! Endpoint, authentication, TLS, and retry live here so every gRPC client in
//! the SDK (`canton-ledger`, `canton-admin`) builds its channel the same way.
//! Authentication is decoupled from any concrete provider via the
//! [`TokenSource`] trait — `canton-auth`'s token provider implements it, which
//! keeps `canton-core` free of a `canton-auth` dependency (that would be a
//! cycle) while letting [`Config`] carry any token source.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};

use crate::retry::RetryConfig;
use crate::{Error, Result};

/// A source of bearer tokens for request authentication.
///
/// Implemented by `canton_auth::TokenProvider` (OIDC client-credentials with
/// caching + refresh). Object-safe by design: [`Auth::Dynamic`] stores it as an
/// `Arc<dyn TokenSource>`.
pub trait TokenSource: Send + Sync + fmt::Debug {
    /// Resolve the current bearer token (fetching/refreshing as needed), or
    /// `None` for an unauthenticated call.
    fn fetch_bearer(&self) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send + '_>>;
}

/// How the client authenticates each request.
///
/// `#[non_exhaustive]` so new auth modes can be added without a breaking change;
/// construct via [`Config::with_token`] / [`Config::with_oidc`] (or match with a
/// wildcard arm).
#[derive(Clone)]
#[non_exhaustive]
pub enum Auth {
    /// No authentication (unauthenticated endpoints, or shared-secret off).
    None,
    /// A fixed bearer token supplied by the caller.
    Static(String),
    /// A dynamic token source (e.g. OIDC client-credentials with auto-refresh).
    Dynamic(Arc<dyn TokenSource>),
}

impl Auth {
    /// Resolve the current bearer token, if any.
    ///
    /// # Errors
    /// Propagates any error from the underlying [`TokenSource`].
    pub async fn bearer(&self) -> Result<Option<String>> {
        match self {
            Auth::None => Ok(None),
            Auth::Static(token) => Ok(Some(token.clone())),
            Auth::Dynamic(source) => source.fetch_bearer().await,
        }
    }
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Auth::None => f.write_str("None"),
            Auth::Static(_) => f.write_str("Static(<redacted>)"),
            Auth::Dynamic(source) => write!(f, "Dynamic({source:?})"),
        }
    }
}

/// TLS settings for the gRPC channel.
///
/// An empty `TlsConfig` (from [`TlsConfig::new`]) enables server-side TLS using
/// the platform's native root certificates. Add a custom CA for private/self-
/// signed servers, a domain-name override for SNI/verification, and a client
/// identity for mutual TLS. `#[non_exhaustive]`.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TlsConfig {
    /// Custom CA certificate chain (PEM). When set, replaces the native roots
    /// (use for self-signed / private CAs).
    pub ca_certificate_pem: Option<Vec<u8>>,
    /// Domain name to verify the server certificate against (SNI). Defaults to
    /// the endpoint host.
    pub domain_name: Option<String>,
    /// Client identity `(certificate_pem, private_key_pem)` for mutual TLS.
    pub client_identity_pem: Option<(Vec<u8>, Vec<u8>)>,
}

impl TlsConfig {
    /// Server-side TLS using the platform's native root certificates.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Trust this PEM CA certificate (chain) instead of the native roots.
    #[must_use]
    pub fn with_ca_certificate(mut self, ca_pem: impl Into<Vec<u8>>) -> Self {
        self.ca_certificate_pem = Some(ca_pem.into());
        self
    }

    /// Override the domain name the server certificate is verified against.
    #[must_use]
    pub fn with_domain_name(mut self, domain: impl Into<String>) -> Self {
        self.domain_name = Some(domain.into());
        self
    }

    /// Present a client identity (mutual TLS): `(certificate_pem, key_pem)`.
    #[must_use]
    pub fn with_client_identity(
        mut self,
        certificate_pem: impl Into<Vec<u8>>,
        private_key_pem: impl Into<Vec<u8>>,
    ) -> Self {
        self.client_identity_pem = Some((certificate_pem.into(), private_key_pem.into()));
        self
    }
}

/// Build a `ClientTlsConfig` from a [`TlsConfig`] (or native roots when TLS is
/// implicit for an `https` endpoint).
fn build_tls(tls: Option<&TlsConfig>) -> ClientTlsConfig {
    let mut config = ClientTlsConfig::new();
    match tls.and_then(|t| t.ca_certificate_pem.as_ref()) {
        Some(ca) => config = config.ca_certificate(Certificate::from_pem(ca.clone())),
        None => config = config.with_native_roots(),
    }
    if let Some(domain) = tls.and_then(|t| t.domain_name.as_ref()) {
        config = config.domain_name(domain.clone());
    }
    if let Some((cert, key)) = tls.and_then(|t| t.client_identity_pem.as_ref()) {
        config = config.identity(Identity::from_pem(cert.clone(), key.clone()));
    }
    config
}

/// Configuration for an SDK gRPC client (shared by `canton-ledger` and
/// `canton-admin`).
#[derive(Clone, Debug)]
pub struct Config {
    endpoint: String,
    auth: Auth,
    retry: Option<RetryConfig>,
    tls: Option<TlsConfig>,
    timeout: Option<Duration>,
}

impl Config {
    /// Create a configuration targeting `endpoint`, with no authentication and
    /// no retrying.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            auth: Auth::None,
            retry: None,
            tls: None,
            timeout: None,
        }
    }

    /// Authenticate with a fixed bearer token.
    #[must_use]
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Auth::Static(token.into());
        self
    }

    /// Authenticate with a dynamic token source (e.g. an OIDC provider with
    /// client-credentials and auto-refresh).
    #[must_use]
    pub fn with_oidc<T: TokenSource + 'static>(mut self, provider: T) -> Self {
        self.auth = Auth::Dynamic(Arc::new(provider));
        self
    }

    /// Enable retrying of unary calls on retriable errors, per `retry`.
    #[must_use]
    pub fn with_retry(mut self, retry: RetryConfig) -> Self {
        self.retry = Some(retry);
        self
    }

    /// Use TLS for the gRPC channel (server-side and, if configured, mutual).
    #[must_use]
    pub fn with_tls(mut self, tls: TlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    /// Set the per-request timeout on the gRPC channel (default 30s). Long
    /// `submitAndWait` calls under load may need a higher bound.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// The gRPC endpoint of the target service, e.g. `http://localhost:3901`.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// The configured authentication mode.
    #[must_use]
    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    /// The configured retry policy, if any.
    #[must_use]
    pub fn retry(&self) -> Option<&RetryConfig> {
        self.retry.as_ref()
    }

    /// Build a lazily-connected gRPC [`Channel`] for this configuration.
    ///
    /// Returns immediately; the TCP/TLS handshake happens on the first RPC. TLS
    /// is applied when [`Config::with_tls`] was set or the endpoint is `https`;
    /// a `with_tls` endpoint given as `http://` is normalised to `https://` so
    /// TLS is never silently downgraded (see [`resolve_endpoint`]).
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] if the endpoint URI or the TLS
    /// configuration is invalid.
    pub fn connect_channel(&self) -> Result<Channel> {
        let (uri, want_tls) = resolve_endpoint(&self.endpoint, self.tls.is_some());
        let mut endpoint = Endpoint::from_shared(uri.clone())
            .map_err(|e| Error::InvalidRequest(format!("invalid endpoint uri {uri:?}: {e}")))?
            .timeout(self.timeout.unwrap_or(Duration::from_secs(30)))
            .connect_timeout(Duration::from_secs(10))
            .http2_keep_alive_interval(Duration::from_secs(30))
            .keep_alive_timeout(Duration::from_secs(20))
            .keep_alive_while_idle(true)
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .tcp_nodelay(true);

        if want_tls {
            endpoint = endpoint
                .tls_config(build_tls(self.tls.as_ref()))
                .map_err(|e| Error::InvalidRequest(format!("invalid TLS config: {e}")))?;
        }

        Ok(endpoint.connect_lazy())
    }
}

/// Resolve the effective endpoint URI and whether TLS should be attached.
///
/// `tonic` gates the TLS handshake on the URI **scheme**, not on the presence of
/// a `tls_config` — so attaching TLS to an `http://` endpoint is silently ignored
/// and the connection runs in plaintext (no encryption, no server-cert
/// verification, no client cert). When TLS is configured — or the endpoint is
/// already `https` — the scheme is normalised to `https` so TLS is actually
/// applied. Scheme detection is case-insensitive (tonic lowercases the parsed
/// scheme, so `HTTPS://…` must be treated as `https`).
fn resolve_endpoint(endpoint: &str, tls_configured: bool) -> (String, bool) {
    let is_https = endpoint
        .get(..8)
        .is_some_and(|s| s.eq_ignore_ascii_case("https://"));
    let is_http = endpoint
        .get(..7)
        .is_some_and(|s| s.eq_ignore_ascii_case("http://"));
    let want_tls = tls_configured || is_https;
    if want_tls && is_http {
        (format!("https://{}", &endpoint[7..]), true)
    } else {
        (endpoint.to_string(), want_tls)
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::resolve_endpoint;

    #[test]
    fn with_tls_on_http_endpoint_is_upgraded_to_https() {
        // The security bug: `with_tls` on an http:// endpoint would otherwise
        // connect in plaintext. It must become https so TLS is applied.
        let (uri, tls) = resolve_endpoint("http://host:5001", true);
        assert_eq!(uri, "https://host:5001");
        assert_eq!(tls, true);
    }

    #[test]
    fn https_scheme_detection_is_case_insensitive() {
        // `HTTPS://` (no explicit with_tls) must still get TLS — tonic sees the
        // lowercased scheme and would otherwise error on an https URI with no TLS.
        let (uri, tls) = resolve_endpoint("HTTPS://host:443", false);
        assert_eq!(uri, "HTTPS://host:443");
        assert_eq!(tls, true);
    }

    #[test]
    fn plain_http_without_tls_stays_plaintext() {
        let (uri, tls) = resolve_endpoint("http://host:3901", false);
        assert_eq!(uri, "http://host:3901");
        assert_eq!(tls, false);
    }

    #[test]
    fn https_without_explicit_tls_wants_tls() {
        let (uri, tls) = resolve_endpoint("https://host:443", false);
        assert_eq!(uri, "https://host:443");
        assert_eq!(tls, true);
    }

    #[test]
    fn uppercase_http_with_tls_is_upgraded() {
        let (uri, tls) = resolve_endpoint("HTTP://host:5001", true);
        assert_eq!(uri, "https://host:5001");
        assert_eq!(tls, true);
    }
}
