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
