//! In-process tests that need a controllable server but no live node.
//!
//! Uses the dev-only `canton-proto/server` feature for a gRPC mock, and a tiny
//! raw-TCP HTTP server for the OIDC token endpoint.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use canton_auth::{OidcConfig, TokenProvider};
use canton_ledger::{CantonClient, Config, RetryConfig};
use canton_proto::com::daml::ledger::api::v2 as pb;
use pb::update_service_server::{UpdateService, UpdateServiceServer};
use pb::version_service_server::{VersionService, VersionServiceServer};
use std::pin::Pin;
use std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// ---- gRPC: retry actually retries -----------------------------------------

/// A `VersionService` that fails with `Unavailable` for the first `fail_before`
/// calls, then succeeds — counting every call.
#[derive(Clone)]
struct FlakyVersion {
    calls: Arc<AtomicUsize>,
    fail_before: usize,
}

#[tonic::async_trait]
impl VersionService for FlakyVersion {
    async fn get_ledger_api_version(
        &self,
        _request: Request<pb::GetLedgerApiVersionRequest>,
    ) -> Result<Response<pb::GetLedgerApiVersionResponse>, Status> {
        let n = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
        if n < self.fail_before {
            return Err(Status::unavailable("warming up"));
        }
        Ok(Response::new(pb::GetLedgerApiVersionResponse {
            version: "recovered".to_string(),
            ..Default::default()
        }))
    }
}

async fn start_version_server(mock: FlakyVersion) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(VersionServiceServer::new(mock), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn retry_recovers_from_transient_unavailable() {
    let calls = Arc::new(AtomicUsize::new(0));
    let url = start_version_server(FlakyVersion {
        calls: calls.clone(),
        fail_before: 3, // fail attempts 1 and 2, succeed on 3
    })
    .await;

    let client = CantonClient::connect_lazy(
        Config::new(url).with_retry(
            RetryConfig::default()
                .with_max_attempts(5)
                .with_initial_backoff(Duration::from_millis(1))
                .with_max_backoff(Duration::from_millis(1)),
        ),
    )
    .unwrap();

    let version = client.version().await.expect("retry should recover");
    assert_eq!(version, "recovered");
    assert_eq!(
        calls.load(Ordering::SeqCst),
        3,
        "server should see 3 attempts"
    );
}

#[tokio::test]
async fn non_retriable_status_is_not_retried() {
    // Server that always returns InvalidArgument (non-retriable).
    #[derive(Clone)]
    struct AlwaysInvalid(Arc<AtomicUsize>);
    #[tonic::async_trait]
    impl VersionService for AlwaysInvalid {
        async fn get_ledger_api_version(
            &self,
            _r: Request<pb::GetLedgerApiVersionRequest>,
        ) -> Result<Response<pb::GetLedgerApiVersionResponse>, Status> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(Status::invalid_argument("nope"))
        }
    }
    let calls = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    let svc = AlwaysInvalid(calls.clone());
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(VersionServiceServer::new(svc), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;

    let client = CantonClient::connect_lazy(
        Config::new(format!("http://localhost:{port}"))
            .with_retry(RetryConfig::default().with_initial_backoff(Duration::from_millis(1))),
    )
    .unwrap();
    let result = client.version().await;
    assert!(result.is_err());
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "a non-retriable status must be attempted exactly once"
    );
}

// ---- gRPC: health check maps serving status --------------------------------

use canton_proto::grpc::health::v1 as health_pb;
use health_pb::health_server::{Health, HealthServer};

/// A `Health` service that always answers `Check` with a fixed status.
#[derive(Clone)]
struct FixedHealth(health_pb::health_check_response::ServingStatus);

#[tonic::async_trait]
impl Health for FixedHealth {
    async fn check(
        &self,
        _request: Request<health_pb::HealthCheckRequest>,
    ) -> Result<Response<health_pb::HealthCheckResponse>, Status> {
        Ok(Response::new(health_pb::HealthCheckResponse {
            status: self.0 as i32,
        }))
    }

    type WatchStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<health_pb::HealthCheckResponse, Status>> + Send>,
    >;

    async fn watch(
        &self,
        _request: Request<health_pb::HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        Err(Status::unimplemented("watch not used by the SDK"))
    }
}

async fn start_health_server(status: health_pb::health_check_response::ServingStatus) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(HealthServer::new(FixedHealth(status)), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn health_check_maps_serving_status() {
    use health_pb::health_check_response::ServingStatus;

    let url = start_health_server(ServingStatus::Serving).await;
    let client = CantonClient::connect_lazy(Config::new(url)).unwrap();
    assert_eq!(client.health_check().await.unwrap(), ServingStatus::Serving);
}

#[tokio::test]
async fn health_check_maps_not_serving_status() {
    use health_pb::health_check_response::ServingStatus;

    let url = start_health_server(ServingStatus::NotServing).await;
    let client = CantonClient::connect_lazy(Config::new(url)).unwrap();
    assert_eq!(
        client.health_check().await.unwrap(),
        ServingStatus::NotServing
    );
}

#[tokio::test]
async fn health_check_maps_service_unknown_and_unknown() {
    use health_pb::health_check_response::ServingStatus;

    for status in [ServingStatus::ServiceUnknown, ServingStatus::Unknown] {
        let url = start_health_server(status).await;
        let client = CantonClient::connect_lazy(Config::new(url)).unwrap();
        assert_eq!(
            client.health_check().await.unwrap(),
            status,
            "the i32→enum mapping must round-trip every status"
        );
    }
}

#[tokio::test]
async fn health_check_on_a_down_node_is_a_retriable_error() {
    // Bind then drop a listener to obtain a definitely-closed port.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let client =
        CantonClient::connect_lazy(Config::new(format!("http://localhost:{port}"))).unwrap();
    let error = client
        .health_check()
        .await
        .expect_err("an unreachable node must fail the health probe");
    assert!(
        error.is_retriable(),
        "a transport failure to a down node should be retriable, got {error:?}"
    );
}

// ---- OIDC token endpoint: caching / refresh / errors ------------------------

/// A minimal raw-TCP HTTP endpoint that answers every request with `response`
/// (a full HTTP/1.1 message, `Connection: close`), counting requests.
async fn start_token_endpoint(
    status_line: &'static str,
    body: &'static str,
) -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let hits = Arc::new(AtomicUsize::new(0));
    let hits2 = hits.clone();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };
            hits2.fetch_add(1, Ordering::SeqCst);
            let body = body.to_string();
            let status_line = status_line.to_string();
            tokio::spawn(async move {
                let mut buf = [0u8; 2048];
                let _ = socket.read(&mut buf).await; // drain the request
                let response = format!(
                    "{status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = socket.write_all(response.as_bytes()).await;
                let _ = socket.shutdown().await;
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    (format!("http://localhost:{port}/token"), hits)
}

#[tokio::test]
async fn token_is_cached_within_its_ttl() {
    let (url, hits) = start_token_endpoint(
        "HTTP/1.1 200 OK",
        r#"{"access_token":"tok-1","expires_in":3600}"#,
    )
    .await;
    let provider = TokenProvider::new(OidcConfig::new(url, "client", "secret"));

    assert_eq!(provider.token().await.unwrap(), "tok-1");
    assert_eq!(provider.token().await.unwrap(), "tok-1");
    assert_eq!(
        hits.load(Ordering::SeqCst),
        1,
        "second token() within TTL must be served from cache"
    );
}

#[tokio::test]
async fn token_refreshes_after_expiry() {
    // expires_in 31 → TTL = 31 - 30 (skew) = 1s.
    let (url, hits) = start_token_endpoint(
        "HTTP/1.1 200 OK",
        r#"{"access_token":"tok","expires_in":31}"#,
    )
    .await;
    let provider = TokenProvider::new(OidcConfig::new(url, "client", "secret"));

    provider.token().await.unwrap();
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    tokio::time::sleep(Duration::from_millis(1200)).await;
    provider.token().await.unwrap();
    assert_eq!(
        hits.load(Ordering::SeqCst),
        2,
        "token past its TTL must be refreshed"
    );
}

#[tokio::test]
async fn token_endpoint_401_is_an_auth_error() {
    let (url, _hits) =
        start_token_endpoint("HTTP/1.1 401 Unauthorized", r#"{"error":"invalid_client"}"#).await;
    let provider = TokenProvider::new(OidcConfig::new(url, "client", "bad-secret"));

    let error = provider.token().await.expect_err("a 401 must fail");
    match &error {
        canton_ledger::Error::Auth(message) => {
            assert!(message.contains("invalid_client"), "got {message}");
        }
        other => panic!("expected Error::Auth, got {other:?}"),
    }
    assert!(
        !error.is_retriable(),
        "a credential rejection is not retriable"
    );
}

#[tokio::test]
async fn token_endpoint_5xx_stays_a_retriable_http_error() {
    let (url, _hits) =
        start_token_endpoint("HTTP/1.1 503 Service Unavailable", r#"{"error":"down"}"#).await;
    let provider = TokenProvider::new(OidcConfig::new(url, "client", "secret"));

    let error = provider.token().await.expect_err("a 503 must fail");
    match &error {
        canton_ledger::Error::Http { status, .. } => assert_eq!(*status, 503),
        other => panic!("expected Error::Http{{503}}, got {other:?}"),
    }
    assert!(error.is_retriable(), "a transient IdP failure is retriable");
}

// ---- gRPC: resumable update stream actually reconnects ----------------------

fn tx_update(offset: i64) -> pb::GetUpdatesResponse {
    pb::GetUpdatesResponse {
        update: Some(pb::get_updates_response::Update::Transaction(
            pb::Transaction {
                update_id: format!("u{offset}"),
                offset,
                ..Default::default()
            },
        )),
    }
}

/// UpdateService whose first `get_updates` yields offsets 1,2 then drops with
/// `Unavailable`; the second yields 3,4 then ends. Records each request's
/// `begin_exclusive` so the test can prove the client resumed from the last
/// offset.
#[derive(Clone, Default)]
struct FlakyUpdates {
    calls: Arc<AtomicUsize>,
    begins: Arc<Mutex<Vec<i64>>>,
}

#[tonic::async_trait]
impl UpdateService for FlakyUpdates {
    type GetUpdatesStream =
        Pin<Box<dyn tokio_stream::Stream<Item = Result<pb::GetUpdatesResponse, Status>> + Send>>;

    async fn get_updates(
        &self,
        request: Request<pb::GetUpdatesRequest>,
    ) -> Result<Response<Self::GetUpdatesStream>, Status> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
        self.begins
            .lock()
            .unwrap()
            .push(request.into_inner().begin_exclusive);
        let items: Vec<Result<pb::GetUpdatesResponse, Status>> = if call == 1 {
            vec![
                Ok(tx_update(1)),
                Ok(tx_update(2)),
                Err(Status::unavailable("connection dropped")),
            ]
        } else {
            vec![Ok(tx_update(3)), Ok(tx_update(4))]
        };
        Ok(Response::new(Box::pin(tokio_stream::iter(items))))
    }

    async fn get_update_by_offset(
        &self,
        _r: Request<pb::GetUpdateByOffsetRequest>,
    ) -> Result<Response<pb::GetUpdateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn get_update_by_id(
        &self,
        _r: Request<pb::GetUpdateByIdRequest>,
    ) -> Result<Response<pb::GetUpdateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn get_update_by_hash(
        &self,
        _r: Request<pb::GetUpdateByHashRequest>,
    ) -> Result<Response<pb::GetUpdateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn get_updates_page(
        &self,
        _r: Request<pb::GetUpdatesPageRequest>,
    ) -> Result<Response<pb::GetUpdatesPageResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
}

#[tokio::test]
async fn resumable_stream_reconnects_from_last_offset() {
    let mock = FlakyUpdates::default();
    let calls = mock.calls.clone();
    let begins = mock.begins.clone();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(UpdateServiceServer::new(mock), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;

    let client =
        CantonClient::connect_lazy(Config::new(format!("http://localhost:{port}"))).unwrap();

    // Collect the continuous sequence across the mid-stream drop.
    let stream = client.updates_resumable(vec!["p".to_string()], 0);
    tokio::pin!(stream);
    let mut offsets = Vec::new();
    while offsets.len() < 4 {
        let next = tokio::time::timeout(Duration::from_secs(5), stream.next())
            .await
            .expect("timed out waiting for updates");
        match next {
            Some(Ok(pb::get_updates_response::Update::Transaction(t))) => offsets.push(t.offset),
            Some(Ok(_)) => {}
            Some(Err(e)) => panic!("stream errored: {e}"),
            None => break,
        }
    }

    assert_eq!(
        offsets,
        vec![1, 2, 3, 4],
        "no updates lost across the reconnect"
    );
    assert_eq!(
        calls.load(Ordering::SeqCst),
        2,
        "the stream should have reconnected exactly once"
    );
    assert_eq!(
        *begins.lock().unwrap(),
        vec![0, 2],
        "the reconnect must resume from the last yielded offset (2)"
    );
}

// ---- gRPC: resumable ACS resumes from the last page token -------------------

use pb::state_service_server::{StateService, StateServiceServer};

fn acs_entry(cid: &str) -> pb::GetActiveContractsResponse {
    pb::GetActiveContractsResponse {
        contract_entry: Some(
            pb::get_active_contracts_response::ContractEntry::ActiveContract(pb::ActiveContract {
                created_event: Some(pb::CreatedEvent {
                    contract_id: cid.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        ),
        ..Default::default()
    }
}

/// A `StateService` whose page reads serve c1,c2 (token "t1"), then fail ONCE
/// with `Unavailable` at token "t1", then serve c3 (end) — recording each
/// request's page token so the test can prove resumption from the last token.
#[derive(Clone, Default)]
struct FlakyAcs {
    calls: Arc<AtomicUsize>,
    tokens: Arc<Mutex<Vec<Vec<u8>>>>,
}

#[tonic::async_trait]
impl StateService for FlakyAcs {
    async fn get_active_contracts_page(
        &self,
        request: Request<pb::GetActiveContractsPageRequest>,
    ) -> Result<Response<pb::GetActiveContractsPageResponse>, Status> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
        let token = request.into_inner().page_token.unwrap_or_default();
        self.tokens.lock().unwrap().push(token.clone());
        match (call, token.as_slice()) {
            (1, b"") => Ok(Response::new(pb::GetActiveContractsPageResponse {
                active_contracts: vec![acs_entry("c1"), acs_entry("c2")],
                next_page_token: Some(b"t1".to_vec()),
                ..Default::default()
            })),
            (2, b"t1") => Err(Status::unavailable("blip")), // transient drop
            (_, b"t1") => Ok(Response::new(pb::GetActiveContractsPageResponse {
                active_contracts: vec![acs_entry("c3")],
                next_page_token: None,
                ..Default::default()
            })),
            other => Err(Status::invalid_argument(format!("unexpected {other:?}"))),
        }
    }

    type GetActiveContractsStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<pb::GetActiveContractsResponse, Status>> + Send>,
    >;
    async fn get_active_contracts(
        &self,
        _r: Request<pb::GetActiveContractsRequest>,
    ) -> Result<Response<Self::GetActiveContractsStream>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn get_connected_synchronizers(
        &self,
        _r: Request<pb::GetConnectedSynchronizersRequest>,
    ) -> Result<Response<pb::GetConnectedSynchronizersResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn get_ledger_end(
        &self,
        _r: Request<pb::GetLedgerEndRequest>,
    ) -> Result<Response<pb::GetLedgerEndResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn get_latest_pruned_offsets(
        &self,
        _r: Request<pb::GetLatestPrunedOffsetsRequest>,
    ) -> Result<Response<pb::GetLatestPrunedOffsetsResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
}

#[tokio::test]
async fn resumable_acs_resumes_from_the_last_page_token() {
    let mock = FlakyAcs::default();
    let tokens = mock.tokens.clone();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(StateServiceServer::new(mock), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;

    // Fast reconnect policy via the client's RetryConfig.
    let client = CantonClient::connect_lazy(
        Config::new(format!("http://localhost:{port}")).with_retry(
            RetryConfig::default()
                .with_max_attempts(3)
                .with_initial_backoff(Duration::from_millis(1)),
        ),
    )
    .unwrap();

    let stream = client.active_contracts_resumable(vec!["p".to_string()], 7, 2);
    tokio::pin!(stream);
    let mut cids = Vec::new();
    while let Some(item) = stream.next().await {
        let contract = item.expect("stream should recover across the blip");
        cids.push(contract.created_event.unwrap().contract_id);
    }

    assert_eq!(
        cids,
        vec!["c1", "c2", "c3"],
        "no contracts lost or duplicated"
    );
    let seen = tokens.lock().unwrap();
    assert_eq!(
        *seen,
        vec![b"".to_vec(), b"t1".to_vec(), b"t1".to_vec()],
        "the retry must resume from the last continuation token, not restart"
    );
}
