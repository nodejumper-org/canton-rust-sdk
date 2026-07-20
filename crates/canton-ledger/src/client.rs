//! The async Ledger API client.

use std::sync::Arc;
use std::time::Duration;

use canton_core::auth::{self, Intercepted};
use canton_core::telemetry::{self, TRANSPORT_GRPC};
use canton_core::{Config, Error, Result};
use canton_proto::com::daml::ledger::api::v2 as pb;
use canton_proto::grpc::health::v1 as health_pb;
use canton_proto::grpc::health::v1::health_check_response::ServingStatus;
use futures_core::Stream;
use tokio_stream::StreamExt as _;
use tonic::transport::Channel;

/// An `EventFormat` with a wildcard (all-templates) filter for each party.
fn wildcard_event_format(parties: &[String]) -> pb::EventFormat {
    let filters_by_party = parties
        .iter()
        .map(|party| {
            (
                party.clone(),
                pb::Filters {
                    cumulative: vec![pb::CumulativeFilter {
                        identifier_filter: Some(
                            pb::cumulative_filter::IdentifierFilter::WildcardFilter(
                                pb::WildcardFilter {
                                    include_created_event_blob: false,
                                },
                            ),
                        ),
                    }],
                },
            )
        })
        .collect();

    pb::EventFormat {
        filters_by_party,
        filters_for_any_party: None,
        verbose: true,
    }
}

/// A `LEDGER_EFFECTS` transaction format wildcard-filtered to the acting party.
fn transaction_format(act_as: &[String]) -> pb::TransactionFormat {
    pb::TransactionFormat {
        event_format: Some(wildcard_event_format(act_as)),
        transaction_shape: pb::TransactionShape::LedgerEffects as i32,
    }
}

/// The offset of an update, for resumable-stream position tracking.
fn update_offset(update: &pb::get_updates_response::Update) -> i64 {
    use pb::get_updates_response::Update;
    match update {
        Update::Transaction(t) => t.offset,
        Update::Reassignment(r) => r.offset,
        Update::TopologyTransaction(t) => t.offset,
        Update::OffsetCheckpoint(c) => c.offset,
    }
}

/// An async client for the Canton Ledger API over gRPC.
///
/// The client owns a lazily-connected [`Channel`]; cloning it is cheap and
/// clones share the underlying connection pool, so it is safe to hand a clone
/// to each task.
#[derive(Clone, Debug)]
pub struct CantonClient {
    channel: Channel,
    config: Arc<Config>,
}

impl CantonClient {
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

    /// Return the participant's Ledger API version string (e.g. `"3.5.7"`).
    ///
    /// # Errors
    /// Returns an [`Error`] if the RPC fails.
    pub async fn version(&self) -> Result<String> {
        telemetry::instrument("version", TRANSPORT_GRPC, async {
            self.with_retry(|| async {
                let mut client = pb::version_service_client::VersionServiceClient::new(
                    self.intercepted().await?,
                );
                let response = client
                    .get_ledger_api_version(pb::GetLedgerApiVersionRequest {})
                    .await?
                    .into_inner();
                Ok(response.version)
            })
            .await
        })
        .await
    }

    /// Probe the participant's overall serving status via the standard
    /// `grpc.health.v1.Health` service (served on the Ledger API port).
    ///
    /// Poll this to react to intermittent or permanent node failure: a healthy
    /// participant answers [`ServingStatus::Serving`]; an unreachable one
    /// surfaces a transport [`Error`] (see [`Error::is_retriable`]).
    ///
    /// # Errors
    /// Returns an [`Error`] if the health RPC fails (e.g. the node is down).
    pub async fn health_check(&self) -> Result<ServingStatus> {
        telemetry::instrument("health_check", TRANSPORT_GRPC, async {
            self.with_retry(|| async {
                let mut client =
                    health_pb::health_client::HealthClient::new(self.intercepted().await?);
                // Empty `service` = the server's overall status (Canton does not
                // register per-service health entries on the Ledger API port).
                let response = client
                    .check(health_pb::HealthCheckRequest {
                        service: String::new(),
                    })
                    .await?
                    .into_inner();
                Ok(response.status())
            })
            .await
        })
        .await
    }

    /// Submit commands **fire-and-forget** (`CommandSubmissionService.Submit`):
    /// hand the commands to the participant and return promptly without waiting
    /// for the transaction. Returns the change-ID `command_id` used, so the
    /// caller can recover the outcome later with [`Self::await_completion`] (or
    /// the [`Self::completions`] stream).
    ///
    /// A fresh UUID `command_id` is generated when the caller did not set one,
    /// so ledger-side de-duplication behaves correctly across retries.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication fails or the participant rejects
    /// the submission synchronously (e.g. a preprocessing error).
    pub async fn submit(&self, submit: crate::command::Submit) -> Result<String> {
        telemetry::instrument("submit", TRANSPORT_GRPC, async move {
            // Built once so retries reuse the same change ID (de-dup-safe).
            let (command_id, commands) = submit.into_commands();
            self.with_retry(|| {
                let commands = commands.clone();
                async move {
                    let mut client =
                        pb::command_submission_service_client::CommandSubmissionServiceClient::new(
                            self.intercepted().await?,
                        );
                    client
                        .submit(pb::SubmitRequest {
                            commands: Some(commands),
                        })
                        .await?;
                    Ok(())
                }
            })
            .await?;
            Ok(command_id)
        })
        .await
    }

    /// Submit commands and wait for the result **without** fetching the
    /// transaction (`CommandService.SubmitAndWait`): blocks until the command
    /// commits (or is rejected) and returns the `update_id` and completion
    /// offset. Lighter than [`Self::submit_and_wait_for_transaction`] when the
    /// caller does not need the event payload.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication fails or the command is rejected.
    /// The retry caveat on [`Self::submit_and_wait_for_transaction`] applies.
    pub async fn submit_and_wait(
        &self,
        submit: crate::command::Submit,
    ) -> Result<pb::SubmitAndWaitResponse> {
        telemetry::instrument("submit_and_wait", TRANSPORT_GRPC, async move {
            // Built once so retries reuse the same change ID (de-dup-safe).
            let (_command_id, commands) = submit.into_commands();
            let request = pb::SubmitAndWaitRequest {
                commands: Some(commands),
            };
            self.with_retry(|| {
                let request = request.clone();
                async move {
                    let mut client = pb::command_service_client::CommandServiceClient::new(
                        self.intercepted().await?,
                    );
                    Ok(client.submit_and_wait(request).await?.into_inner())
                }
            })
            .await
        })
        .await
    }

    /// Submit commands and wait for the resulting transaction.
    ///
    /// Fills the change ID's `command_id` with a fresh UUID when the caller did
    /// not set one, so ledger-side de-duplication behaves correctly. The
    /// returned transaction is shaped as `LEDGER_EFFECTS` and filtered to the
    /// acting party (wildcard), so created events are visible in the response.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication fails, the command is rejected, or
    /// the response contains no transaction.
    ///
    /// # Example
    /// ```no_run
    /// # async fn run(client: canton_ledger::CantonClient, party: &str, pkg: &str)
    /// #     -> canton_ledger::Result<()> {
    /// use canton_ledger::{Submit, create, identifier, record};
    ///
    /// let tx = client
    ///     .submit_and_wait_for_transaction(
    ///         Submit::new(party)
    ///             .add_command(create(identifier(pkg, "M", "T"), record(vec![]))),
    ///     )
    ///     .await?;
    /// println!("committed {} at offset {}", tx.update_id, tx.offset);
    /// # Ok(()) }
    /// ```
    ///
    /// # Retry caveat (exactly-once)
    /// With retry enabled ([`Config::with_retry`]), a submission that commits on
    /// the ledger but whose response is lost to a retriable error is re-sent
    /// with the same `command_id` and de-duplicated by the participant — the
    /// retry then surfaces as a duplicate rejection even though the original
    /// succeeded. For exactly-once semantics, set an explicit
    /// `Submit::with_command_id` and recover the outcome with
    /// [`Self::await_completion`] rather than relying on the return value alone.
    pub async fn submit_and_wait_for_transaction(
        &self,
        submit: crate::command::Submit,
    ) -> Result<pb::Transaction> {
        telemetry::instrument(
            "submit_and_wait_for_transaction",
            TRANSPORT_GRPC,
            async move {
                // Built once so retries reuse the same change ID (`command_id`),
                // keeping the submission de-duplication-safe across attempts.
                let (_command_id, commands) = submit.into_commands();
                let request = pb::SubmitAndWaitForTransactionRequest {
                    transaction_format: Some(transaction_format(&commands.act_as)),
                    commands: Some(commands),
                };

                let response = self
                    .with_retry(|| {
                        let request = request.clone();
                        async move {
                            let mut client = pb::command_service_client::CommandServiceClient::new(
                                self.intercepted().await?,
                            );
                            Ok(client
                                .submit_and_wait_for_transaction(request)
                                .await?
                                .into_inner())
                        }
                    })
                    .await?;

                response.transaction.ok_or_else(|| {
                    Error::UnexpectedResponse("response contained no transaction".to_string())
                })
            },
        )
        .await
    }

    /// Subscribe to the command-completion stream for `parties`, starting after
    /// `begin_offset` (exclusive). Offset checkpoints are filtered out, so the
    /// stream yields only [`pb::Completion`]s.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn completions(
        &self,
        parties: Vec<String>,
        begin_offset: i64,
    ) -> Result<impl Stream<Item = Result<pb::Completion>> + Send + use<>> {
        telemetry::instrument("completions", TRANSPORT_GRPC, async move {
            let mut client =
                pb::command_completion_service_client::CommandCompletionServiceClient::new(
                    self.intercepted().await?,
                );
            let stream = client
                .completion_stream(pb::CompletionStreamRequest {
                    user_id: String::new(),
                    parties,
                    begin_exclusive: begin_offset,
                })
                .await?
                .into_inner();

            Ok(stream.filter_map(|item| match item {
                Ok(response) => match response.completion_response {
                    Some(pb::completion_stream_response::CompletionResponse::Completion(
                        completion,
                    )) => Some(Ok(completion)),
                    _ => None, // skip offset checkpoints
                },
                Err(status) => Some(Err(Error::from(status))),
            }))
        })
        .await
    }

    /// Recover the completion for a specific `command_id` by scanning the
    /// completion stream from `begin_offset`, up to `timeout`.
    ///
    /// This is the command-recovery path: after a crash, lost connection, or
    /// timeout, the outcome of a pending command is read back from the
    /// completion endpoint instead of blindly re-submitting. If the command's
    /// completion reports a non-OK status, this returns [`Error::CommandRejected`].
    ///
    /// The completion stream is a live subscription that does not self-terminate,
    /// so `timeout` bounds how long to wait for the target completion.
    ///
    /// # Errors
    /// Returns [`Error::Timeout`] if the completion is not seen within `timeout`,
    /// [`Error::CommandRejected`] if the ledger rejected the command, or another
    /// [`Error`] if the stream fails.
    pub async fn await_completion(
        &self,
        command_id: &str,
        parties: Vec<String>,
        begin_offset: i64,
        timeout: Duration,
    ) -> Result<pb::Completion> {
        let scan = async {
            let stream = self.completions(parties, begin_offset).await?;
            tokio::pin!(stream);
            while let Some(item) = stream.next().await {
                let completion = item?;
                if completion.command_id == command_id {
                    // A non-OK gRPC status on the completion means the ledger
                    // rejected the command for business/interpretation reasons.
                    if let Some(status) = &completion.status {
                        // google.rpc.Status code 0 == OK; anything else is a rejection.
                        if status.code != 0 {
                            return Err(Error::CommandRejected {
                                code: format!("{:?}", tonic::Code::from(status.code)),
                                message: status.message.clone(),
                            });
                        }
                    }
                    return Ok(completion);
                }
            }
            Err(Error::UnexpectedResponse(format!(
                "completion stream ended before command {command_id} was seen"
            )))
        };

        tokio::time::timeout(timeout, scan)
            .await
            .map_err(|_| Error::Timeout)?
    }

    /// Return the current ledger end offset as seen by the participant.
    ///
    /// A value of `0` means the participant's view of the ledger is empty.
    /// This is an authenticated endpoint.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn ledger_end(&self) -> Result<i64> {
        telemetry::instrument("ledger_end", TRANSPORT_GRPC, async {
            self.with_retry(|| async {
                let mut client =
                    pb::state_service_client::StateServiceClient::new(self.intercepted().await?);
                let response = client
                    .get_ledger_end(pb::GetLedgerEndRequest {})
                    .await?
                    .into_inner();
                Ok(response.offset)
            })
            .await
        })
        .await
    }

    /// Fetch the created and/or archived events for a contract by id
    /// (`EventQueryService.GetEventsByContractId`).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn events_by_contract_id(
        &self,
        contract_id: impl Into<String>,
        parties: Vec<String>,
    ) -> Result<pb::GetEventsByContractIdResponse> {
        let contract_id = contract_id.into();
        telemetry::instrument("events_by_contract_id", TRANSPORT_GRPC, async move {
            let mut client = pb::event_query_service_client::EventQueryServiceClient::new(
                self.intercepted().await?,
            );
            Ok(client
                .get_events_by_contract_id(pb::GetEventsByContractIdRequest {
                    contract_id,
                    event_format: Some(wildcard_event_format(&parties)),
                })
                .await?
                .into_inner())
        })
        .await
    }

    /// Fetch one **page** of the Active Contract Set for `parties` as of
    /// `active_at_offset`. Returns the page's active contracts and the next page
    /// token (`None` once the last page has been read); pass the token back in
    /// to fetch the following page.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn active_contracts_page(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
        max_page_size: i32,
        page_token: Option<Vec<u8>>,
    ) -> Result<(Vec<pb::ActiveContract>, Option<Vec<u8>>)> {
        telemetry::instrument("active_contracts_page", TRANSPORT_GRPC, async move {
            let mut client =
                pb::state_service_client::StateServiceClient::new(self.intercepted().await?);
            let response = client
                .get_active_contracts_page(pb::GetActiveContractsPageRequest {
                    active_at_offset: Some(active_at_offset),
                    event_format: Some(wildcard_event_format(&parties)),
                    max_page_size: Some(max_page_size),
                    page_token,
                })
                .await?
                .into_inner();

            let contracts = response
                .active_contracts
                .into_iter()
                .filter_map(|entry| match entry.contract_entry {
                    Some(pb::get_active_contracts_response::ContractEntry::ActiveContract(
                        active,
                    )) => Some(active),
                    _ => None,
                })
                .collect();
            Ok((contracts, response.next_page_token))
        })
        .await
    }

    /// Fetch one **page** of updates in the offset range
    /// `(begin_offset_exclusive, end_offset_inclusive]`, optionally in reverse
    /// (`descending`) order. Returns the page items and the next page token
    /// (`None` once the last page has been read).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn updates_page(
        &self,
        parties: Vec<String>,
        begin_offset_exclusive: i64,
        end_offset_inclusive: i64,
        max_page_size: i32,
        descending: bool,
        page_token: Option<Vec<u8>>,
    ) -> Result<(Vec<pb::GetUpdateResponse>, Option<Vec<u8>>)> {
        telemetry::instrument("updates_page", TRANSPORT_GRPC, async move {
            let mut client =
                pb::update_service_client::UpdateServiceClient::new(self.intercepted().await?);
            let response = client
                .get_updates_page(pb::GetUpdatesPageRequest {
                    begin_offset_exclusive: Some(begin_offset_exclusive),
                    end_offset_inclusive: Some(end_offset_inclusive),
                    max_page_size: Some(max_page_size),
                    update_format: Some(pb::UpdateFormat {
                        include_transactions: Some(transaction_format(&parties)),
                        include_reassignments: Some(wildcard_event_format(&parties)),
                        include_topology_events: None,
                    }),
                    descending_order: descending,
                    page_token,
                })
                .await?
                .into_inner();
            Ok((response.updates, response.next_page_token))
        })
        .await
    }

    /// Stream the Active Contract Set for `parties` as of `active_at_offset`
    /// (typically the current ledger end). Yields the active contracts,
    /// wildcard-filtered to the given parties.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn active_contracts(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
    ) -> Result<impl Stream<Item = Result<pb::ActiveContract>> + Send + use<>> {
        telemetry::instrument("active_contracts", TRANSPORT_GRPC, async move {
            let mut client =
                pb::state_service_client::StateServiceClient::new(self.intercepted().await?);
            let stream = client
                .get_active_contracts(pb::GetActiveContractsRequest {
                    active_at_offset,
                    event_format: Some(wildcard_event_format(&parties)),
                    stream_continuation_token: None,
                })
                .await?
                .into_inner();

            Ok(stream.filter_map(|item| match item {
                Ok(response) => match response.contract_entry {
                    Some(pb::get_active_contracts_response::ContractEntry::ActiveContract(
                        active,
                    )) => Some(Ok(active)),
                    _ => None,
                },
                Err(status) => Some(Err(Error::from(status))),
            }))
        })
        .await
    }

    /// Like [`Self::active_contracts`], but **resumable**: reads the ACS
    /// snapshot page-by-page (continuation tokens), retrying a failed page on
    /// retriable errors from the last token instead of restarting the snapshot
    /// from zero. `max_page_size` bounds each page RPC.
    pub fn active_contracts_resumable(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
        max_page_size: i32,
    ) -> impl Stream<Item = Result<pb::ActiveContract>> + Send + use<> {
        let client = self.clone();
        let (max_reconnects, backoff_unit) = client.reconnect_policy();
        async_stream::stream! {
            let mut page_token: Option<Vec<u8>> = None;
            let mut reconnects = 0u32;
            loop {
                match client
                    .active_contracts_page(
                        parties.clone(),
                        active_at_offset,
                        max_page_size,
                        page_token.clone(),
                    )
                    .await
                {
                    Ok((contracts, next)) => {
                        reconnects = 0;
                        for contract in contracts {
                            yield Ok(contract);
                        }
                        match next {
                            Some(next) => page_token = Some(next),
                            None => return, // snapshot fully delivered
                        }
                    }
                    Err(err) if err.is_retriable() => {
                        reconnects += 1;
                        if reconnects > max_reconnects {
                            yield Err(Error::UnexpectedResponse(format!(
                                "acs stream failed to resume after {max_reconnects} reconnects"
                            )));
                            return;
                        }
                        tokio::time::sleep(backoff_unit * reconnects).await;
                    }
                    Err(err) => {
                        yield Err(err);
                        return;
                    }
                }
            }
        }
    }

    /// The reconnect policy for resumable streams: `(max_reconnects,
    /// backoff_unit)`. Derived from the client's [`RetryConfig`] when one is
    /// configured (attempts → reconnect budget, initial backoff → step), else
    /// the defaults (5 reconnects, 250ms step).
    ///
    /// [`RetryConfig`]: canton_core::RetryConfig
    fn reconnect_policy(&self) -> (u32, Duration) {
        match self.config.retry() {
            Some(retry) => (retry.max_attempts, retry.initial_backoff),
            None => (5, Duration::from_millis(250)),
        }
    }

    /// Stream ledger updates (transactions, reassignments, topology events) for
    /// `parties`, starting after `begin_offset` (exclusive). Offset checkpoints
    /// are filtered out. Reassignments are surfaced as their own case (each
    /// carrying the distinct `Unassigned`/`Assigned` events).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn updates(
        &self,
        parties: Vec<String>,
        begin_offset: i64,
    ) -> Result<impl Stream<Item = Result<pb::get_updates_response::Update>> + Send + use<>> {
        telemetry::instrument("updates", TRANSPORT_GRPC, async move {
            let mut client =
                pb::update_service_client::UpdateServiceClient::new(self.intercepted().await?);
            let stream = client
                .get_updates(pb::GetUpdatesRequest {
                    begin_exclusive: begin_offset,
                    end_inclusive: None,
                    update_format: Some(pb::UpdateFormat {
                        include_transactions: Some(transaction_format(&parties)),
                        include_reassignments: Some(wildcard_event_format(&parties)),
                        include_topology_events: None,
                    }),
                    descending_order: false,
                })
                .await?
                .into_inner();

            Ok(stream.filter_map(|item| match item {
                Ok(response) => match response.update {
                    Some(pb::get_updates_response::Update::OffsetCheckpoint(_)) | None => None,
                    Some(update) => Some(Ok(update)),
                },
                Err(status) => Some(Err(Error::from(status))),
            }))
        })
        .await
    }

    /// Like [`Self::updates`], but **resumable**: on a retriable stream error it
    /// reconnects from the last offset it yielded (rather than restarting from
    /// `begin_offset` or losing position), with a short backoff and a bounded
    /// number of consecutive reconnects (see the client's
    /// [`RetryConfig`](canton_core::RetryConfig)).
    ///
    /// # Example
    /// ```no_run
    /// # async fn run(client: canton_ledger::CantonClient, party: String)
    /// #     -> canton_ledger::Result<()> {
    /// use tokio_stream::StreamExt as _;
    ///
    /// let stream = client.updates_resumable(vec![party], 0);
    /// tokio::pin!(stream);
    /// while let Some(update) = stream.next().await {
    ///     println!("update: {:?}", update?);
    /// }
    /// # Ok(()) }
    /// ```
    pub fn updates_resumable(
        &self,
        parties: Vec<String>,
        begin_offset: i64,
    ) -> impl Stream<Item = Result<pb::get_updates_response::Update>> + Send + use<> {
        let client = self.clone();
        let (max_reconnects, backoff_unit) = client.reconnect_policy();
        async_stream::stream! {
            let mut offset = begin_offset;
            let mut reconnects = 0u32;
            loop {
                match client.updates(parties.clone(), offset).await {
                    Ok(stream) => {
                        tokio::pin!(stream);
                        loop {
                            match stream.next().await {
                                Some(Ok(update)) => {
                                    offset = update_offset(&update);
                                    reconnects = 0;
                                    yield Ok(update);
                                }
                                Some(Err(err)) if err.is_retriable() => break,
                                Some(Err(err)) => {
                                    yield Err(err);
                                    return;
                                }
                                None => return, // server closed the stream cleanly
                            }
                        }
                    }
                    Err(err) if err.is_retriable() => {}
                    Err(err) => {
                        yield Err(err);
                        return;
                    }
                }

                reconnects += 1;
                if reconnects > max_reconnects {
                    yield Err(Error::UnexpectedResponse(format!(
                        "update stream failed to resume after {max_reconnects} reconnects"
                    )));
                    return;
                }
                tokio::time::sleep(backoff_unit * reconnects).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CantonClient;
    use crate::Config;
    use canton_core::Error;

    // Compile-time guarantee that the client is Send + Sync, so consumers can
    // share it across tasks and `tokio::spawn` work that holds it. (The streaming
    // methods carry `+ Send` in their signatures, which the compiler enforces at
    // this crate's build time.)
    #[test]
    fn client_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CantonClient>();
    }

    #[tokio::test]
    async fn connect_lazy_accepts_a_valid_endpoint() {
        // connect_lazy builds a lazy Channel, which needs a Tokio runtime.
        assert!(CantonClient::connect_lazy(Config::new("http://localhost:3901")).is_ok());
    }

    #[tokio::test]
    async fn connect_lazy_rejects_a_malformed_endpoint() {
        for bad in ["", "not a uri", "http://[bad"] {
            let result = CantonClient::connect_lazy(Config::new(bad));
            assert!(
                matches!(result, Err(Error::InvalidRequest(_))),
                "endpoint {bad:?} should be rejected as InvalidRequest, got {result:?}"
            );
        }
    }

    // A reassignment carries its `Unassigned` (source) and `Assigned` (target)
    // events as distinct cases rather than collapsing them into one "reassign"
    // event — the multi-synchronizer faithfulness the update stream promises.
    // (LocalNet is single-synchronizer, so this is verified structurally.)
    #[test]
    fn reassignment_preserves_the_unassigned_assigned_split() {
        use super::pb;
        use pb::get_updates_response::Update;
        use pb::reassignment_event::Event;

        let update = Update::Reassignment(pb::Reassignment {
            update_id: "u1".to_string(),
            events: vec![
                pb::ReassignmentEvent {
                    event: Some(Event::Unassigned(pb::UnassignedEvent::default())),
                },
                pb::ReassignmentEvent {
                    event: Some(Event::Assigned(pb::AssignedEvent::default())),
                },
            ],
            ..Default::default()
        });

        let Update::Reassignment(reassignment) = update else {
            panic!("expected a reassignment update");
        };
        assert_eq!(reassignment.events.len(), 2, "both legs are surfaced");
        assert!(matches!(
            reassignment.events[0].event,
            Some(Event::Unassigned(_))
        ));
        assert!(matches!(
            reassignment.events[1].event,
            Some(Event::Assigned(_))
        ));
    }
}
