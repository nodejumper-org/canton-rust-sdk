//! Compile-time guarantee: every stream the SDK returns is detached from the
//! client's lifetime (precise capturing, `use<>`), so the drop-client /
//! `tokio::spawn` pattern works without pre-cloning. If a signature regresses
//! to capturing `&self`, this file stops compiling (E0505/E0597).
#![allow(clippy::unwrap_used, clippy::large_futures)]

use canton_ledger::{
    ActiveContractsRequest, CantonClient, CompletionsRequest, Config, UpdatesRequest,
};
use tokio_stream::StreamExt as _;

/// gRPC streams outlive the client and are spawnable.
#[tokio::test]
async fn grpc_streams_outlive_the_client() {
    let client = CantonClient::connect_lazy(Config::new("http://localhost:1")).unwrap();

    let resumable = client.updates_resumable(vec!["p".to_string()], 0);
    let resumable_with =
        client.updates_resumable_with(UpdatesRequest::new(vec!["p".to_string()], 0));
    let acs_resumable_with = client
        .active_contracts_resumable_with(ActiveContractsRequest::new(vec!["p".to_string()], 0), 10);
    // These fail at runtime (nothing listens on :1) — only their types matter.
    let completions = client.completions(vec!["p".to_string()], 0).await;
    let acs = client.active_contracts(vec!["p".to_string()], 0).await;
    let acs_with = client
        .active_contracts_with(ActiveContractsRequest::new(vec!["p".to_string()], 0))
        .await;
    let updates = client.updates(vec!["p".to_string()], 0).await;
    let updates_with = client
        .updates_with(UpdatesRequest::new(vec!["p".to_string()], 0).until(10))
        .await;
    let completions_with = client
        .completions_with(CompletionsRequest::new(vec!["p".to_string()], 0))
        .await;
    drop(client);

    // The streams must be usable (and spawnable) after the client is gone.
    let handle = tokio::spawn(async move {
        tokio::pin!(resumable);
        let _ = tokio::time::timeout(std::time::Duration::from_millis(10), resumable.next()).await;
        drop(resumable_with);
        drop(acs_resumable_with);
        drop(completions);
        drop(acs);
        drop(acs_with);
        drop(updates);
        drop(updates_with);
        drop(completions_with);
    });
    handle.await.unwrap();
}

/// WebSocket streams (feature `ws`) outlive the client and are spawnable.
#[cfg(feature = "ws")]
#[tokio::test]
async fn ws_streams_outlive_the_client() {
    use canton_ledger::JsonClient;

    let client = JsonClient::new("http://localhost:1");

    let resumable = client.ws_updates_resumable(vec!["p".to_string()], 0);
    let updates = client.ws_updates(vec!["p".to_string()], 0, None).await;
    let updates_with = client
        .ws_updates_with(&UpdatesRequest::new(vec!["p".to_string()], 0))
        .await;
    let acs = client.ws_active_contracts(vec!["p".to_string()], 0).await;
    let acs_with = client
        .ws_active_contracts_with(&ActiveContractsRequest::new(vec!["p".to_string()], 0))
        .await;
    let completions = client.ws_completions(vec!["p".to_string()], 0).await;
    let completions_with = client
        .ws_completions_with(&CompletionsRequest::new(vec!["p".to_string()], 0))
        .await;
    drop(client);

    let handle = tokio::spawn(async move {
        tokio::pin!(resumable);
        let _ = tokio::time::timeout(std::time::Duration::from_millis(10), resumable.next()).await;
        drop(updates);
        drop(updates_with);
        drop(acs);
        drop(acs_with);
        drop(completions);
        drop(completions_with);
    });
    handle.await.unwrap();
}
