//! In-process tests that need a controllable server but no live node.
//!
//! Uses the dev-only `canton-proto/server` feature to stand up mock
//! `PartyManagementService` / `UserManagementService` servers, exercising the
//! client logic (pagination, not-retried allocation, error mapping, self-inspect
//! filtering) that the live tests only smoke-check.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use canton_admin::{AdminClient, Config, RetryConfig};
use canton_proto::com::daml::ledger::api::v2::admin as pb;
use pb::party_management_service_server::{PartyManagementService, PartyManagementServiceServer};
use pb::user_management_service_server::{UserManagementService, UserManagementServiceServer};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// ---- PartyManagementService mock -------------------------------------------

/// A configurable `PartyManagementService`:
/// - `list_known_parties` serves five parties across three token-keyed pages
///   (2 + 2 + 1), or `PermissionDenied` when `deny` is set;
/// - `allocate_party` counts calls and fails with `Unavailable` (retriable)
///   when `allocate_fails` is set, to prove the client does not retry it;
/// - `get_parties` echoes requested parties that start with `party-`.
#[derive(Clone, Default)]
struct MockParty {
    allocate_calls: Arc<AtomicUsize>,
    allocate_fails: bool,
    deny: bool,
}

fn party(n: usize) -> pb::PartyDetails {
    pb::PartyDetails {
        party: format!("party-{n}::fp"),
        is_local: true,
        ..Default::default()
    }
}

#[tonic::async_trait]
impl PartyManagementService for MockParty {
    async fn get_participant_id(
        &self,
        _r: Request<pb::GetParticipantIdRequest>,
    ) -> Result<Response<pb::GetParticipantIdResponse>, Status> {
        Ok(Response::new(pb::GetParticipantIdResponse {
            participant_id: "participant::mock".to_string(),
        }))
    }

    async fn list_known_parties(
        &self,
        request: Request<pb::ListKnownPartiesRequest>,
    ) -> Result<Response<pb::ListKnownPartiesResponse>, Status> {
        if self.deny {
            return Err(Status::permission_denied("needs ParticipantAdmin"));
        }
        let (party_details, next_page_token) = match request.into_inner().page_token.as_str() {
            "" => (vec![party(0), party(1)], "p1".to_string()),
            "p1" => (vec![party(2), party(3)], "p2".to_string()),
            "p2" => (vec![party(4)], String::new()),
            other => return Err(Status::invalid_argument(format!("bad token {other}"))),
        };
        Ok(Response::new(pb::ListKnownPartiesResponse {
            party_details,
            next_page_token,
        }))
    }

    async fn get_parties(
        &self,
        request: Request<pb::GetPartiesRequest>,
    ) -> Result<Response<pb::GetPartiesResponse>, Status> {
        // Echo only the parties this participant "knows" (drop the rest).
        let party_details = request
            .into_inner()
            .parties
            .into_iter()
            .filter(|p| p.starts_with("party-"))
            .map(|party| pb::PartyDetails {
                party,
                is_local: true,
                ..Default::default()
            })
            .collect();
        Ok(Response::new(pb::GetPartiesResponse { party_details }))
    }

    async fn allocate_party(
        &self,
        request: Request<pb::AllocatePartyRequest>,
    ) -> Result<Response<pb::AllocatePartyResponse>, Status> {
        self.allocate_calls.fetch_add(1, Ordering::SeqCst);
        if self.allocate_fails {
            return Err(Status::unavailable("try again"));
        }
        let hint = request.into_inner().party_id_hint;
        Ok(Response::new(pb::AllocatePartyResponse {
            party_details: Some(pb::PartyDetails {
                party: format!("{hint}::fp"),
                is_local: true,
                ..Default::default()
            }),
        }))
    }

    async fn allocate_external_party(
        &self,
        _r: Request<pb::AllocateExternalPartyRequest>,
    ) -> Result<Response<pb::AllocateExternalPartyResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn generate_external_party_topology(
        &self,
        _r: Request<pb::GenerateExternalPartyTopologyRequest>,
    ) -> Result<Response<pb::GenerateExternalPartyTopologyResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn update_party_details(
        &self,
        _r: Request<pb::UpdatePartyDetailsRequest>,
    ) -> Result<Response<pb::UpdatePartyDetailsResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn update_party_identity_provider_id(
        &self,
        _r: Request<pb::UpdatePartyIdentityProviderIdRequest>,
    ) -> Result<Response<pb::UpdatePartyIdentityProviderIdResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
}

async fn start_party_server(mock: MockParty) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(PartyManagementServiceServer::new(mock), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn participant_id_round_trips() {
    let url = start_party_server(MockParty::default()).await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();
    assert_eq!(client.participant_id().await.unwrap(), "participant::mock");
}

#[tokio::test]
async fn list_known_parties_follows_pagination() {
    let url = start_party_server(MockParty::default()).await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();

    let all = client.list_known_parties().await.unwrap();
    let names: Vec<_> = all.into_iter().map(|p| p.party).collect();
    assert_eq!(
        names,
        vec![
            "party-0::fp",
            "party-1::fp",
            "party-2::fp",
            "party-3::fp",
            "party-4::fp",
        ],
        "every page should be collected in order"
    );
}

#[tokio::test]
async fn list_known_parties_page_threads_token_and_ends_with_none() {
    let url = start_party_server(MockParty::default()).await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();

    // First page carries a continuation token…
    let (page, next) = client.list_known_parties_page(0, None).await.unwrap();
    assert_eq!(page.len(), 2);
    assert_eq!(next.as_deref(), Some("p1"));

    // …and the final page returns no token.
    let (last, done) = client
        .list_known_parties_page(0, Some("p2".to_string()))
        .await
        .unwrap();
    assert_eq!(last.len(), 1);
    assert_eq!(done, None, "the last page must not carry a token");
}

#[tokio::test]
async fn allocate_party_is_not_retried() {
    // Retry is ENABLED, but allocation must still be attempted exactly once:
    // a non-idempotent topology mutation must not be replayed on a lost response.
    let calls = Arc::new(AtomicUsize::new(0));
    let url = start_party_server(MockParty {
        allocate_calls: calls.clone(),
        allocate_fails: true,
        ..Default::default()
    })
    .await;
    let client = AdminClient::connect_lazy(
        Config::new(url).with_retry(
            RetryConfig::default()
                .with_max_attempts(5)
                .with_initial_backoff(Duration::from_millis(1))
                .with_max_backoff(Duration::from_millis(1)),
        ),
    )
    .unwrap();

    let result = client.allocate_party(Some("sdk")).await;
    assert!(result.is_err(), "the transient failure should surface");
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "allocate_party must be attempted exactly once even with retry enabled"
    );
}

#[tokio::test]
async fn get_parties_drops_unknown_and_handles_empty() {
    let url = start_party_server(MockParty::default()).await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();

    let got = client
        .get_parties(vec!["party-1::fp".to_string(), "unknown::0".to_string()])
        .await
        .unwrap();
    let names: Vec<_> = got.into_iter().map(|p| p.party).collect();
    assert_eq!(names, vec!["party-1::fp"], "unknown parties are dropped");

    assert!(
        client.get_parties(vec![]).await.unwrap().is_empty(),
        "empty input returns an empty list, not an error"
    );
}

#[tokio::test]
async fn permission_denied_surfaces_as_a_non_retriable_status() {
    let url = start_party_server(MockParty {
        deny: true,
        ..Default::default()
    })
    .await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();

    let error = client
        .list_known_parties()
        .await
        .expect_err("an under-privileged token must be rejected");
    assert_eq!(error.code(), Some(tonic::Code::PermissionDenied));
    assert!(
        !error.is_retriable(),
        "PermissionDenied must not be retriable"
    );
}

// ---- UserManagementService mock (self-inspect) -----------------------------

/// A `UserManagementService` whose `get_user` echoes the requested id (or the
/// synthetic `self-user` for the empty/self id, and `None` for `"missing"`),
/// and whose `list_user_rights` returns a deliberate mix of right kinds.
#[derive(Clone, Default)]
struct MockUsers;

fn right(kind: pb::right::Kind) -> pb::Right {
    pb::Right { kind: Some(kind) }
}

#[tonic::async_trait]
impl UserManagementService for MockUsers {
    async fn get_user(
        &self,
        request: Request<pb::GetUserRequest>,
    ) -> Result<Response<pb::GetUserResponse>, Status> {
        let user_id = request.into_inner().user_id;
        let user = match user_id.as_str() {
            "missing" => None,
            "" => Some(pb::User {
                id: "self-user".to_string(),
                ..Default::default()
            }),
            id => Some(pb::User {
                id: id.to_string(),
                ..Default::default()
            }),
        };
        Ok(Response::new(pb::GetUserResponse { user }))
    }

    async fn list_user_rights(
        &self,
        _r: Request<pb::ListUserRightsRequest>,
    ) -> Result<Response<pb::ListUserRightsResponse>, Status> {
        use pb::right::{CanActAs, CanActAsAnyParty, CanReadAs, Kind, ParticipantAdmin};
        Ok(Response::new(pb::ListUserRightsResponse {
            rights: vec![
                right(Kind::CanActAs(CanActAs {
                    party: "p1".to_string(),
                })),
                right(Kind::CanReadAs(CanReadAs {
                    party: "p2".to_string(),
                })),
                right(Kind::ParticipantAdmin(ParticipantAdmin {})),
                right(Kind::CanActAsAnyParty(CanActAsAnyParty {})),
            ],
        }))
    }

    async fn create_user(
        &self,
        _r: Request<pb::CreateUserRequest>,
    ) -> Result<Response<pb::CreateUserResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn update_user(
        &self,
        _r: Request<pb::UpdateUserRequest>,
    ) -> Result<Response<pb::UpdateUserResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn delete_user(
        &self,
        _r: Request<pb::DeleteUserRequest>,
    ) -> Result<Response<pb::DeleteUserResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_users(
        &self,
        _r: Request<pb::ListUsersRequest>,
    ) -> Result<Response<pb::ListUsersResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn grant_user_rights(
        &self,
        _r: Request<pb::GrantUserRightsRequest>,
    ) -> Result<Response<pb::GrantUserRightsResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn revoke_user_rights(
        &self,
        _r: Request<pb::RevokeUserRightsRequest>,
    ) -> Result<Response<pb::RevokeUserRightsResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn update_user_identity_provider_id(
        &self,
        _r: Request<pb::UpdateUserIdentityProviderIdRequest>,
    ) -> Result<Response<pb::UpdateUserIdentityProviderIdResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
}

async fn start_user_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(UserManagementServiceServer::new(MockUsers), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn current_user_and_get_user_by_id() {
    let url = start_user_server().await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();

    // current_user sends an empty id → the authenticated ("self") user.
    assert_eq!(client.current_user().await.unwrap().id, "self-user");
    // get_user threads the requested id through.
    assert_eq!(client.get_user("alice").await.unwrap().id, "alice");
}

#[tokio::test]
async fn get_user_with_no_user_is_an_unexpected_response() {
    let url = start_user_server().await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();

    let error = client
        .get_user("missing")
        .await
        .expect_err("a response with no user must error");
    assert!(
        matches!(error, canton_admin::Error::UnexpectedResponse(_)),
        "got {error:?}"
    );
}

#[tokio::test]
async fn acting_parties_keeps_only_can_act_as() {
    let url = start_user_server().await;
    let client = AdminClient::connect_lazy(Config::new(url)).unwrap();

    // Of {CanActAs(p1), CanReadAs(p2), ParticipantAdmin, CanActAsAnyParty},
    // only the specific CanActAs(p1) is returned.
    let acting = client.acting_parties().await.unwrap();
    assert_eq!(
        acting,
        vec!["p1"],
        "read/admin/any-party rights are excluded"
    );

    // And the full rights list is available for callers that need the wildcard.
    assert_eq!(client.current_user_rights().await.unwrap().len(), 4);
}
