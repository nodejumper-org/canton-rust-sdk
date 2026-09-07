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

// ---- Parties ----------------------------------------------------------------

/// The party details the JSON API sends, in its own spelling.
fn party_json(party: &str, annotations: &str) -> String {
    format!(
        r#"{{"party":"{party}","isLocal":true,"localMetadata":{{"resourceVersion":"3","annotations":{annotations}}},"identityProviderId":""}}"#
    )
}

#[tokio::test]
async fn participant_id_asks_its_own_path() {
    let (url, seen) = scripted_server(vec![(
        200,
        r#"{"participantId":"participant::1220ab"}"#.to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let id = client.participant_id().await.expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(
        (request.method.as_str(), request.path.as_str()),
        ("GET", "/v2/parties/participant-id")
    );
    assert_eq!(id, "participant::1220ab");
}

#[tokio::test]
async fn a_party_page_is_asked_with_an_encoded_token_and_answered_in_the_grpc_type() {
    let body = format!(
        r#"{{"partyDetails":[{}],"nextPageToken":"page/2+x="}}"#,
        party_json("alice::1220aa", r#"{"team":"blue"}"#)
    );
    let (url, seen) = scripted_server(vec![(200, body)]).await;
    let client = JsonClient::new(url);

    let (parties, next) = client
        .list_known_parties_page(2, Some("a&pageSize=9".to_string()))
        .await
        .expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(request.method, "GET");
    // One encoded value: a token that carried `&pageSize=` must not become a
    // second query parameter.
    assert_eq!(
        request.path,
        "/v2/parties?pageSize=2&pageToken=a%26pageSize%3D9"
    );
    assert_eq!(next.as_deref(), Some("page/2+x="));
    assert_eq!(parties.len(), 1);
    let alice = &parties[0];
    assert_eq!(alice.party, "alice::1220aa");
    assert!(alice.is_local);
    let meta = alice.local_metadata.as_ref().expect("metadata");
    assert_eq!(meta.resource_version, "3");
    assert_eq!(
        meta.annotations.get("team").map(String::as_str),
        Some("blue")
    );
}

#[tokio::test]
async fn a_page_with_the_server_default_size_and_no_token_sends_no_query_at_all() {
    let (url, seen) = scripted_server(vec![(200, r#"{"partyDetails":[]}"#.to_string())]).await;
    let client = JsonClient::new(url);

    let (parties, next) = client
        .list_known_parties_page(0, None)
        .await
        .expect("success");

    assert_eq!(seen.lock().unwrap()[0].path, "/v2/parties");
    assert!(parties.is_empty());
    assert_eq!(next, None);
}

#[tokio::test]
async fn listing_every_party_follows_the_token_until_there_is_none() {
    let page = |party: &str, next: &str| {
        format!(
            r#"{{"partyDetails":[{}],"nextPageToken":"{next}"}}"#,
            party_json(party, "{}")
        )
    };
    let (url, seen) = scripted_server(vec![
        (200, page("a::1220aa", "t1")),
        (200, page("b::1220bb", "t2")),
        (200, page("c::1220cc", "")),
    ])
    .await;
    let client = JsonClient::new(url);

    let parties = client.list_known_parties().await.expect("success");

    let paths: Vec<String> = seen
        .lock()
        .unwrap()
        .iter()
        .map(|r| r.path.clone())
        .collect();
    assert_eq!(
        paths,
        vec![
            "/v2/parties",
            "/v2/parties?pageToken=t1",
            "/v2/parties?pageToken=t2"
        ]
    );
    let names: Vec<&str> = parties.iter().map(|p| p.party.as_str()).collect();
    assert_eq!(names, vec!["a::1220aa", "b::1220bb", "c::1220cc"]);
}

#[tokio::test]
async fn a_participant_that_repeats_a_page_token_is_an_error_not_a_prefix() {
    let page = |party: &str| {
        format!(
            r#"{{"partyDetails":[{}],"nextPageToken":"same"}}"#,
            party_json(party, "{}")
        )
    };
    let (url, seen) =
        scripted_server(vec![(200, page("a::1220aa")), (200, page("b::1220bb"))]).await;
    let client = JsonClient::new(url);

    let error = client
        .list_known_parties()
        .await
        .expect_err("a token that does not advance cannot be followed");

    assert!(
        matches!(error, canton_ledger::Error::UnexpectedResponse(_)),
        "{error:?}"
    );
    assert!(format!("{error}").contains("after 2 parties"), "{error}");
    assert_eq!(seen.lock().unwrap().len(), 2, "it stopped at the repeat");
}

#[tokio::test]
async fn get_parties_puts_the_first_in_the_path_and_the_rest_in_the_query() {
    let body = format!(
        r#"{{"partyDetails":[{}]}}"#,
        party_json("alice::1220aa", "{}")
    );
    let (url, seen) = scripted_server(vec![(200, body)]).await;
    let client = JsonClient::new(url);

    let parties = client
        .get_parties(vec![
            "alice::1220aa".to_string(),
            "bob::1220bb".to_string(),
            "carol::1220cc".to_string(),
        ])
        .await
        .expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.path,
        "/v2/parties/alice%3A%3A1220aa?parties=bob%3A%3A1220bb&parties=carol%3A%3A1220cc"
    );
    // The participant answers only for the parties it knows; two unknown ones
    // are simply absent.
    assert_eq!(parties.len(), 1);
    assert_eq!(parties[0].party, "alice::1220aa");
}

#[tokio::test]
async fn get_parties_with_nothing_to_ask_about_never_reaches_the_wire() {
    let (url, seen) = scripted_server(vec![]).await;
    let client = JsonClient::new(url);

    let error = client.get_parties(vec![]).await.expect_err("refused");
    assert!(
        matches!(error, canton_ledger::Error::InvalidRequest(_)),
        "{error:?}"
    );

    let error = client
        .get_parties(vec!["alice::1220aa".to_string(), "not a party".to_string()])
        .await
        .expect_err("refused");
    assert!(
        matches!(error, canton_ledger::Error::InvalidRequest(_)),
        "{error:?}"
    );
    assert!(seen.lock().unwrap().is_empty(), "nothing was sent");
}

#[tokio::test]
async fn allocating_with_a_hint_posts_only_the_hint() {
    let body = format!(
        r#"{{"partyDetails":{}}}"#,
        party_json("alice::1220aa", "{}")
    );
    let (url, seen) = scripted_server(vec![(200, body)]).await;
    let client = JsonClient::new(url);

    let details = client.allocate_party(Some("alice")).await.expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(
        (request.method.as_str(), request.path.as_str()),
        ("POST", "/v2/parties")
    );
    assert_eq!(request.body, serde_json::json!({ "partyIdHint": "alice" }));
    assert_eq!(details.party, "alice::1220aa");
}

#[tokio::test]
async fn allocating_with_no_hint_posts_an_empty_object() {
    let body = format!(
        r#"{{"partyDetails":{}}}"#,
        party_json("party-1220aa::1220aa", "{}")
    );
    let (url, seen) = scripted_server(vec![(200, body)]).await;
    let client = JsonClient::new(url);

    client.allocate_party(None).await.expect("success");

    assert_eq!(seen.lock().unwrap()[0].body, serde_json::json!({}));
}

#[tokio::test]
async fn the_full_allocation_request_is_spelled_the_way_the_api_spells_it() {
    use canton_ledger::AllocateParty;

    let body = format!(
        r#"{{"partyDetails":{}}}"#,
        party_json("alice::1220aa", r#"{"team":"blue"}"#)
    );
    let (url, seen) = scripted_server(vec![(200, body)]).await;
    let client = JsonClient::new(url);

    let request = AllocateParty::new()
        .with_hint("alice")
        .with_annotation("team", "blue")
        .with_identity_provider_id("idp-1")
        .with_synchronizer_id("global::1220ff")
        .with_user_id("wallet-user");
    let details = client.allocate_party_with(&request).await.expect("success");

    assert_eq!(
        seen.lock().unwrap()[0].body,
        serde_json::json!({
            "partyIdHint": "alice",
            "localMetadata": { "annotations": { "team": "blue" } },
            "identityProviderId": "idp-1",
            "synchronizerId": "global::1220ff",
            "userId": "wallet-user",
        })
    );
    let meta = details.local_metadata.expect("metadata");
    assert_eq!(
        meta.annotations.get("team").map(String::as_str),
        Some("blue")
    );
}

#[tokio::test]
async fn updating_a_party_patches_its_path_with_the_details_and_the_mask() {
    use canton_ledger::proto::admin::{ObjectMeta, PartyDetails};

    let body = format!(
        r#"{{"partyDetails":{}}}"#,
        party_json("alice::1220aa", r#"{"team":"red"}"#)
    );
    let (url, seen) = scripted_server(vec![(200, body)]).await;
    let client = JsonClient::new(url);

    let details = PartyDetails {
        party: "alice::1220aa".to_string(),
        is_local: true,
        local_metadata: Some(ObjectMeta {
            resource_version: "3".to_string(),
            annotations: [("team".to_string(), "red".to_string())]
                .into_iter()
                .collect(),
        }),
        identity_provider_id: String::new(),
    };
    let updated = client
        .update_party_details(&details, &["local_metadata.annotations"])
        .await
        .expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(
        (request.method.as_str(), request.path.as_str()),
        ("PATCH", "/v2/parties/alice%3A%3A1220aa")
    );
    assert_eq!(
        request.body,
        serde_json::json!({
            "partyDetails": {
                "party": "alice::1220aa",
                "isLocal": true,
                "localMetadata": { "resourceVersion": "3", "annotations": { "team": "red" } },
            },
            "updateMask": { "paths": ["local_metadata.annotations"] },
        })
    );
    let meta = updated.local_metadata.expect("metadata");
    assert_eq!(
        meta.annotations.get("team").map(String::as_str),
        Some("red")
    );
}

#[tokio::test]
async fn an_empty_update_mask_is_refused_before_the_trip() {
    use canton_ledger::proto::admin::PartyDetails;

    let (url, seen) = scripted_server(vec![]).await;
    let client = JsonClient::new(url);
    let details = PartyDetails {
        party: "alice::1220aa".to_string(),
        ..Default::default()
    };

    let error = client
        .update_party_details(&details, &[])
        .await
        .expect_err("the participant would refuse it; this says so first");

    assert!(
        matches!(error, canton_ledger::Error::InvalidRequest(_)),
        "{error:?}"
    );
    assert!(seen.lock().unwrap().is_empty(), "nothing was sent");
}

#[tokio::test]
async fn a_refused_party_read_keeps_the_participants_status_and_body() {
    // What a plain user gets for a party it may not read: the participant's
    // security-sensitive 403, whose body is deliberately uninformative. The
    // status and the body both reach the caller unchanged.
    let (url, _seen) = scripted_server(vec![(
        403,
        r#"{"code":"NA","cause":"A security-sensitive error has been received"}"#.to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let error = client
        .get_parties(vec!["stranger::1220ff".to_string()])
        .await
        .expect_err("refused");

    match error {
        canton_ledger::Error::Http { status, body, .. } => {
            assert_eq!(status, 403);
            assert!(body.contains("security-sensitive"), "{body}");
        }
        other => panic!("expected Http, got {other:?}"),
    }
}

// ---- The reads and the wrapped submission ------------------------------------

#[tokio::test]
async fn submit_and_wait_for_transaction_wraps_the_commands_and_reads_the_transaction() {
    let (url, seen) = scripted_server(vec![(
        200,
        r#"{"transaction":{"updateId":"u-1","commandId":"c-1","offset":77,"events":[{"created":{}}]}}"#
            .to_string(),
    )])
    .await;
    let client = JsonClient::new(url);

    let response = client
        .submit_and_wait_for_transaction(
            &commands().with_min_ledger_time_rel(serde_json::json!("10s")),
        )
        .await
        .expect("success");

    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(
        (request.method.as_str(), request.path.as_str()),
        ("POST", "/v2/commands/submit-and-wait-for-transaction")
    );
    // This endpoint takes a request object *wrapping* the command set — the
    // other two take the set itself — and the relative ledger time rides along.
    assert_eq!(
        request.body["commands"]["actAs"],
        serde_json::json!(["alice"])
    );
    assert_eq!(request.body["commands"]["minLedgerTimeRel"], "10s");
    assert_eq!(response.transaction.update_id, "u-1");
    assert_eq!(response.transaction.offset, 77);
    assert_eq!(response.transaction.events.len(), 1);
}

#[tokio::test]
async fn the_bounded_reads_post_their_request_and_carry_the_limit_in_the_query() {
    let (url, seen) = scripted_server(vec![
        (200, r#"[{"contractEntry":{"id":"00a"}}]"#.to_string()),
        (
            200,
            r#"[{"contractEntry":{"id":"00b"}},{"contractEntry":{"id":"00c"}}]"#.to_string(),
        ),
        (
            200,
            r#"[{"update":{"offset":7}},{"update":{"offset":8}}]"#.to_string(),
        ),
        (200, r#"[{"update":{"offset":9}}]"#.to_string()),
    ])
    .await;
    let client = JsonClient::new(url);

    // Each read returns what the participant sent — the whole array, as sent.
    let acs = client
        .active_contracts(vec!["alice::1220ab".to_string()], 40, Some(10))
        .await
        .expect("success");
    assert_eq!(acs, vec![serde_json::json!({"contractEntry":{"id":"00a"}})]);

    let request = canton_ledger::ActiveContractsRequest::new(vec!["alice::1220ab".to_string()], 41);
    let acs_with = client
        .active_contracts_with(&request, None)
        .await
        .expect("success");
    assert_eq!(acs_with.len(), 2);
    assert_eq!(acs_with[1]["contractEntry"]["id"], "00c");

    let updates = client
        .updates(vec!["alice::1220ab".to_string()], 5, Some(9), Some(2))
        .await
        .expect("success");
    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0]["update"]["offset"], 7);

    let request = canton_ledger::UpdatesRequest::new(vec!["alice::1220ab".to_string()], 6);
    let updates_with = client.updates_with(&request, None).await.expect("success");
    assert_eq!(
        updates_with,
        vec![serde_json::json!({"update":{"offset":9}})]
    );

    let requests = seen.lock().unwrap().clone();
    let paths: Vec<&str> = requests.iter().map(|r| r.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "/v2/state/active-contracts?limit=10",
            "/v2/state/active-contracts",
            "/v2/updates?limit=2",
            "/v2/updates",
        ]
    );
    assert!(requests.iter().all(|r| r.method == "POST"));
    assert_eq!(requests[0].body["activeAtOffset"], 40);
    assert_eq!(requests[1].body["activeAtOffset"], 41);
    assert_eq!(requests[2].body["beginExclusive"], 5);
    assert_eq!(requests[2].body["endInclusive"], 9);
    assert_eq!(requests[3].body["beginExclusive"], 6);
    assert!(
        requests[3].body.get("endInclusive").is_none(),
        "no end means an unbounded read, not `null`: {}",
        requests[3].body
    );
}

#[tokio::test]
async fn ledger_end_is_the_offset_the_participant_reports() {
    let (url, seen) = scripted_server(vec![(200, r#"{"offset":4242}"#.to_string())]).await;
    let client = JsonClient::new(url);

    let end = client.ledger_end().await.expect("success");

    assert_eq!(end, 4242);
    let request = seen.lock().unwrap()[0].clone();
    assert_eq!(
        (request.method.as_str(), request.path.as_str()),
        ("GET", "/v2/state/ledger-end")
    );
}

/// The participant names a duplicate two ways — the 409 status, and
/// `DUPLICATE_COMMAND` in the body — and either alone is enough, because
/// proxies rewrite one and older nodes omit the other. Anything else on a
/// retry is the failure it says it is.
#[tokio::test]
async fn a_retry_refused_as_a_duplicate_by_either_signal_alone_is_a_success() {
    for (status, body) in [
        (409, r#"{"cause":"conflict"}"#),
        (
            400,
            r#"{"code":"DUPLICATE_COMMAND","cause":"command already exists"}"#,
        ),
    ] {
        let (url, seen) = scripted_server(vec![
            (503, r#"{"cause":"lost"}"#.to_string()),
            (status, body.to_string()),
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
            .unwrap_or_else(|e| panic!("{status} {body}: {e}"));
        assert_eq!(seen.lock().unwrap().len(), 2, "{status} {body}");
    }

    let (url, _seen) = scripted_server(vec![
        (503, r#"{"cause":"lost"}"#.to_string()),
        (500, r#"{"cause":"the participant fell over"}"#.to_string()),
    ])
    .await;
    let client = JsonClient::new(url).with_retry(
        canton_ledger::RetryConfig::default()
            .with_max_attempts(2)
            .with_initial_backoff(Duration::from_millis(1)),
    );
    let error = client
        .submit(&commands())
        .await
        .expect_err("a retry that failed for another reason is a failure");
    assert!(
        matches!(error, canton_ledger::Error::Http { status: 500, .. }),
        "{error:?}"
    );
}
