//! The topology read client against an in-process server.
//!
//! The three public list methods were covered only by the env-gated live
//! suite, which CI never runs — the exact shape of gap the M1 review called
//! out on live tests generally, found here by measuring coverage rather than
//! reading it. These pin the request each method sends (store, filters,
//! head-state time query) and both halves of the response contract: a complete
//! answer maps into `Entry`s, and a partial row fails the whole read.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_admin::Config;
use canton_admin::{Store, TopologyClient};
use canton_proto::com::digitalasset::canton::protocol::v30 as protocol;
use canton_proto::com::digitalasset::canton::topology::admin::v30 as topo;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use topo::topology_manager_read_service_server::{
    TopologyManagerReadService, TopologyManagerReadServiceServer,
};

type Empty<T> = Pin<Box<dyn tokio_stream::Stream<Item = Result<T, Status>> + Send>>;

fn context() -> topo::BaseResult {
    topo::BaseResult {
        serial: 7,
        ..Default::default()
    }
}

/// Serves one row per service; `partial` drops the mapping from the party row.
/// Records the party-list request so the test can pin what the client asks.
#[derive(Clone, Default)]
struct MockTopology {
    partial: bool,
    party_requests: Arc<Mutex<Vec<topo::ListPartyToParticipantRequest>>>,
}

#[tonic::async_trait]
impl TopologyManagerReadService for MockTopology {
    async fn list_party_to_participant(
        &self,
        request: Request<topo::ListPartyToParticipantRequest>,
    ) -> Result<Response<topo::ListPartyToParticipantResponse>, Status> {
        self.party_requests
            .lock()
            .unwrap()
            .push(request.into_inner());
        let item = (!self.partial).then(|| protocol::PartyToParticipant {
            party: "alice::fp".to_string(),
            ..Default::default()
        });
        Ok(Response::new(topo::ListPartyToParticipantResponse {
            results: vec![topo::list_party_to_participant_response::Result {
                context: Some(context()),
                item,
            }],
        }))
    }

    async fn list_namespace_delegation(
        &self,
        _r: Request<topo::ListNamespaceDelegationRequest>,
    ) -> Result<Response<topo::ListNamespaceDelegationResponse>, Status> {
        Ok(Response::new(topo::ListNamespaceDelegationResponse {
            results: vec![topo::list_namespace_delegation_response::Result {
                context: Some(context()),
                item: Some(protocol::NamespaceDelegation::default()),
            }],
        }))
    }

    async fn list_vetted_packages(
        &self,
        _r: Request<topo::ListVettedPackagesRequest>,
    ) -> Result<Response<topo::ListVettedPackagesResponse>, Status> {
        Ok(Response::new(topo::ListVettedPackagesResponse {
            results: vec![topo::list_vetted_packages_response::Result {
                context: Some(context()),
                item: Some(protocol::VettedPackages::default()),
            }],
        }))
    }

    async fn list_decentralized_namespace_definition(
        &self,
        _r: Request<topo::ListDecentralizedNamespaceDefinitionRequest>,
    ) -> Result<Response<topo::ListDecentralizedNamespaceDefinitionResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_owner_to_key_mapping(
        &self,
        _r: Request<topo::ListOwnerToKeyMappingRequest>,
    ) -> Result<Response<topo::ListOwnerToKeyMappingResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_party_to_key_mapping(
        &self,
        _r: Request<topo::ListPartyToKeyMappingRequest>,
    ) -> Result<Response<topo::ListPartyToKeyMappingResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_synchronizer_trust_certificate(
        &self,
        _r: Request<topo::ListSynchronizerTrustCertificateRequest>,
    ) -> Result<Response<topo::ListSynchronizerTrustCertificateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_participant_synchronizer_permission(
        &self,
        _r: Request<topo::ListParticipantSynchronizerPermissionRequest>,
    ) -> Result<Response<topo::ListParticipantSynchronizerPermissionResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_party_hosting_limits(
        &self,
        _r: Request<topo::ListPartyHostingLimitsRequest>,
    ) -> Result<Response<topo::ListPartyHostingLimitsResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_synchronizer_parameters_state(
        &self,
        _r: Request<topo::ListSynchronizerParametersStateRequest>,
    ) -> Result<Response<topo::ListSynchronizerParametersStateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_sequencing_parameters_state(
        &self,
        _r: Request<topo::ListSequencingParametersStateRequest>,
    ) -> Result<Response<topo::ListSequencingParametersStateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_mediator_synchronizer_state(
        &self,
        _r: Request<topo::ListMediatorSynchronizerStateRequest>,
    ) -> Result<Response<topo::ListMediatorSynchronizerStateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_sequencer_synchronizer_state(
        &self,
        _r: Request<topo::ListSequencerSynchronizerStateRequest>,
    ) -> Result<Response<topo::ListSequencerSynchronizerStateResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_lsu_announcement(
        &self,
        _r: Request<topo::ListLsuAnnouncementRequest>,
    ) -> Result<Response<topo::ListLsuAnnouncementResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_lsu_sequencer_connection_successor(
        &self,
        _r: Request<topo::ListLsuSequencerConnectionSuccessorRequest>,
    ) -> Result<Response<topo::ListLsuSequencerConnectionSuccessorResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_available_stores(
        &self,
        _r: Request<topo::ListAvailableStoresRequest>,
    ) -> Result<Response<topo::ListAvailableStoresResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_all(
        &self,
        _r: Request<topo::ListAllRequest>,
    ) -> Result<Response<topo::ListAllResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
    async fn list_all_v2(
        &self,
        _r: Request<topo::ListAllV2Request>,
    ) -> Result<Response<topo::ListAllV2Response>, Status> {
        Err(Status::unimplemented("test"))
    }
    type ExportTopologySnapshotStream = Empty<topo::ExportTopologySnapshotResponse>;
    async fn export_topology_snapshot(
        &self,
        _r: Request<topo::ExportTopologySnapshotRequest>,
    ) -> Result<Response<Self::ExportTopologySnapshotStream>, Status> {
        Err(Status::unimplemented("test"))
    }
    type ExportTopologySnapshotV2Stream = Empty<topo::ExportTopologySnapshotV2Response>;
    async fn export_topology_snapshot_v2(
        &self,
        _r: Request<topo::ExportTopologySnapshotV2Request>,
    ) -> Result<Response<Self::ExportTopologySnapshotV2Stream>, Status> {
        Err(Status::unimplemented("test"))
    }
    type GenesisStateStream = Empty<topo::GenesisStateResponse>;
    async fn genesis_state(
        &self,
        _r: Request<topo::GenesisStateRequest>,
    ) -> Result<Response<Self::GenesisStateStream>, Status> {
        Err(Status::unimplemented("test"))
    }
    type GenesisStateV2Stream = Empty<topo::GenesisStateV2Response>;
    async fn genesis_state_v2(
        &self,
        _r: Request<topo::GenesisStateV2Request>,
    ) -> Result<Response<Self::GenesisStateV2Stream>, Status> {
        Err(Status::unimplemented("test"))
    }
    type SequencerLsuStateStream = Empty<topo::SequencerLsuStateResponse>;
    async fn sequencer_lsu_state(
        &self,
        _r: Request<topo::SequencerLsuStateRequest>,
    ) -> Result<Response<Self::SequencerLsuStateStream>, Status> {
        Err(Status::unimplemented("test"))
    }
}

async fn serve(mock: MockTopology) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(TopologyManagerReadServiceServer::new(mock), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(150)).await;
    format!("http://localhost:{port}")
}

#[tokio::test]
async fn the_three_list_reads_map_complete_responses_into_entries() {
    let mock = MockTopology::default();
    let requests = mock.party_requests.clone();
    let client = TopologyClient::connect_lazy(Config::new(serve(mock).await)).unwrap();

    let parties = client
        .list_party_to_participant(Store::Authorized, "alice", "participant-1")
        .await
        .expect("a complete response maps");
    assert_eq!(parties.len(), 1);
    assert_eq!(parties[0].context.serial, 7);
    assert_eq!(parties[0].item.party, "alice::fp");

    // The request carries what the caller asked: the store, both filters, and
    // the head-state time query — silently dropping a filter would return the
    // whole topology as if it were the filtered answer.
    let sent = requests.lock().unwrap()[0].clone();
    assert_eq!(sent.filter_party, "alice");
    assert_eq!(sent.filter_participant, "participant-1");
    let base = sent.base_query.expect("a base query");
    assert!(base.store.is_some(), "the store must be named");
    assert!(
        matches!(
            base.time_query,
            Some(topo::base_query::TimeQuery::HeadState(()))
        ),
        "reads are against head state"
    );

    let delegations = client
        .list_namespace_delegations(Store::Authorized, "", "")
        .await
        .expect("delegations map");
    assert_eq!(delegations.len(), 1);

    let vetted = client
        .list_vetted_packages(Store::Authorized, "")
        .await
        .expect("vetted packages map");
    assert_eq!(vetted.len(), 1);
}

#[tokio::test]
async fn a_partial_topology_row_fails_the_read_instead_of_shrinking_it() {
    let client = TopologyClient::connect_lazy(Config::new(
        serve(MockTopology {
            partial: true,
            ..Default::default()
        })
        .await,
    ))
    .unwrap();

    // One row, mapping absent: the whole read errors. A topology answer that
    // is quietly short looks exactly like a right one, which is why partial
    // data is refused end to end and not only in the unit-tested helper.
    let error = client
        .list_party_to_participant(Store::Authorized, "", "")
        .await
        .expect_err("a row without its mapping cannot be returned as data");
    assert!(format!("{error}").contains("incomplete"), "got {error:?}");
}
