//! The holdings read, against a `StateService` standing in this process.
//!
//! `canton_token::holdings` is what every transfer and allocation starts from,
//! and until now it ran only against a live participant: the filter it sends,
//! the views it decodes, and the lock it reads were checked nowhere CI can
//! reach. The stub here answers what a participant answers and records what
//! it was asked, so both halves are pinned.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_daml as rt;
use canton_daml::{Contract as _, ToValue as _};
use canton_ledger::{CantonClient, Config};
use canton_proto::com::daml::ledger::api::v2 as pb;
use canton_token::types::metadata as md;
use canton_token::types::v1::holding as h;
use pb::state_service_server::{StateService, StateServiceServer};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

fn party(s: &str) -> rt::Party {
    rt::Party::parse(s).expect("a party")
}

/// A created event carrying one `Holding` interface view.
fn holding(contract_id: &str, instrument: &str, amount: &str, locked: bool) -> pb::CreatedEvent {
    let view = h::HoldingView {
        owner: party("alice::1220ab"),
        instrument_id: h::InstrumentId {
            admin: party("dso::1220ef"),
            id: instrument.to_string(),
        },
        amount: amount.parse().expect("a numeric"),
        lock: locked.then(|| h::Lock {
            holders: vec![party("dso::1220ef")],
            expires_at: Some(rt::Timestamp(1_700_000_600_000_000)),
            expires_after: None,
            context: Some("allocation in flight".to_string()),
        }),
        meta: md::Metadata {
            values: rt::TextMap::new(),
        },
    };
    let record = match view.to_value().sum {
        Some(pb::value::Sum::Record(record)) => record,
        other => panic!("a view is a record, got {other:?}"),
    };
    with_view(contract_id, Some(record))
}

fn with_view(contract_id: &str, view_value: Option<pb::Record>) -> pb::CreatedEvent {
    pb::CreatedEvent {
        contract_id: contract_id.to_string(),
        interface_views: vec![pb::InterfaceView {
            interface_id: Some(h::Holding::template_id()),
            view_status: None,
            view_value,
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn active(created_event: Option<pb::CreatedEvent>) -> pb::GetActiveContractsResponse {
    pb::GetActiveContractsResponse {
        contract_entry: Some(
            pb::get_active_contracts_response::ContractEntry::ActiveContract(pb::ActiveContract {
                created_event,
                synchronizer_id: "sync::1220ab".to_string(),
                reassignment_counter: 0,
            }),
        ),
        ..Default::default()
    }
}

/// A `StateService` that answers one ledger end and one fixed ACS, and keeps
/// the ACS request it was sent.
#[derive(Clone)]
struct Acs {
    entries: Vec<pb::GetActiveContractsResponse>,
    seen: Arc<Mutex<Option<pb::GetActiveContractsRequest>>>,
}

#[tonic::async_trait]
impl StateService for Acs {
    async fn get_active_contracts_page(
        &self,
        _request: Request<pb::GetActiveContractsPageRequest>,
    ) -> Result<Response<pb::GetActiveContractsPageResponse>, Status> {
        Err(Status::unimplemented("the read streams"))
    }

    type GetActiveContractsStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<pb::GetActiveContractsResponse, Status>> + Send>,
    >;
    async fn get_active_contracts(
        &self,
        request: Request<pb::GetActiveContractsRequest>,
    ) -> Result<Response<Self::GetActiveContractsStream>, Status> {
        *self.seen.lock().unwrap() = Some(request.into_inner());
        let items: Vec<Result<pb::GetActiveContractsResponse, Status>> =
            self.entries.iter().cloned().map(Ok).collect();
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
        Ok(Response::new(pb::GetLedgerEndResponse {
            offset: 42,
            ..Default::default()
        }))
    }
    async fn get_latest_pruned_offsets(
        &self,
        _r: Request<pb::GetLatestPrunedOffsetsRequest>,
    ) -> Result<Response<pb::GetLatestPrunedOffsetsResponse>, Status> {
        Err(Status::unimplemented("test"))
    }
}

async fn ledger(entries: Vec<pb::GetActiveContractsResponse>) -> (CantonClient, Acs) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);
    let service = Acs {
        entries,
        seen: Arc::new(Mutex::new(None)),
    };
    let serving = service.clone();
    tokio::spawn(async move {
        Server::builder()
            .serve_with_incoming(StateServiceServer::new(serving), incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    let client = CantonClient::connect_lazy(Config::new(format!("http://127.0.0.1:{port}")))
        .expect("a client");
    (client, service)
}

/// The participant's answer for a wallet with three holdings, one of them
/// locked, plus two entries a real ACS carries that are not holdings at all.
fn a_wallet() -> Vec<pb::GetActiveContractsResponse> {
    vec![
        active(None),
        active(Some(pb::CreatedEvent {
            contract_id: "00no-views".to_string(),
            ..Default::default()
        })),
        active(Some(holding("00a", "Amulet", "10.5", false))),
        active(Some(holding("00b", "Amulet", "5", true))),
        active(Some(holding("00c", "Other", "7", false))),
    ]
}

#[tokio::test]
async fn the_read_asks_for_the_holding_interface_view_at_the_ledger_end() {
    let (client, service) = ledger(a_wallet()).await;

    canton_token::holdings::holdings(&client, "alice::1220ab", None)
        .await
        .expect("reads");

    let request = service
        .seen
        .lock()
        .unwrap()
        .clone()
        .expect("the ACS was asked");
    assert_eq!(
        request.active_at_offset, 42,
        "as of the ledger end it asked for first"
    );
    let format = request.event_format.expect("an event format");
    let filters = format
        .filters_by_party
        .get("alice::1220ab")
        .expect("filtered to the owner");
    let shape = format!("{filters:?}");
    assert!(shape.contains("include_interface_view: true"), "{shape}");
    assert!(shape.contains("Splice.Api.Token.HoldingV1"), "{shape}");
    assert!(shape.contains("entity_name: \"Holding\""), "{shape}");
}

#[tokio::test]
async fn every_holding_is_read_and_the_instrument_filter_is_exact() {
    let (client, _) = ledger(a_wallet()).await;

    let all = canton_token::holdings::holdings(&client, "alice::1220ab", None)
        .await
        .expect("reads");
    let ids: Vec<&str> = all.iter().map(|h| h.contract_id.as_str()).collect();
    assert_eq!(
        ids,
        vec!["00a", "00b", "00c"],
        "entries without a view are skipped, not errors"
    );

    let amulet = canton_token::holdings::holdings(&client, "alice::1220ab", Some("Amulet"))
        .await
        .expect("reads");
    let ids: Vec<&str> = amulet.iter().map(|h| h.contract_id.as_str()).collect();
    assert_eq!(ids, vec!["00a", "00b"]);

    let first = &amulet[0];
    assert_eq!(first.instrument_id(), "Amulet");
    assert_eq!(first.amount(), &"10.5".parse::<rt::Numeric>().unwrap());
    assert!(!first.is_locked());
    assert!(
        amulet[1].is_locked(),
        "a Lock in the view is a locked holding"
    );
    assert_eq!(first.typed::<h::Holding>().as_str(), "00a");

    let none = canton_token::holdings::holdings(&client, "alice::1220ab", Some("Amule"))
        .await
        .expect("reads");
    assert!(
        none.is_empty(),
        "the instrument id is matched whole, not as a prefix"
    );
}

#[tokio::test]
async fn spendable_is_the_unlocked_subset_of_one_instrument() {
    let (client, _) = ledger(a_wallet()).await;

    let spendable = canton_token::holdings::spendable(&client, "alice::1220ab", "Amulet")
        .await
        .expect("reads");

    let ids: Vec<&str> = spendable.iter().map(|h| h.contract_id.as_str()).collect();
    assert_eq!(
        ids,
        vec!["00a"],
        "the locked one and the other instrument are both out"
    );
}

#[tokio::test]
async fn a_view_that_is_not_a_holding_view_names_the_contract() {
    let wrong = pb::Record {
        fields: vec![pb::RecordField {
            label: "owner".to_string(),
            value: Some(pb::Value {
                sum: Some(pb::value::Sum::Int64(7)),
            }),
        }],
        ..Default::default()
    };
    let (client, _) = ledger(vec![active(Some(with_view("00bad", Some(wrong))))]).await;

    let error = canton_token::holdings::holdings(&client, "alice::1220ab", None)
        .await
        .expect_err("a view that does not decode is not silently dropped");

    assert!(
        matches!(error, canton_core::Error::UnexpectedResponse(_)),
        "{error:?}"
    );
    assert!(error.to_string().contains("00bad"), "{error}");
}
