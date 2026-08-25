//! In-process WebSocket tests (feature `ws`): checkpoint filtering, error
//! frames, and resumable reconnect — no live node.
//!
//! A mock WS server (the same `tokio-tungstenite` the `ws` feature pulls in)
//! echoes the `daml.ws.auth` subprotocol, reads the subscription frame, then
//! replies with a scripted set of frames.
#![cfg(feature = "ws")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::large_futures,
    clippy::result_large_err
)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_ledger::JsonClient;
use futures_util::{SinkExt as _, StreamExt as _};
use serde_json::{Value, json};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

fn update_frame(offset: i64) -> String {
    json!({ "update": { "Transaction": { "value": { "updateId": format!("u{offset}"), "offset": offset } } } })
        .to_string()
}
fn checkpoint_frame(offset: i64) -> String {
    json!({ "update": { "OffsetCheckpoint": { "value": { "offset": offset } } } }).to_string()
}
fn error_frame() -> String {
    json!({ "code": "X", "cause": "boom", "errorCategory": 2, "grpcCodeValue": 7 }).to_string()
}
fn completion_frame(command_id: &str) -> String {
    json!({ "completionResponse": { "Completion": { "value": { "commandId": command_id } } } })
        .to_string()
}
fn completion_checkpoint_frame(offset: i64) -> String {
    json!({ "completionResponse": { "OffsetCheckpoint": { "value": { "offset": offset } } } })
        .to_string()
}

/// Accept a WS handshake, echoing the `daml.ws.auth` subprotocol the client
/// requires (the tungstenite client rejects a server that ignores it).
async fn accept(stream: TcpStream) -> WebSocketStream<TcpStream> {
    use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
    use tokio_tungstenite::tungstenite::http::HeaderValue;
    tokio_tungstenite::accept_hdr_async(stream, |_req: &Request, mut resp: Response| {
        resp.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static("daml.ws.auth"),
        );
        Ok(resp)
    })
    .await
    .unwrap()
}

/// Read (and discard) the client's single subscription frame, returning its
/// `beginExclusive` if present.
async fn read_begin(ws: &mut WebSocketStream<TcpStream>) -> Option<i64> {
    match ws.next().await {
        Some(Ok(Message::Text(text))) => serde_json::from_str::<Value>(text.as_str())
            .ok()?
            .get("beginExclusive")
            .and_then(Value::as_i64),
        _ => None,
    }
}

/// A server that serves every connection the same `frames` then closes.
async fn start_scripted_server(frames: Vec<String>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let frames = frames.clone();
            tokio::spawn(async move {
                let mut ws = accept(stream).await;
                let _ = read_begin(&mut ws).await;
                for frame in frames {
                    let _ = ws.send(Message::text(frame)).await;
                }
                let _ = ws.close(None).await;
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn ws_updates_filters_checkpoints() {
    let url =
        start_scripted_server(vec![update_frame(1), checkpoint_frame(2), update_frame(3)]).await;
    let client = JsonClient::new(url);

    let stream = client
        .ws_updates(vec!["p".to_string()], 0, Some(3))
        .await
        .unwrap();
    tokio::pin!(stream);
    let mut offsets = Vec::new();
    while let Some(item) = stream.next().await {
        let frame = item.unwrap();
        offsets.push(
            frame["update"]["Transaction"]["value"]["offset"]
                .as_i64()
                .unwrap(),
        );
    }
    assert_eq!(
        offsets,
        vec![1, 3],
        "the checkpoint frame must be filtered out"
    );
}

#[tokio::test]
async fn ws_error_frame_surfaces_as_err_then_ends() {
    let url = start_scripted_server(vec![update_frame(1), error_frame()]).await;
    let client = JsonClient::new(url);

    let stream = client
        .ws_updates(vec!["p".to_string()], 0, Some(9))
        .await
        .unwrap();
    tokio::pin!(stream);

    assert!(
        stream.next().await.unwrap().is_ok(),
        "first frame is a real update"
    );
    assert!(
        stream.next().await.unwrap().is_err(),
        "the JsCantonError frame must surface as Err"
    );
    assert!(
        stream.next().await.is_none(),
        "the stream ends after the error"
    );
}

#[tokio::test]
async fn ws_completions_filters_checkpoints() {
    let url = start_scripted_server(vec![
        completion_frame("c1"),
        completion_checkpoint_frame(2),
        completion_frame("c2"),
    ])
    .await;
    let client = JsonClient::new(url);

    let stream = client
        .ws_completions(vec!["p".to_string()], 0)
        .await
        .unwrap();
    tokio::pin!(stream);
    let mut ids = Vec::new();
    while let Some(item) = stream.next().await {
        let frame = item.unwrap();
        ids.push(
            frame["completionResponse"]["Completion"]["value"]["commandId"]
                .as_str()
                .unwrap()
                .to_string(),
        );
    }
    assert_eq!(
        ids,
        vec!["c1", "c2"],
        "completion checkpoints must be filtered"
    );
}

/// A server whose first connection yields offsets 1,2 then closes (a drop), and
/// whose second yields 3,4 — recording each connection's `beginExclusive` so the
/// test can prove the client resumed from the last offset.
async fn start_resumable_server() -> (String, Arc<Mutex<Vec<i64>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let begins = Arc::new(Mutex::new(Vec::<i64>::new()));
    let begins_srv = begins.clone();
    let conn = Arc::new(AtomicUsize::new(0));
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let begins = begins_srv.clone();
            let conn = conn.clone();
            tokio::spawn(async move {
                let n = conn.fetch_add(1, Ordering::SeqCst) + 1;
                let mut ws = accept(stream).await;
                let begin = read_begin(&mut ws).await.unwrap_or(-1);
                begins.lock().unwrap().push(begin);
                let offsets = if n == 1 { [1, 2] } else { [3, 4] };
                for offset in offsets {
                    let _ = ws.send(Message::text(update_frame(offset))).await;
                }
                let _ = ws.close(None).await;
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    (format!("http://localhost:{port}"), begins)
}

#[tokio::test]
async fn ws_updates_resumable_reconnects_from_last_offset() {
    let (url, begins) = start_resumable_server().await;
    let client = JsonClient::new(url);

    let stream = client.ws_updates_resumable(vec!["p".to_string()], 0);
    tokio::pin!(stream);
    let mut offsets = Vec::new();
    while offsets.len() < 4 {
        let next = tokio::time::timeout(Duration::from_secs(5), stream.next())
            .await
            .expect("timed out waiting for a resumed frame");
        match next {
            Some(Ok(frame)) => {
                offsets.push(
                    frame["update"]["Transaction"]["value"]["offset"]
                        .as_i64()
                        .unwrap(),
                );
            }
            Some(Err(e)) => panic!("stream errored: {e}"),
            None => break,
        }
    }

    assert_eq!(
        offsets,
        vec![1, 2, 3, 4],
        "no updates lost across the reconnect"
    );
    let begins = begins.lock().unwrap();
    assert!(begins.len() >= 2, "the client should have reconnected");
    assert_eq!(
        begins[0], 0,
        "first connection starts at the requested offset"
    );
    assert_eq!(
        begins[1], 2,
        "the reconnect must resume from the last offset yielded (2)"
    );
}

// ---- the resumable ACS snapshot, and what giving up reports ----------------

fn acs_frame(contract_id: &str, token: Option<&str>) -> String {
    let mut frame = json!({
        "contractEntry": {
            "JsActiveContract": {
                "createdEvent": { "contractId": contract_id }
            }
        }
    });
    if let Some(token) = token {
        frame["streamContinuationToken"] = Value::String(token.to_string());
    }
    frame.to_string()
}

/// Read the subscription frame and return its `streamContinuationToken`, or the
/// empty string when the client sent none (the first connection).
async fn read_continuation_token(ws: &mut WebSocketStream<TcpStream>) -> String {
    let Some(Ok(Message::Text(text))) = ws.next().await else {
        return String::new();
    };
    serde_json::from_str::<Value>(&text)
        .ok()
        .and_then(|request| {
            request
                .get("streamContinuationToken")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_default()
}

/// First connection: one contract carrying token "t1", then a drop. Second:
/// the rest. Records the token each connection was asked to continue from.
async fn start_resumable_acs_server() -> (String, Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let tokens = Arc::new(Mutex::new(Vec::<String>::new()));
    let tokens_srv = tokens.clone();
    let conn = Arc::new(AtomicUsize::new(0));
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let tokens = tokens_srv.clone();
            let conn = conn.clone();
            tokio::spawn(async move {
                let n = conn.fetch_add(1, Ordering::SeqCst) + 1;
                let mut ws = accept(stream).await;
                let token = read_continuation_token(&mut ws).await;
                tokens.lock().unwrap().push(token);
                if n == 1 {
                    let _ = ws.send(Message::text(acs_frame("c1", Some("t1")))).await;
                    // Drop mid-snapshot, without a close frame.
                } else {
                    let _ = ws.send(Message::text(acs_frame("c2", Some("t2")))).await;
                    let _ = ws.close(None).await;
                }
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    (format!("http://localhost:{port}"), tokens)
}

#[tokio::test]
async fn ws_active_contracts_resumable_continues_from_the_last_token() {
    let (url, tokens) = start_resumable_acs_server().await;
    let client = JsonClient::new(url);

    let stream = client.ws_active_contracts_resumable(vec!["p".to_string()], 7);
    tokio::pin!(stream);
    let mut ids = Vec::new();
    while ids.len() < 2 {
        let next = tokio::time::timeout(Duration::from_secs(5), stream.next())
            .await
            .expect("timed out waiting for the resumed snapshot");
        match next {
            Some(Ok(frame)) => ids.push(
                frame["contractEntry"]["JsActiveContract"]["createdEvent"]["contractId"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            ),
            Some(Err(e)) => panic!("stream errored: {e}"),
            None => break,
        }
    }

    assert_eq!(ids, vec!["c1", "c2"], "no contracts lost across the drop");
    let tokens = tokens.lock().unwrap();
    assert!(tokens.len() >= 2, "the client should have reconnected");
    assert_eq!(tokens[0], "", "the first connection starts the snapshot");
    assert_eq!(
        tokens[1], "t1",
        "the reconnect must continue from the last token, not restart the snapshot"
    );
}

/// A server that accepts the handshake and immediately drops every connection,
/// so the reconnect budget runs out.
async fn start_always_dropping_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut ws = accept(stream).await;
                let _ = ws.next().await; // read the subscription, answer nothing
                let _ = ws.send(Message::text(error_frame())).await;
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn a_ws_stream_that_gives_up_reports_the_failure_that_caused_it() {
    let url = start_always_dropping_server().await;
    let client = JsonClient::new(url).with_retry(
        canton_ledger::RetryConfig::default()
            .with_max_attempts(2)
            .with_initial_backoff(Duration::from_millis(1)),
    );

    let stream = client.ws_active_contracts_resumable(vec!["p".to_string()], 7);
    tokio::pin!(stream);
    let error = tokio::time::timeout(Duration::from_secs(10), stream.next())
        .await
        .expect("timed out waiting for the stream to give up")
        .expect("the stream must yield the failure rather than ending silently")
        .expect_err("a server that answers with an error frame produces no contracts");

    // The gRPC streams stopped substituting an error of their own; the
    // WebSocket ones were still doing it, and this is the assertion that says
    // so. `errorCategory: 2` is Canton's transient-server-failure category, so
    // the classification has to survive too.
    assert!(
        format!("{error}").contains("boom"),
        "the participant's own message must survive: {error}"
    );
    assert!(
        error.is_retriable(),
        "and so must its classification: {error:?}"
    );
}

// ---- the JSON recovery handle reads its answer off the completion stream ----

fn full_completion_frame(command_id: &str, act_as: &[&str], update_id: &str, code: i64) -> String {
    json!({
        "completionResponse": { "Completion": { "value": {
            "commandId": command_id,
            "userId": "app-1",
            "actAs": act_as,
            "updateId": update_id,
            "status": { "code": code, "message": if code == 0 { "" } else { "rejected: DUPLICATE_COMMAND" } },
        } } }
    })
    .to_string()
}

/// A completion stream carrying somebody else's completion under the same
/// command id, then ours — the collision `matches_json` exists to resolve.
async fn start_completion_scan_server(our_code: i64) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut ws = accept(stream).await;
                let _ = ws.next().await; // subscription frame
                let _ = ws
                    .send(Message::text(full_completion_frame(
                        "shared-id",
                        &["alice", "intruder"],
                        "not-ours",
                        0,
                    )))
                    .await;
                let _ = ws
                    .send(Message::text(full_completion_frame(
                        "shared-id",
                        &["alice"],
                        "ours",
                        our_code,
                    )))
                    .await;
                let _ = ws.close(None).await;
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn json_recover_finds_its_own_completion_by_the_whole_change_id() {
    let url = start_completion_scan_server(0).await;
    let client = JsonClient::new(url);

    let submission = client.submission(
        canton_ledger::JsonCommands::new(vec!["alice".to_string()])
            .with_user_id("app-1")
            .with_command_id("shared-id"),
    );
    let completion = submission
        .recover(0, Duration::from_secs(5))
        .await
        .expect("our completion is on the stream");
    assert_eq!(
        completion["updateId"], "ours",
        "a same-command-id completion with a different act_as set must not win"
    );
}

#[tokio::test]
async fn json_recover_reports_a_rejection_as_command_rejected() {
    let url = start_completion_scan_server(6).await; // 6 = ALREADY_EXISTS
    let client = JsonClient::new(url);

    let submission = client.submission(
        canton_ledger::JsonCommands::new(vec!["alice".to_string()])
            .with_user_id("app-1")
            .with_command_id("shared-id"),
    );
    let error = submission
        .recover(0, Duration::from_secs(5))
        .await
        .expect_err("a non-OK completion status is the ledger's answer");
    assert!(
        matches!(&error, canton_ledger::Error::CommandRejected { code, .. } if code == "6"),
        "got {error:?}"
    );
}
