//! Ledger-API admin client: party management + user self-inspect.

use std::sync::Arc;

use canton_core::auth::{self, Intercepted};
use canton_core::telemetry::{self, TRANSPORT_GRPC};
use canton_core::{Config, Error, Result};
use canton_proto::com::daml::ledger::api::v2 as lapi;
use canton_proto::com::daml::ledger::api::v2::admin as pb;
use lapi::package_service_client::PackageServiceClient;
use pb::party_management_service_client::PartyManagementServiceClient;
use pb::user_management_service_client::UserManagementServiceClient;
use tonic::transport::Channel;

/// A client for Canton's Ledger-API **admin** services: `PartyManagementService`
/// and `UserManagementService`.
///
/// Served on the Ledger API port (e.g. `:3901`) — the same endpoint and auth as
/// [`canton_ledger`]. Party-management RPCs require the `ParticipantAdmin`
/// right; the `current_user*` self-inspect calls do not.
///
/// [`canton_ledger`]: https://docs.rs/canton-ledger
#[derive(Clone, Debug)]
pub struct AdminClient {
    channel: Channel,
    config: Arc<Config>,
}

impl AdminClient {
    /// Build a lazily-connected client. Returns immediately; the TCP/TLS
    /// handshake happens on the first RPC.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] if the endpoint is not a valid URI.
    pub fn connect_lazy(config: Config) -> Result<Self> {
        Ok(Self {
            channel: config.connect_channel()?,
            config: Arc::new(config),
        })
    }

    /// A channel wrapped with a fresh bearer-token interceptor for this call.
    async fn intercepted(&self) -> Result<Intercepted> {
        auth::intercepted(&self.channel, self.config.auth()).await
    }

    /// Run a unary operation under the configured retry policy (no-op when
    /// retrying is not enabled).
    async fn with_retry<T, F, Fut>(&self, op: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        canton_core::retry::run_with_retry(self.config.retry(), op).await
    }

    // ---- PartyManagementService --------------------------------------------

    /// The participant's id (`GetParticipantId`). Works with any authenticated
    /// token.
    ///
    /// # Errors
    /// Returns an [`Error`] if the RPC fails.
    pub async fn participant_id(&self) -> Result<String> {
        telemetry::instrument("participant_id", TRANSPORT_GRPC, async {
            self.with_retry(|| async {
                let mut client = PartyManagementServiceClient::new(self.intercepted().await?);
                Ok(client
                    .get_participant_id(pb::GetParticipantIdRequest {})
                    .await?
                    .into_inner()
                    .participant_id)
            })
            .await
        })
        .await
    }

    /// Allocate a new party (`AllocateParty`). `party_id_hint` suggests the
    /// local part of the party id; the participant returns the actual
    /// [`PartyDetails`](pb::PartyDetails) (`party = "<hint>::<fingerprint>"`).
    ///
    /// Requires the `ParticipantAdmin` right. Not retried: party allocation is
    /// a non-idempotent topology mutation (a lost response must not double
    /// allocate), so a transient failure surfaces to the caller.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication/authorization fails, the RPC
    /// fails, or the response carries no party details.
    pub async fn allocate_party(&self, party_id_hint: Option<&str>) -> Result<pb::PartyDetails> {
        let party_id_hint = party_id_hint.unwrap_or_default().to_string();
        telemetry::instrument("allocate_party", TRANSPORT_GRPC, async move {
            let mut client = PartyManagementServiceClient::new(self.intercepted().await?);
            let response = client
                .allocate_party(pb::AllocatePartyRequest {
                    party_id_hint,
                    ..Default::default()
                })
                .await?
                .into_inner();
            response.party_details.ok_or_else(|| {
                Error::UnexpectedResponse("allocate_party returned no party details".to_string())
            })
        })
        .await
    }

    /// List one page of known parties (`ListKnownParties`). Pass the returned
    /// token to fetch the next page; `page_size` `0` uses the server default.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication/authorization or the RPC fails.
    pub async fn list_known_parties_page(
        &self,
        page_size: i32,
        page_token: Option<String>,
    ) -> Result<(Vec<pb::PartyDetails>, Option<String>)> {
        telemetry::instrument("list_known_parties_page", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let page_token = page_token.clone().unwrap_or_default();
                async move {
                    let mut client = PartyManagementServiceClient::new(self.intercepted().await?);
                    let response = client
                        .list_known_parties(pb::ListKnownPartiesRequest {
                            page_token,
                            page_size,
                            ..Default::default()
                        })
                        .await?
                        .into_inner();
                    let next =
                        (!response.next_page_token.is_empty()).then_some(response.next_page_token);
                    Ok((response.party_details, next))
                }
            })
            .await
        })
        .await
    }

    /// All known parties, following pagination internally (`ListKnownParties`).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication/authorization or any page RPC
    /// fails.
    pub async fn list_known_parties(&self) -> Result<Vec<pb::PartyDetails>> {
        telemetry::instrument("list_known_parties", TRANSPORT_GRPC, async {
            let mut all = Vec::new();
            let mut page_token = String::new();
            loop {
                let sent = page_token.clone();
                let (parties, next) = self
                    .with_retry(|| {
                        let page_token = sent.clone();
                        async move {
                            let mut client =
                                PartyManagementServiceClient::new(self.intercepted().await?);
                            let response = client
                                .list_known_parties(pb::ListKnownPartiesRequest {
                                    page_token,
                                    ..Default::default()
                                })
                                .await?
                                .into_inner();
                            Ok((response.party_details, response.next_page_token))
                        }
                    })
                    .await?;
                all.extend(parties);
                if next.is_empty() {
                    break;
                }
                // Guard against a server that never advances the token: without
                // this a degenerate/buggy participant would loop forever.
                if next == sent {
                    tracing::warn!(
                        "list_known_parties: server returned an unchanged page token; \
                         stopping pagination with {} parties collected",
                        all.len()
                    );
                    break;
                }
                page_token = next;
            }
            Ok(all)
        })
        .await
    }

    /// Details for specific parties (`GetParties`). Unknown parties are silently
    /// omitted from the result rather than erroring.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication/authorization or the RPC fails.
    pub async fn get_parties(&self, parties: Vec<String>) -> Result<Vec<pb::PartyDetails>> {
        telemetry::instrument("get_parties", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let parties = parties.clone();
                async move {
                    let mut client = PartyManagementServiceClient::new(self.intercepted().await?);
                    Ok(client
                        .get_parties(pb::GetPartiesRequest {
                            parties,
                            ..Default::default()
                        })
                        .await?
                        .into_inner()
                        .party_details)
                }
            })
            .await
        })
        .await
    }

    // ---- UserManagementService (self-inspect) ------------------------------

    /// The authenticated user's own record (`GetUser` with an empty user id).
    /// Authorized for any authenticated token (no admin right needed).
    ///
    /// # Errors
    /// Returns an [`Error`] if the RPC fails or the response carries no user.
    pub async fn current_user(&self) -> Result<pb::User> {
        self.get_user("").await
    }

    /// A user's record by id (`GetUser`); an empty `user_id` means the
    /// authenticated user.
    ///
    /// # Errors
    /// Returns an [`Error`] if the RPC fails or the response carries no user.
    pub async fn get_user(&self, user_id: &str) -> Result<pb::User> {
        let user_id = user_id.to_string();
        telemetry::instrument("get_user", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let user_id = user_id.clone();
                async move {
                    let mut client = UserManagementServiceClient::new(self.intercepted().await?);
                    let response = client
                        .get_user(pb::GetUserRequest {
                            user_id,
                            ..Default::default()
                        })
                        .await?
                        .into_inner();
                    response.user.ok_or_else(|| {
                        Error::UnexpectedResponse("get_user returned no user".to_string())
                    })
                }
            })
            .await
        })
        .await
    }

    /// The authenticated user's own rights (`ListUserRights` with an empty user
    /// id). Authorized for any authenticated token.
    ///
    /// # Errors
    /// Returns an [`Error`] if the RPC fails.
    pub async fn current_user_rights(&self) -> Result<Vec<pb::Right>> {
        self.list_user_rights("").await
    }

    /// A user's rights by id (`ListUserRights`); an empty `user_id` means the
    /// authenticated user.
    ///
    /// # Errors
    /// Returns an [`Error`] if the RPC fails.
    pub async fn list_user_rights(&self, user_id: &str) -> Result<Vec<pb::Right>> {
        let user_id = user_id.to_string();
        telemetry::instrument("list_user_rights", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let user_id = user_id.clone();
                async move {
                    let mut client = UserManagementServiceClient::new(self.intercepted().await?);
                    Ok(client
                        .list_user_rights(pb::ListUserRightsRequest {
                            user_id,
                            ..Default::default()
                        })
                        .await?
                        .into_inner()
                        .rights)
                }
            })
            .await
        })
        .await
    }

    // ---- PackageService (read) ----------------------------------------------

    /// The ids of all Daml packages known to the participant
    /// (`PackageService.ListPackages`).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn list_packages(&self) -> Result<Vec<String>> {
        telemetry::instrument("list_packages", TRANSPORT_GRPC, async {
            self.with_retry(|| async {
                let mut client = PackageServiceClient::new(self.intercepted().await?);
                Ok(client
                    .list_packages(lapi::ListPackagesRequest {})
                    .await?
                    .into_inner()
                    .package_ids)
            })
            .await
        })
        .await
    }

    /// The status of a package on the participant
    /// (`PackageService.GetPackageStatus`): `Registered` if the package is
    /// known, `Unspecified` otherwise.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn get_package_status(&self, package_id: &str) -> Result<lapi::PackageStatus> {
        let package_id = package_id.to_string();
        telemetry::instrument("get_package_status", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let package_id = package_id.clone();
                async move {
                    let mut client = PackageServiceClient::new(self.intercepted().await?);
                    let response = client
                        .get_package_status(lapi::GetPackageStatusRequest { package_id })
                        .await?
                        .into_inner();
                    Ok(response.package_status())
                }
            })
            .await
        })
        .await
    }

    /// The specific parties the authenticated user may act as — the `CanActAs`
    /// subset of [`Self::current_user_rights`]. Supersedes the ad-hoc party
    /// discovery used in early tests.
    ///
    /// Note: a user granted the wildcard `CanActAsAnyParty` right may act as
    /// *any* party yet carries no explicit `CanActAs` entries, so this returns
    /// an empty list for such users — an empty result means "no specific
    /// parties", not necessarily "cannot act". Inspect
    /// [`Self::current_user_rights`] directly if you need the wildcard.
    ///
    /// # Errors
    /// Returns an [`Error`] if the underlying rights RPC fails.
    pub async fn acting_parties(&self) -> Result<Vec<String>> {
        let rights = self.current_user_rights().await?;
        Ok(rights
            .into_iter()
            .filter_map(|right| match right.kind {
                Some(pb::right::Kind::CanActAs(can)) => Some(can.party),
                _ => None,
            })
            .collect())
    }
}
