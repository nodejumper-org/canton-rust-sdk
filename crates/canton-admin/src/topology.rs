//! Canton admin-API client: topology **read**.
//!
//! Wraps `TopologyManagerReadService` (served on the Canton admin API port,
//! e.g. `:3902`) for the M1 read surface: party→participant mappings, namespace
//! delegations, and vetted packages.

use std::sync::Arc;

use canton_core::auth::{self, Intercepted};
use canton_core::telemetry::{self, TRANSPORT_GRPC};
use canton_core::{Config, Error, Result};
use canton_proto::com::digitalasset::canton::protocol::v30 as protocol;
use canton_proto::com::digitalasset::canton::topology::admin::v30 as topo;
use tonic::transport::Channel;
use topo::topology_manager_read_service_client::TopologyManagerReadServiceClient;

/// Which topology store to query.
///
/// `#[non_exhaustive]` so new store kinds can be added without a breaking
/// change.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Store {
    /// The node's authorized store (its own local topology state).
    Authorized,
    /// A synchronizer store, addressed by its physical synchronizer id.
    Synchronizer(String),
    /// A temporary store, by name.
    Temporary(String),
}

impl Store {
    fn to_store_id(&self) -> topo::StoreId {
        use topo::store_id::Store as S;
        let store = match self {
            Store::Authorized => S::Authorized(topo::store_id::Authorized {}),
            Store::Synchronizer(physical_id) => S::Synchronizer(topo::Synchronizer {
                kind: Some(topo::synchronizer::Kind::PhysicalId(physical_id.clone())),
            }),
            Store::Temporary(name) => {
                S::Temporary(topo::store_id::Temporary { name: name.clone() })
            }
        };
        topo::StoreId { store: Some(store) }
    }
}

/// One topology mapping result: the mapping `item` plus its
/// [`BaseResult`](topo::BaseResult) `context` (store, validity window, serial,
/// …). `#[non_exhaustive]` so fields can be added without a breaking change.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Entry<T> {
    /// Topology metadata for this mapping (store, validity window, serial, …).
    pub context: topo::BaseResult,
    /// The mapping item.
    pub item: T,
}

/// A client for Canton's `TopologyManagerReadService` (admin API).
///
/// On LocalNet the admin API is unauthenticated; in hardened deployments set a
/// bearer token via [`Config::with_oidc`](canton_core::Config::with_oidc).
#[derive(Clone, Debug)]
pub struct TopologyClient {
    channel: Channel,
    config: Arc<Config>,
}

/// Build a gRPC service client on the authenticated channel, with this
/// client's decode limit applied.
///
/// A macro rather than a function because `max_decoding_message_size` is an
/// inherent method on each generated client — tonic exposes no trait for it —
/// so there is nothing to be generic over. Keeping every construction site
/// behind one expansion is the point: `tonic`'s 4 MiB default is small enough
/// that a real ACS page trips it, and a new RPC added later would otherwise
/// pick the default up silently.
macro_rules! service {
    ($self:ident, $ctor:expr) => {
        $ctor($self.intercepted().await?)
            .max_decoding_message_size($self.config.max_decoding_message_size())
    };
}

impl TopologyClient {
    /// Build a lazily-connected client for the admin API endpoint (e.g.
    /// `http://localhost:3902`).
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`](canton_core::Error::InvalidRequest) if
    /// the endpoint is not a valid URI.
    pub fn connect_lazy(config: Config) -> Result<Self> {
        Ok(Self {
            channel: config.connect_channel()?,
            config: Arc::new(config),
        })
    }

    async fn intercepted(&self) -> Result<Intercepted> {
        auth::intercepted(&self.channel, self.config.auth()).await
    }

    async fn with_retry<T, F, Fut>(&self, op: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        canton_core::retry::run_with_retry(self.config.retry(), op).await
    }

    /// A `BaseQuery` for the current (head) state of `store`, no proposals.
    fn base_query(store: &Store) -> topo::BaseQuery {
        topo::BaseQuery {
            store: Some(store.to_store_id()),
            proposals: false,
            operation: 0, // TOPOLOGY_CHANGE_OP_UNSPECIFIED
            time_query: Some(topo::base_query::TimeQuery::HeadState(())),
            filter_signed_key: String::new(),
            protocol_version: None,
        }
    }

    /// Read party→participant mappings (`ListPartyToParticipant`). Empty filters
    /// match all.
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if the RPC fails.
    pub async fn list_party_to_participant(
        &self,
        store: Store,
        filter_party: &str,
        filter_participant: &str,
    ) -> Result<Vec<Entry<protocol::PartyToParticipant>>> {
        let filter_party = filter_party.to_string();
        let filter_participant = filter_participant.to_string();
        telemetry::instrument("list_party_to_participant", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let (store, filter_party, filter_participant) = (
                    store.clone(),
                    filter_party.clone(),
                    filter_participant.clone(),
                );
                async move {
                    let mut client = service!(self, TopologyManagerReadServiceClient::new);
                    let response = client
                        .list_party_to_participant(topo::ListPartyToParticipantRequest {
                            base_query: Some(Self::base_query(&store)),
                            filter_party,
                            filter_participant,
                        })
                        .await?
                        .into_inner();
                    collect_entries(response.results.into_iter().map(|r| (r.context, r.item)))
                }
            })
            .await
        })
        .await
    }

    /// Read namespace delegations (`ListNamespaceDelegation`). Empty filters
    /// match all.
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if the RPC fails.
    pub async fn list_namespace_delegations(
        &self,
        store: Store,
        filter_namespace: &str,
        filter_target_key_fingerprint: &str,
    ) -> Result<Vec<Entry<protocol::NamespaceDelegation>>> {
        let filter_namespace = filter_namespace.to_string();
        let filter_target_key_fingerprint = filter_target_key_fingerprint.to_string();
        telemetry::instrument("list_namespace_delegations", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let (store, filter_namespace, filter_target_key_fingerprint) = (
                    store.clone(),
                    filter_namespace.clone(),
                    filter_target_key_fingerprint.clone(),
                );
                async move {
                    let mut client = service!(self, TopologyManagerReadServiceClient::new);
                    let response = client
                        .list_namespace_delegation(topo::ListNamespaceDelegationRequest {
                            base_query: Some(Self::base_query(&store)),
                            filter_namespace,
                            filter_target_key_fingerprint,
                        })
                        .await?
                        .into_inner();
                    collect_entries(response.results.into_iter().map(|r| (r.context, r.item)))
                }
            })
            .await
        })
        .await
    }

    /// Read vetted packages (`ListVettedPackages`). Empty participant filter
    /// matches all.
    ///
    /// Vetted packages are stored per-synchronizer: query with
    /// [`Store::Synchronizer`] (the synchronizer's physical id) rather than
    /// [`Store::Authorized`], or the result is empty.
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if the RPC fails.
    pub async fn list_vetted_packages(
        &self,
        store: Store,
        filter_participant: &str,
    ) -> Result<Vec<Entry<protocol::VettedPackages>>> {
        let filter_participant = filter_participant.to_string();
        telemetry::instrument("list_vetted_packages", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let (store, filter_participant) = (store.clone(), filter_participant.clone());
                async move {
                    let mut client = service!(self, TopologyManagerReadServiceClient::new);
                    let response = client
                        .list_vetted_packages(topo::ListVettedPackagesRequest {
                            base_query: Some(Self::base_query(&store)),
                            filter_participant,
                        })
                        .await?
                        .into_inner();
                    collect_entries(response.results.into_iter().map(|r| (r.context, r.item)))
                }
            })
            .await
        })
        .await
    }
}

/// Keep only results that carry both a context and an item, pairing them.
/// Assemble the entries of a topology response, refusing an incomplete one.
///
/// Both halves of an entry are `Required` in the proto, so a row missing
/// either is a participant that answered something this client cannot read.
/// Dropping such a row silently is the dangerous shape: topology reads answer
/// questions like "which participants host this party", and a short answer is
/// indistinguishable from a true one — the caller acts on a view of the
/// network that is missing a member it was never told about.
fn collect_entries<T>(
    results: impl Iterator<Item = (Option<topo::BaseResult>, Option<T>)>,
) -> Result<Vec<Entry<T>>> {
    results
        .map(|(context, item)| match (context, item) {
            (Some(context), Some(item)) => Ok(Entry { context, item }),
            (None, _) => Err(Error::UnexpectedResponse(
                "a topology result carried no context; the response is incomplete".to_string(),
            )),
            (_, None) => Err(Error::UnexpectedResponse(
                "a topology result carried no mapping; the response is incomplete".to_string(),
            )),
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn store_maps_to_the_right_store_id_variant() {
        use topo::store_id::Store as S;

        assert!(matches!(
            Store::Authorized.to_store_id().store,
            Some(S::Authorized(_))
        ));

        match Store::Synchronizer("pid-1".to_string()).to_store_id().store {
            Some(S::Synchronizer(s)) => assert_eq!(
                s.kind,
                Some(topo::synchronizer::Kind::PhysicalId("pid-1".to_string()))
            ),
            other => panic!("expected a synchronizer store, got {other:?}"),
        }

        match Store::Temporary("scratch".to_string()).to_store_id().store {
            Some(S::Temporary(t)) => assert_eq!(t.name, "scratch"),
            other => panic!("expected a temporary store, got {other:?}"),
        }
    }

    #[test]
    fn base_query_asks_for_head_state_without_proposals() {
        let query = TopologyClient::base_query(&Store::Authorized);
        assert!(matches!(
            query.time_query,
            Some(topo::base_query::TimeQuery::HeadState(()))
        ));
        assert!(!query.proposals, "reads default to committed state");
        assert!(query.store.is_some());
    }

    #[test]
    fn collect_entries_refuses_partial_results() {
        let ctx = topo::BaseResult::default();

        // A complete response reads normally.
        let rows = vec![
            (Some(ctx.clone()), Some(7u8)),
            (Some(ctx.clone()), Some(8u8)),
        ];
        let kept = collect_entries(rows.into_iter()).expect("a complete response");
        assert_eq!(kept.len(), 2);

        // A row missing either half fails the whole read rather than shrinking
        // it: a topology answer that is quietly short is worse than none.
        let missing_context = vec![(Some(ctx.clone()), Some(7u8)), (None, Some(8u8))];
        assert!(matches!(
            collect_entries(missing_context.into_iter()),
            Err(Error::UnexpectedResponse(_))
        ));
        let missing_item = vec![(Some(ctx.clone()), Some(7u8)), (Some(ctx), None::<u8>)];
        assert!(matches!(
            collect_entries(missing_item.into_iter()),
            Err(Error::UnexpectedResponse(_))
        ));
    }
}
