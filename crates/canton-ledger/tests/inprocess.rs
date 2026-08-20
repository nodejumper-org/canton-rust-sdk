//! In-process tests that need a controllable server but no live node.
//!
//! Uses the dev-only `canton-proto/server` feature for a gRPC mock, and a tiny
//! raw-TCP HTTP server for the OIDC token endpoint.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use canton_auth::{OidcConfig, TokenProvider};
use canton_ledger::{AcsEntry, CantonClient, ChangeId, Config, RetryConfig};
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

// ---- gRPC: the resumable stream honours checkpoints and keeps the cause -----

fn checkpoint_update(offset: i64) -> pb::GetUpdatesResponse {
    pb::GetUpdatesResponse {
        update: Some(pb::get_updates_response::Update::OffsetCheckpoint(
            pb::OffsetCheckpoint {
                offset,
                synchronizer_times: Vec::new(),
            },
        )),
    }
}

/// `UpdateService` whose first stream carries nothing but a checkpoint at
/// offset 10 before dropping with `Unavailable`. A client that tracks only the
/// updates it yielded has no position to resume from and restarts at 0 — which
/// after pruning the participant can refuse outright.
#[derive(Clone, Default)]
struct CheckpointThenDrop {
    calls: Arc<AtomicUsize>,
    begins: Arc<Mutex<Vec<i64>>>,
}

#[tonic::async_trait]
impl UpdateService for CheckpointThenDrop {
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
                Ok(checkpoint_update(10)),
                Err(Status::unavailable("connection dropped")),
            ]
        } else {
            vec![Ok(tx_update(11))]
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

async fn serve_updates<S>(service: S) -> u16
where
    S: UpdateService,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(UpdateServiceServer::new(service), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    port
}

#[tokio::test]
async fn resumable_stream_restarts_after_the_latest_checkpoint() {
    let mock = CheckpointThenDrop::default();
    let begins = mock.begins.clone();
    let port = serve_updates(mock).await;

    let client =
        CantonClient::connect_lazy(Config::new(format!("http://localhost:{port}"))).unwrap();

    let stream = client.updates_resumable(vec!["p".to_string()], 0);
    tokio::pin!(stream);
    let first = tokio::time::timeout(Duration::from_secs(5), stream.next())
        .await
        .expect("timed out waiting for the update after the reconnect");
    match first {
        Some(Ok(pb::get_updates_response::Update::Transaction(t))) => assert_eq!(t.offset, 11),
        other => panic!("expected the transaction that follows the checkpoint, got {other:?}"),
    }

    assert_eq!(
        *begins.lock().unwrap(),
        vec![0, 10],
        "the reconnect must resume from the checkpoint the participant sent, not from 0"
    );
}

/// `UpdateService` that is never available, so the reconnect budget runs out.
#[derive(Clone, Default)]
struct AlwaysUnavailable;

#[tonic::async_trait]
impl UpdateService for AlwaysUnavailable {
    type GetUpdatesStream =
        Pin<Box<dyn tokio_stream::Stream<Item = Result<pb::GetUpdatesResponse, Status>> + Send>>;

    async fn get_updates(
        &self,
        _request: Request<pb::GetUpdatesRequest>,
    ) -> Result<Response<Self::GetUpdatesStream>, Status> {
        Err(Status::unavailable("participant is restarting"))
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
async fn exhausted_reconnects_report_the_participants_own_failure() {
    let port = serve_updates(AlwaysUnavailable).await;

    let client = CantonClient::connect_lazy(
        Config::new(format!("http://localhost:{port}")).with_retry(
            RetryConfig::default()
                .with_max_attempts(2)
                .with_initial_backoff(Duration::from_millis(1)),
        ),
    )
    .unwrap();

    let stream = client.updates_resumable(vec!["p".to_string()], 0);
    tokio::pin!(stream);
    let last = tokio::time::timeout(Duration::from_secs(5), stream.next())
        .await
        .expect("timed out waiting for the stream to give up")
        .expect("the stream must yield the failure rather than ending silently")
        .expect_err("an unavailable participant cannot produce an update");

    // The status, its details and its classification are what an application
    // branches on; replacing them with a message of our own throws all three
    // away at the moment they matter most.
    assert_eq!(
        last.code(),
        Some(tonic::Code::Unavailable),
        "the original gRPC status must survive: {last:?}"
    );
    assert!(
        last.is_retriable(),
        "a transient failure stays classified as retriable: {last:?}"
    );
    assert!(
        format!("{last}").contains("participant is restarting"),
        "the participant's own message must survive: {last}"
    );
}

// ---- gRPC: an ACS snapshot is not only active contracts ---------------------

fn incomplete_unassigned_entry(cid: &str) -> pb::GetActiveContractsResponse {
    pb::GetActiveContractsResponse {
        contract_entry: Some(
            pb::get_active_contracts_response::ContractEntry::IncompleteUnassigned(
                pb::IncompleteUnassigned {
                    created_event: Some(pb::CreatedEvent {
                        contract_id: cid.to_string(),
                        ..Default::default()
                    }),
                    unassigned_event: Some(pb::UnassignedEvent {
                        contract_id: cid.to_string(),
                        ..Default::default()
                    }),
                },
            ),
        ),
        ..Default::default()
    }
}

fn incomplete_assigned_entry(cid: &str) -> pb::GetActiveContractsResponse {
    pb::GetActiveContractsResponse {
        contract_entry: Some(
            pb::get_active_contracts_response::ContractEntry::IncompleteAssigned(
                pb::IncompleteAssigned {
                    assigned_event: Some(pb::AssignedEvent {
                        reassignment_id: cid.to_string(),
                        ..Default::default()
                    }),
                },
            ),
        ),
        ..Default::default()
    }
}

/// A `StateService` whose single page carries one entry of each kind — the
/// shape a multi-synchronizer participant returns while reassignments are in
/// flight at the snapshot offset.
#[derive(Clone, Default)]
struct MixedAcs;

#[tonic::async_trait]
impl StateService for MixedAcs {
    async fn get_active_contracts_page(
        &self,
        _request: Request<pb::GetActiveContractsPageRequest>,
    ) -> Result<Response<pb::GetActiveContractsPageResponse>, Status> {
        Ok(Response::new(pb::GetActiveContractsPageResponse {
            active_contracts: vec![
                acs_entry("active-1"),
                incomplete_unassigned_entry("leaving-1"),
                incomplete_assigned_entry("arriving-1"),
            ],
            next_page_token: None,
            ..Default::default()
        }))
    }

    type GetActiveContractsStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<pb::GetActiveContractsResponse, Status>> + Send>,
    >;
    async fn get_active_contracts(
        &self,
        _r: Request<pb::GetActiveContractsRequest>,
    ) -> Result<Response<Self::GetActiveContractsStream>, Status> {
        let items: Vec<Result<pb::GetActiveContractsResponse, Status>> = vec![
            Ok(acs_entry("active-1")),
            Ok(incomplete_unassigned_entry("leaving-1")),
            Ok(incomplete_assigned_entry("arriving-1")),
        ];
        Ok(Response::new(Box::pin(tokio_stream::iter(items))))
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

async fn serve_state<S>(service: S) -> u16
where
    S: StateService,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(StateServiceServer::new(service), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    port
}

#[tokio::test]
async fn the_lossless_acs_read_keeps_incomplete_reassignments() {
    let port = serve_state(MixedAcs).await;
    let client =
        CantonClient::connect_lazy(Config::new(format!("http://localhost:{port}"))).unwrap();

    // Paged.
    let (entries, next) = client
        .acs_page(vec!["p".to_string()], 7, 10, None)
        .await
        .unwrap();
    assert!(next.is_none());
    assert_eq!(entries.len(), 3, "every entry kind survives: {entries:?}");
    assert!(matches!(entries[0], AcsEntry::Active(_)));
    assert!(matches!(entries[1], AcsEntry::IncompleteUnassigned(_)));
    assert!(matches!(entries[2], AcsEntry::IncompleteAssigned(_)));

    // Streaming.
    let stream = client.acs_entries(vec!["p".to_string()], 7).await.unwrap();
    tokio::pin!(stream);
    let mut kinds = Vec::new();
    while let Some(entry) = stream.next().await {
        kinds.push(match entry.unwrap() {
            AcsEntry::Active(_) => "active",
            AcsEntry::IncompleteUnassigned(_) => "unassigned",
            AcsEntry::IncompleteAssigned(_) => "assigned",
            // `AcsEntry` is `#[non_exhaustive]`: a kind added by a future
            // Ledger API must fail this test rather than be counted as one of
            // the three.
            other => panic!("unexpected entry kind: {other:?}"),
        });
    }
    assert_eq!(kinds, vec!["active", "unassigned", "assigned"]);

    // Resumable, page-based.
    let stream = client.acs_entries_resumable(vec!["p".to_string()], 7, 10);
    tokio::pin!(stream);
    let mut count = 0;
    while let Some(entry) = stream.next().await {
        entry.unwrap();
        count += 1;
    }
    assert_eq!(count, 3);
}

#[tokio::test]
async fn the_active_only_read_still_returns_active_contracts_only() {
    let port = serve_state(MixedAcs).await;
    let client =
        CantonClient::connect_lazy(Config::new(format!("http://localhost:{port}"))).unwrap();

    // The convenience methods keep their meaning — the point of the lossless
    // read is that it is now a caller's choice rather than the only behaviour.
    let (contracts, _) = client
        .active_contracts_page(vec!["p".to_string()], 7, 10, None)
        .await
        .unwrap();
    assert_eq!(contracts.len(), 1);
    assert_eq!(
        contracts[0].created_event.as_ref().unwrap().contract_id,
        "active-1"
    );

    let stream = client
        .active_contracts(vec!["p".to_string()], 7)
        .await
        .unwrap();
    tokio::pin!(stream);
    let mut ids = Vec::new();
    while let Some(contract) = stream.next().await {
        ids.push(contract.unwrap().created_event.unwrap().contract_id);
    }
    assert_eq!(ids, vec!["active-1"]);
}

// ---- gRPC: recovering a command by its whole identity -----------------------

/// A completion stream carrying, in order: somebody else's completion with the
/// *same* command id, then this command's own. A recovery that matches on the
/// command id alone answers with the first — the wrong application's outcome,
/// reported as this one's.
#[derive(Clone, Default)]
struct CollidingCompletions;

#[tonic::async_trait]
impl pb::command_completion_service_server::CommandCompletionService for CollidingCompletions {
    type CompletionStreamStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<pb::CompletionStreamResponse, Status>> + Send>,
    >;

    async fn completion_stream(
        &self,
        _request: Request<pb::CompletionStreamRequest>,
    ) -> Result<Response<Self::CompletionStreamStream>, Status> {
        let response = |completion: pb::Completion| {
            Ok(pb::CompletionStreamResponse {
                completion_response: Some(
                    pb::completion_stream_response::CompletionResponse::Completion(completion),
                ),
            })
        };
        let items: Vec<Result<pb::CompletionStreamResponse, Status>> = vec![
            response(pb::Completion {
                command_id: "shared-command-id".to_string(),
                user_id: "someone-else".to_string(),
                act_as: vec!["alice".to_string()],
                update_id: "not-ours".to_string(),
                ..Default::default()
            }),
            response(pb::Completion {
                command_id: "shared-command-id".to_string(),
                user_id: "us".to_string(),
                act_as: vec!["alice".to_string()],
                update_id: "ours".to_string(),
                ..Default::default()
            }),
        ];
        Ok(Response::new(Box::pin(tokio_stream::iter(items))))
    }

    type GetCompletionsStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<pb::CompletionStreamResponse, Status>> + Send>,
    >;
    async fn get_completions(
        &self,
        _r: Request<pb::GetCompletionsRequest>,
    ) -> Result<Response<Self::GetCompletionsStream>, Status> {
        Err(Status::unimplemented("test"))
    }
}

#[tokio::test]
async fn recovery_matches_the_whole_change_id() {
    use pb::command_completion_service_server::CommandCompletionServiceServer;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(
                CommandCompletionServiceServer::new(CollidingCompletions),
                incoming,
            )
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;

    let client =
        CantonClient::connect_lazy(Config::new(format!("http://localhost:{port}"))).unwrap();

    let ours = ChangeId::new("us", vec!["alice".to_string()], "shared-command-id");
    let completion = client
        .await_completion(&ours, 0, Duration::from_secs(5))
        .await
        .expect("our completion is on the stream");
    assert_eq!(
        completion.update_id, "ours",
        "a command id is not an identity: the user must match too"
    );
}

#[tokio::test]
async fn a_submission_knows_its_identity_before_it_is_sent() {
    // Nothing is submitted here, and that is the point: the change ID has to
    // exist before the call that might fail ambiguously, or there is nothing
    // to recover with afterwards.
    let client = CantonClient::connect_lazy(Config::new("http://localhost:1")).unwrap();
    let submission = client.submission(canton_ledger::Submit::new("alice").add_command(
        canton_ledger::create(
            canton_ledger::identifier("pkg", "M", "T"),
            canton_ledger::record(vec![]),
        ),
    ));

    let change_id = submission.change_id().clone();
    assert!(change_id.command_id().starts_with("sdk-"));
    assert_eq!(change_id.act_as(), ["alice".to_string()]);
    assert!(
        change_id.user_id().is_empty(),
        "unset means token-derived, and the client must not invent one"
    );

    // And it stays the same across sends, which is what makes a retry
    // de-duplicate rather than execute twice.
    assert_eq!(submission.change_id(), &change_id);
    assert!(submission.submit().await.is_err(), "nothing is listening");
    assert_eq!(submission.change_id(), &change_id);
}

// ---- gRPC: what the submission asks the participant to show it -------------

/// A `CommandService` that records the request and answers with an empty
/// transaction. The interesting part is the *request*: which parties the
/// transaction format filters to.
#[derive(Clone, Default)]
struct CapturingCommands {
    requests: Arc<Mutex<Vec<pb::SubmitAndWaitForTransactionRequest>>>,
}

#[tonic::async_trait]
impl pb::command_service_server::CommandService for CapturingCommands {
    async fn submit_and_wait_for_transaction(
        &self,
        request: Request<pb::SubmitAndWaitForTransactionRequest>,
    ) -> Result<Response<pb::SubmitAndWaitForTransactionResponse>, Status> {
        self.requests.lock().unwrap().push(request.into_inner());
        Ok(Response::new(pb::SubmitAndWaitForTransactionResponse {
            transaction: Some(pb::Transaction {
                update_id: "u-1".to_string(),
                ..Default::default()
            }),
        }))
    }
    async fn submit_and_wait(
        &self,
        _r: Request<pb::SubmitAndWaitRequest>,
    ) -> Result<Response<pb::SubmitAndWaitResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn submit_and_wait_for_reassignment(
        &self,
        _r: Request<pb::SubmitAndWaitForReassignmentRequest>,
    ) -> Result<Response<pb::SubmitAndWaitForReassignmentResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
}

#[tokio::test]
async fn the_transaction_filter_covers_read_as_as_well_as_act_as() {
    use pb::command_service_server::CommandServiceServer;

    let mock = CapturingCommands::default();
    let requests = mock.requests.clone();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(CommandServiceServer::new(mock), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;

    let client =
        CantonClient::connect_lazy(Config::new(format!("http://localhost:{port}"))).unwrap();

    client
        .submit_and_wait_for_transaction(
            canton_ledger::Submit::new("alice")
                .with_read_as(vec!["bob".to_string()])
                .add_command(canton_ledger::create(
                    canton_ledger::identifier("pkg", "M", "T"),
                    canton_ledger::record(vec![]),
                )),
        )
        .await
        .expect("the mock commits");

    let captured = requests.lock().unwrap()[0].clone();
    let filters = captured
        .transaction_format
        .expect("a transaction format")
        .event_format
        .expect("an event format")
        .filters_by_party;

    // Filtering to `act_as` alone returns a transaction whose events for the
    // read-as party are simply absent — a wrong answer that looks like a right
    // one. The Ledger API's own default for a submission is both sets.
    let mut parties: Vec<&String> = filters.keys().collect();
    parties.sort();
    assert_eq!(
        parties,
        vec!["alice", "bob"],
        "read_as must reach the filter too, saw {parties:?}"
    );
}

#[tokio::test]
async fn a_submission_that_cannot_succeed_is_refused_before_it_is_sent() {
    // Nothing is listening on port 1: if any of these reached the network the
    // error would be a connection failure, not an InvalidRequest.
    let client = CantonClient::connect_lazy(Config::new("http://localhost:1")).unwrap();
    let command = || {
        canton_ledger::create(
            canton_ledger::identifier("pkg", "M", "T"),
            canton_ledger::record(vec![]),
        )
    };

    let no_parties = canton_ledger::Submit::new_multi(vec![]).add_command(command());
    let no_commands = canton_ledger::Submit::new("alice");
    let both_times = canton_ledger::Submit::new("alice")
        .add_command(command())
        .with_min_ledger_time_abs(prost_types::Timestamp::default())
        .with_min_ledger_time_rel(Duration::from_secs(1));

    for (name, submit) in [
        ("no acting party", no_parties),
        ("no commands", no_commands),
        ("both min-ledger-times", both_times),
    ] {
        let result = client.submit_and_wait_for_transaction(submit).await;
        assert!(
            matches!(result, Err(canton_ledger::Error::InvalidRequest(_))),
            "{name} should be refused locally, got {result:?}"
        );
    }
}
