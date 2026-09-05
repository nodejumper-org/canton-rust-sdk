//! The JSON transport's HTTP surface, against a scripted in-process server.
//!
//! These paths existed only in the env-gated live suite, which means CI — where
//! no participant is reachable — never ran them at all. That is the gap the M1
//! review named about live tests generally, and command submission over JSON is
//! the largest surface it applied to: four endpoints whose request shape is the
//! whole point (the two `commands/*` endpoints take the command set *as* the
//! body, where `submit-and-wait-for-transaction` wraps it in a request object,
//! and getting that wrong is a 400 nobody sees until a node is in front of it).
//!
//! So the assertions here are about what goes out on the wire, not only what
//! comes back.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_ledger::{JsonClient, JsonCommands};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpListener;

/// One captured request: the line, the headers, and the JSON body.
#[derive(Clone, Debug)]
struct Captured {
    method: String,
    path: String,
    body: serde_json::Value,
}

/// An HTTP server that answers each request with the next scripted response and
/// records what it was asked. One connection per request (`Connection: close`),
/// which is all `reqwest` needs and keeps the parser here honest.
async fn scripted_server(responses: Vec<(u16, String)>) -> (String, Arc<Mutex<Vec<Captured>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let captured = seen.clone();

    tokio::spawn(async move {
        for (status, body) in responses {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            // Read until the body is complete: headers, then Content-Length bytes.
            let mut raw = Vec::new();
            let mut buf = [0u8; 4096];
            loop {
                let read = match socket.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                raw.extend_from_slice(&buf[..read]);
                let text = String::from_utf8_lossy(&raw).into_owned();
                let Some(head_end) = text.find("\r\n\r\n") else {
                    continue;
                };
                let head = &text[..head_end];
                let want: usize = head
                    .lines()
                    .find_map(|l| {
                        let (name, value) = l.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse().ok())?
                    })
                    .unwrap_or(0);
                if raw.len() >= head_end + 4 + want {
                    break;
                }
            }

            let text = String::from_utf8_lossy(&raw).into_owned();
            let (head, body_text) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
            let mut parts = head.lines().next().unwrap_or_default().split_whitespace();
            captured.lock().unwrap().push(Captured {
                method: parts.next().unwrap_or_default().to_string(),
                path: parts.next().unwrap_or_default().to_string(),
                body: serde_json::from_str(body_text).unwrap_or(serde_json::Value::Null),
            });

            let response = format!(
                "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        }
    });

    tokio::time::sleep(Duration::from_millis(100)).await;
    (format!("http://127.0.0.1:{port}"), seen)
}

fn commands() -> JsonCommands {
    JsonCommands::new(vec!["alice".to_string()])
        .with_command_id("cmd-1")
        .add_create("#pkg:M:T", serde_json::json!({"owner": "alice"}))
}

#[tokio::test]
async fn async_submit_posts_the_command_set_as_the_body() {
    let (url, seen) = scripted_server(vec![(200, "{}".to_string())]).await;
    let client = JsonClient::new(url);

    client.submit(&commands()).await.expect("a 200 is success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/v2/commands/async/submit");
    // The command set *is* the body here — not wrapped in a request object.
    // A body of `{"commands": {...}}` is what the participant answers 400 to.
    assert_eq!(request.body["commandId"], "cmd-1");
    assert_eq!(request.body["actAs"][0], "alice");
    assert!(
        request
            .body
            .get("commands")
            .is_some_and(serde_json::Value::is_array),
        "the commands array sits at the top level: {}",
        request.body
    );
}

#[tokio::test]
async fn submit_and_wait_reads_back_where_the_command_landed() {
    let (url, seen) = scripted_server(vec![(
        200,
        r#"{"updateId":"u-1","completionOffset":42}"#.to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let response = client.submit_and_wait(&commands()).await.expect("success");

    assert_eq!(response.update_id, "u-1");
    assert_eq!(response.completion_offset, 42);
    assert_eq!(seen.lock().unwrap()[0].path, "/v2/commands/submit-and-wait");
}

#[tokio::test]
async fn events_by_contract_id_sends_a_contract_id_and_an_event_format() {
    let (url, seen) = scripted_server(vec![(200, r#"{"created":{}}"#.to_string())]).await;
    let client = JsonClient::new(url);

    client
        .events_by_contract_id("cid-7", vec!["alice".to_string()])
        .await
        .expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(request.path, "/v2/events/events-by-contract-id");
    assert_eq!(request.body["contractId"], "cid-7");
    // Without a party filter the participant answers with nothing visible, so
    // the format is not optional decoration.
    assert!(
        request.body["eventFormat"]["filtersByParty"]
            .get("alice")
            .is_some(),
        "the reading party must reach the event format: {}",
        request.body
    );
}

#[tokio::test]
async fn a_json_submission_knows_its_identity_before_it_is_sent() {
    let (url, _seen) = scripted_server(vec![(200, "{}".to_string())]).await;
    let client = JsonClient::new(url);

    let submission = client.submission(commands());
    let change_id = submission.change_id().clone();

    // The same property the gRPC handle has: an ambiguous send is recoverable
    // only if the identity existed beforehand.
    assert_eq!(change_id.command_id(), "cmd-1");
    assert_eq!(change_id.act_as(), ["alice".to_string()]);

    submission.submit().await.expect("the scripted 200");
    assert_eq!(submission.change_id(), &change_id, "identity is stable");
}

#[tokio::test]
async fn a_generated_command_id_is_visible_on_the_json_handle_too() {
    let (url, seen) = scripted_server(vec![(200, "{}".to_string())]).await;
    let client = JsonClient::new(url);

    // No `with_command_id`: the SDK generates one, and the whole point of the
    // handle is that the caller can still see it.
    let submission = client.submission(JsonCommands::new(vec!["alice".to_string()]));
    let generated = submission.change_id().command_id().to_string();
    assert!(generated.starts_with("sdk-"), "got {generated}");

    submission.submit().await.expect("the scripted 200");
    assert_eq!(
        seen.lock().unwrap()[0].body["commandId"],
        generated,
        "the id the caller was given is the id that went to the participant"
    );
}

#[tokio::test]
async fn an_http_error_keeps_its_status_and_body() {
    let (url, _seen) = scripted_server(vec![(
        503,
        r#"{"errorCategory":1,"cause":"participant restarting"}"#.to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let error = client
        .submit_and_wait(&commands())
        .await
        .expect_err("503 is a failure");

    // 5xx stays retriable through the shared error model, and the body survives
    // so a caller can read the participant's own reason.
    assert!(error.is_retriable(), "{error:?}");
    assert!(
        format!("{error}").contains("participant restarting"),
        "{error}"
    );
}

/// `from_env_for` builds the variable name it looked for out of the role, and
/// says which one it was. The name-mangling — uppercase, dashes to underscores
/// — is the part with a bug in it, and the message is what a user sees when a
/// network was not exported into their shell.
#[test]
fn a_missing_endpoint_names_the_variable_it_looked_for() {
    // Deliberately a role nothing exports: the assertion is about the message,
    // and a role that happened to be set would test nothing.
    let error = JsonClient::from_env_for("app-user")
        .expect_err("no local network is exported in the test environment");

    let message = format!("{error}");
    assert!(
        message.contains("CANTON_APP_USER_JSON_LEDGER_API_URL"),
        "the message must name the variable, dashes folded to underscores: {message}"
    );
    assert!(
        message.contains("canton-devkit localnet env"),
        "and say how to produce it: {message}"
    );
}

/// A retry the participant refuses as a duplicate is our own earlier attempt
/// having landed — over JSON exactly as over gRPC. Reporting it as a failure
/// tells the caller their command did not happen when it did, which is the
/// failure the change ID exists to prevent.
#[tokio::test]
async fn a_de_duplicated_retry_is_a_success_not_a_failure() {
    // First attempt: a 503 the client will retry. Second: the participant
    // refusing the same change id as a duplicate.
    let (url, seen) = scripted_server(vec![
        (
            503,
            r#"{"cause":"connection lost after acceptance"}"#.to_string(),
        ),
        (
            409,
            r#"{"code":"DUPLICATE_COMMAND","cause":"command already exists"}"#.to_string(),
        ),
    ])
    .await;
    let client = JsonClient::new(url).with_retry(
        canton_ledger::RetryConfig::default()
            .with_max_attempts(3)
            .with_initial_backoff(Duration::from_millis(1)),
    );

    client
        .submit(&commands())
        .await
        .expect("the command committed on the attempt whose response was lost");

    let requests = seen.lock().unwrap();
    assert_eq!(requests.len(), 2, "one attempt, then one retry");
    assert_eq!(
        requests[0].body["commandId"], requests[1].body["commandId"],
        "the retry must reuse the change id — that is what makes it a duplicate"
    );
}

/// The same rejection on the *first* attempt is a real one: nothing of ours is
/// at the participant, so the caller reused a change id from an earlier
/// submission and needs to hear about it.
#[tokio::test]
async fn a_duplicate_on_the_first_attempt_is_still_an_error() {
    let (url, _seen) = scripted_server(vec![(
        409,
        r#"{"code":"DUPLICATE_COMMAND","cause":"command already exists"}"#.to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let error = client
        .submit(&commands())
        .await
        .expect_err("a change id the caller reused is a rejection they must see");
    assert!(format!("{error}").contains("DUPLICATE_COMMAND"), "{error}");
}

// ---- Packages (issue #2) ----------------------------------------------------

#[tokio::test]
async fn list_packages_asks_the_packages_path_and_returns_every_id() {
    let (url, seen) =
        scripted_server(vec![(200, r#"{"packageIds":["aa11","bb22"]}"#.to_string())]).await;
    let client = JsonClient::new(url);

    let ids = client.list_packages().await.expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(request.method, "GET");
    assert_eq!(request.path, "/v2/packages");
    assert_eq!(ids, vec!["aa11".to_string(), "bb22".to_string()]);
}

#[tokio::test]
async fn package_status_names_the_package_in_the_path_and_speaks_the_grpc_vocabulary() {
    use canton_ledger::proto::PackageStatus;

    let (url, seen) = scripted_server(vec![(
        200,
        r#"{"packageStatus":"PACKAGE_STATUS_REGISTERED"}"#.to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let status = client.package_status("deadbeef").await.expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(request.method, "GET");
    assert_eq!(request.path, "/v2/packages/deadbeef/status");
    assert_eq!(status, PackageStatus::Registered);
}

#[tokio::test]
async fn a_package_status_this_build_does_not_know_is_an_error_not_unspecified() {
    let (url, _seen) = scripted_server(vec![(
        200,
        r#"{"packageStatus":"PACKAGE_STATUS_FROM_THE_FUTURE"}"#.to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let error = client
        .package_status("deadbeef")
        .await
        .expect_err("an unknown status is not silently unspecified");

    assert!(
        matches!(error, canton_ledger::Error::UnexpectedResponse(_)),
        "{error:?}"
    );
    assert!(format!("{error}").contains("FROM_THE_FUTURE"), "{error}");
}

#[tokio::test]
async fn a_package_id_that_cannot_be_a_path_segment_never_reaches_the_wire() {
    // No scripted responses: a request reaching the server would hang the
    // test, which is the assertion.
    let (url, seen) = scripted_server(vec![]).await;
    let client = JsonClient::new(url);

    for bad in ["", "a/b", "id?x", "id#1", "with space", "%2e%2e"] {
        let error = client
            .package_status(bad)
            .await
            .expect_err("refused before sending");
        assert!(
            matches!(error, canton_ledger::Error::InvalidRequest(_)),
            "{bad:?} -> {error:?}"
        );
    }
    assert!(seen.lock().unwrap().is_empty(), "nothing was sent");
}
