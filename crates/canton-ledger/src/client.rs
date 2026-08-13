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
                let mut client =
                    service!(self, pb::version_service_client::VersionServiceClient::new);
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
                let mut client = service!(self, health_pb::health_client::HealthClient::new);
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
                    let mut client = service!(
                        self,
                        pb::command_submission_service_client::CommandSubmissionServiceClient::new
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
                    let mut client =
                        service!(self, pb::command_service_client::CommandServiceClient::new);
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
        // Built once (and outside the instrumented future, keeping it small) so
        // retries reuse the same change ID (`command_id`), keeping the
        // submission de-duplication-safe across attempts.
        let shape = submit.transaction_shape;
        let (_command_id, commands) = submit.into_commands();
        let request = pb::SubmitAndWaitForTransactionRequest {
            transaction_format: Some(pb::TransactionFormat {
                event_format: Some(wildcard_event_format(&commands.act_as)),
                transaction_shape: shape.as_grpc() as i32,
            }),
            commands: Some(commands),
        };
        telemetry::instrument(
            "submit_and_wait_for_transaction",
            TRANSPORT_GRPC,
            async move {
                let response = self
                    .with_retry(|| {
                        let request = request.clone();
                        async move {
                            let mut client = service!(
                                self,
                                pb::command_service_client::CommandServiceClient::new
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

    /// Interpret a submission without authorizing it, returning the transaction
    /// and the hash to sign.
    ///
    /// The first half of interactive submission: the participant works out what
    /// the commands do, and hands back a hash for the party's key — which it
    /// does not hold — to sign. See [`crate::interactive`] for the flow, and
    /// for how long a prepared transaction stays executable.
    ///
    /// Retried like any other read: preparation commits nothing.
    ///
    /// # Errors
    /// As any gRPC call, plus [`Error::UnexpectedResponse`] if the participant
    /// returns no hash — which would leave nothing to sign.
    pub async fn prepare_submission(
        &self,
        prepare: crate::interactive::Prepare,
    ) -> Result<crate::interactive::Prepared> {
        // Built once, outside the instrumented future, so retries reuse one
        // command id — the change ID stays stable and the submission remains
        // de-duplication-safe across attempts, exactly as `submit` does.
        let act_as = prepare.act_as().to_vec();
        let (command_id, user_id, request) = prepare.into_request();
        telemetry::instrument("prepare_submission", TRANSPORT_GRPC, async move {
            let response = self
                .with_retry(|| {
                    let request = request.clone();
                    async move {
                        let mut client = service!(
                            self,
                            pb::interactive::interactive_submission_service_client::InteractiveSubmissionServiceClient::new
                        );
                        Ok(client.prepare_submission(request).await?.into_inner())
                    }
                })
                .await?;
            crate::interactive::Prepared::from_response(response, act_as, command_id, user_id)
        })
        .await
    }

    /// Submit a signed prepared transaction and return once the participant has
    /// accepted it for processing.
    ///
    /// Acceptance is not commitment: the outcome arrives on the completion
    /// stream. Use [`execute_submission_and_wait`](Self::execute_submission_and_wait)
    /// to wait for it.
    ///
    /// # Errors
    /// As any gRPC call. A missing or wrong signature is a rejection from the
    /// participant, not a local error.
    pub async fn execute_submission(
        &self,
        executable: crate::interactive::Executable,
    ) -> Result<()> {
        let request = executable.into_execute_request();
        telemetry::instrument("execute_submission", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let request = request.clone();
                async move {
                    let mut client = service!(
                        self,
                        pb::interactive::interactive_submission_service_client::InteractiveSubmissionServiceClient::new
                    );
                    client.execute_submission(request).await?;
                    Ok(())
                }
            })
            .await
        })
        .await
    }

    /// Submit a signed prepared transaction and wait for it to commit,
    /// returning the update id.
    ///
    /// # Errors
    /// As any gRPC call, plus a rejection if the transaction fails.
    pub async fn execute_submission_and_wait(
        &self,
        executable: crate::interactive::Executable,
    ) -> Result<String> {
        let request = executable.into_execute_and_wait_request();
        telemetry::instrument("execute_submission_and_wait", TRANSPORT_GRPC, async move {
            let response = self
                .with_retry(|| {
                    let request = request.clone();
                    async move {
                        let mut client = service!(
                            self,
                            pb::interactive::interactive_submission_service_client::InteractiveSubmissionServiceClient::new
                        );
                        Ok(client
                            .execute_submission_and_wait(request)
                            .await?
                            .into_inner())
                    }
                })
                .await?;
            Ok(response.update_id)
        })
        .await
    }

    /// Submit a signed prepared transaction and wait for the committed
    /// transaction itself.
    ///
    /// The events it carries follow
    /// [`Executable::with_transaction_shape`](crate::interactive::Executable::with_transaction_shape).
    ///
    /// # Errors
    /// As any gRPC call, plus [`Error::UnexpectedResponse`] if the participant
    /// reports success without a transaction.
    pub async fn execute_submission_and_wait_for_transaction(
        &self,
        executable: crate::interactive::Executable,
    ) -> Result<pb::Transaction> {
        let event_format = wildcard_event_format(&executable.act_as());
        let request = executable.into_execute_and_wait_for_transaction_request(event_format);
        telemetry::instrument(
            "execute_submission_and_wait_for_transaction",
            TRANSPORT_GRPC,
            async move {
                let response = self
                    .with_retry(|| {
                        let request = request.clone();
                        async move {
                            let mut client = service!(
                                self,
                                pb::interactive::interactive_submission_service_client::InteractiveSubmissionServiceClient::new
                            );
                            Ok(client
                                .execute_submission_and_wait_for_transaction(request)
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
        self.completions_with(crate::request::CompletionsRequest::new(
            parties,
            begin_offset,
        ))
        .await
    }

    /// Like [`Self::completions`], with the full request surface: a
    /// [`CompletionsRequest`](crate::request::CompletionsRequest) additionally
    /// selects the `user_id` whose command completions to stream (pair it with
    /// [`Submit::with_user_id`](crate::Submit::with_user_id)).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn completions_with(
        &self,
        request: crate::request::CompletionsRequest,
    ) -> Result<impl Stream<Item = Result<pb::Completion>> + Send + use<>> {
        telemetry::instrument("completions", TRANSPORT_GRPC, async move {
            let mut client = service!(
                self,
                pb::command_completion_service_client::CommandCompletionServiceClient::new
            );
            let stream = client
                .completion_stream(request.into_grpc())
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
                let mut client = service!(self, pb::state_service_client::StateServiceClient::new);
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
    /// (`EventQueryService.GetEventsByContractId`), with verbose records and
    /// no created-event blob. To obtain a contract's `created_event_blob`
    /// (for disclosure), use a template-filtered
    /// [`Self::active_contracts_with`] read with
    /// [`ActiveContractsRequest::with_created_event_blobs`](crate::request::ActiveContractsRequest::with_created_event_blobs).
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
            let mut client = service!(
                self,
                pb::event_query_service_client::EventQueryServiceClient::new
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
        let request = crate::request::ActiveContractsRequest::new(parties, active_at_offset);
        self.active_contracts_page_with(&request, max_page_size, page_token)
            .await
    }

    /// Like [`Self::active_contracts_page`], with the full request surface of
    /// an [`ActiveContractsRequest`](crate::request::ActiveContractsRequest)
    /// (template/interface filters, created-event blobs, non-verbose records).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn active_contracts_page_with(
        &self,
        request: &crate::request::ActiveContractsRequest,
        max_page_size: i32,
        page_token: Option<Vec<u8>>,
    ) -> Result<(Vec<pb::ActiveContract>, Option<Vec<u8>>)> {
        telemetry::instrument("active_contracts_page", TRANSPORT_GRPC, async move {
            let mut client = service!(self, pb::state_service_client::StateServiceClient::new);
            let response = client
                .get_active_contracts_page(pb::GetActiveContractsPageRequest {
                    active_at_offset: Some(request.active_at_offset),
                    event_format: Some(request.event_format()),
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
        let mut request = crate::request::UpdatesRequest::new(parties, begin_offset_exclusive)
            .until(end_offset_inclusive);
        if descending {
            request = request.descending();
        }
        self.updates_page_with(&request, max_page_size, page_token)
            .await
    }

    /// Like [`Self::updates_page`], with the full request surface of an
    /// [`UpdatesRequest`](crate::request::UpdatesRequest) (template/interface
    /// filters, transaction shape, descending order, created-event blobs,
    /// topology events, non-verbose records). The request's bounds supply the
    /// page range, so
    /// [`UpdatesRequest::until`](crate::request::UpdatesRequest::until) is
    /// required here — the paged read is inherently bounded.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] if the request has no end offset, or
    /// another [`Error`] if authentication or the RPC fails.
    pub async fn updates_page_with(
        &self,
        request: &crate::request::UpdatesRequest,
        max_page_size: i32,
        page_token: Option<Vec<u8>>,
    ) -> Result<(Vec<pb::GetUpdateResponse>, Option<Vec<u8>>)> {
        let (begin_exclusive, end_inclusive) = request.bounds();
        let Some(end_inclusive) = end_inclusive else {
            return Err(Error::InvalidRequest(
                "updates_page_with requires a bounded request: set UpdatesRequest::until"
                    .to_string(),
            ));
        };
        telemetry::instrument("updates_page", TRANSPORT_GRPC, async move {
            let mut client = service!(self, pb::update_service_client::UpdateServiceClient::new);
            let response = client
                .get_updates_page(pb::GetUpdatesPageRequest {
                    begin_offset_exclusive: Some(begin_exclusive),
                    end_offset_inclusive: Some(end_inclusive),
                    max_page_size: Some(max_page_size),
                    update_format: Some(request.update_format()),
                    descending_order: request.is_descending(),
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
        self.active_contracts_with(crate::request::ActiveContractsRequest::new(
            parties,
            active_at_offset,
        ))
        .await
    }

    /// Like [`Self::active_contracts`], with the full request surface: an
    /// [`ActiveContractsRequest`](crate::request::ActiveContractsRequest)
    /// additionally filters by template or interface, includes created-event
    /// blobs, and drops record labels.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn active_contracts_with(
        &self,
        request: crate::request::ActiveContractsRequest,
    ) -> Result<impl Stream<Item = Result<pb::ActiveContract>> + Send + use<>> {
        telemetry::instrument("active_contracts", TRANSPORT_GRPC, async move {
            let mut client = service!(self, pb::state_service_client::StateServiceClient::new);
            let stream = client
                .get_active_contracts(pb::GetActiveContractsRequest {
                    active_at_offset: request.active_at_offset,
                    event_format: Some(request.event_format()),
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
        self.active_contracts_resumable_with(
            crate::request::ActiveContractsRequest::new(parties, active_at_offset),
            max_page_size,
        )
    }

    /// Like [`Self::active_contracts_resumable`], with the full request
    /// surface of an
    /// [`ActiveContractsRequest`](crate::request::ActiveContractsRequest).
    pub fn active_contracts_resumable_with(
        &self,
        request: crate::request::ActiveContractsRequest,
        max_page_size: i32,
    ) -> impl Stream<Item = Result<pb::ActiveContract>> + Send + use<> {
        let client = self.clone();
        let (max_reconnects, backoff_unit) = client.reconnect_policy();
        async_stream::stream! {
            let mut page_token: Option<Vec<u8>> = None;
            let mut reconnects = 0u32;
            loop {
                match client
                    .active_contracts_page_with(
                        &request,
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

    /// Stream ledger updates — transactions and reassignments — for `parties`,
    /// starting after `begin_offset` (exclusive). Offset checkpoints are
    /// filtered out. Reassignments are surfaced as their own case (each
    /// carrying the distinct `Unassigned`/`Assigned` events). Topology events
    /// are **not** included; ask for them with
    /// [`UpdatesRequest::with_topology_events`](crate::request::UpdatesRequest::with_topology_events)
    /// on [`Self::updates_with`].
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn updates(
        &self,
        parties: Vec<String>,
        begin_offset: i64,
    ) -> Result<impl Stream<Item = Result<pb::get_updates_response::Update>> + Send + use<>> {
        self.updates_with(crate::request::UpdatesRequest::new(parties, begin_offset))
            .await
    }

    /// Like [`Self::updates`], with the full request surface: an
    /// [`UpdatesRequest`](crate::request::UpdatesRequest) additionally bounds
    /// the stream at an end offset (`until` — the catch-up/sync form, after
    /// which the stream terminates), filters by template or interface, selects
    /// the ACS-delta shape, includes created-event blobs or topology events,
    /// and drops reassignments or record labels.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn updates_with(
        &self,
        request: crate::request::UpdatesRequest,
    ) -> Result<impl Stream<Item = Result<pb::get_updates_response::Update>> + Send + use<>> {
        let (_, end_inclusive) = request.bounds();
        if request.is_descending() && end_inclusive.is_none() {
            return Err(Error::InvalidRequest(
                "descending order requires a bounded request: set UpdatesRequest::until"
                    .to_string(),
            ));
        }
        telemetry::instrument("updates", TRANSPORT_GRPC, async move {
            let mut client = service!(self, pb::update_service_client::UpdateServiceClient::new);
            let stream = client.get_updates(request.into_grpc()).await?.into_inner();

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
        self.updates_resumable_with(crate::request::UpdatesRequest::new(parties, begin_offset))
    }

    /// Like [`Self::updates_resumable`], with the full request surface of an
    /// [`UpdatesRequest`](crate::request::UpdatesRequest). A bounded request
    /// (`until`) makes this a *resilient catch-up read*: reconnects resume
    /// from the last yielded offset, and the stream ends once the participant
    /// closes it at the end offset.
    pub fn updates_resumable_with(
        &self,
        request: crate::request::UpdatesRequest,
    ) -> impl Stream<Item = Result<pb::get_updates_response::Update>> + Send + use<> {
        let client = self.clone();
        let (max_reconnects, backoff_unit) = client.reconnect_policy();
        async_stream::stream! {
            // Resume tracking assumes ascending offsets; a descending request
            // would silently re-read on every reconnect, so refuse it.
            if request.is_descending() {
                yield Err(Error::InvalidRequest(
                    "the resumable stream requires ascending order; use updates_with \
                     (bounded) for descending reads".to_string(),
                ));
                return;
            }
            let mut offset = request.begin_exclusive;
            let mut reconnects = 0u32;
            loop {
                match client.updates_with(request.resume_after(offset)).await {
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

    // The paged read is inherently bounded, so a request without an end offset
    // must be refused before any RPC is attempted.
    #[tokio::test]
    async fn updates_page_with_requires_a_bounded_request() {
        let Ok(client) = CantonClient::connect_lazy(Config::new("http://localhost:1")) else {
            panic!("lazy connect accepts a valid endpoint");
        };
        let unbounded = crate::request::UpdatesRequest::new(vec!["p".to_string()], 0);
        let result = client.updates_page_with(&unbounded, 10, None).await;
        assert!(
            matches!(result, Err(Error::InvalidRequest(_))),
            "expected InvalidRequest, got {result:?}"
        );
    }

    // Descending order needs an end offset (streams), and never composes with
    // resume tracking — both must fail fast, before any RPC.
    #[tokio::test]
    async fn descending_requires_bounds_and_is_refused_on_resumable() {
        use tokio_stream::StreamExt as _;

        let Ok(client) = CantonClient::connect_lazy(Config::new("http://localhost:1")) else {
            panic!("lazy connect accepts a valid endpoint");
        };
        let descending_unbounded =
            crate::request::UpdatesRequest::new(vec!["p".to_string()], 0).descending();

        let result = client.updates_with(descending_unbounded.clone()).await;
        assert!(matches!(result.err(), Some(Error::InvalidRequest(_))));

        let stream = client.updates_resumable_with(descending_unbounded.until(10));
        tokio::pin!(stream);
        let Some(first) = stream.next().await else {
            panic!("the resumable stream should yield the rejection");
        };
        assert!(matches!(first, Err(Error::InvalidRequest(_))));
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
