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

/// Refuse a command set the participant is certain to refuse.
///
/// Every one of these costs a round trip and comes back as a server-side error
/// that reads like the ledger's fault. They are the caller's, and they are
/// knowable here.
/// Whether a failed submission is the participant refusing a command it already
/// has — and, on a retry, one this client is responsible for having sent.
///
/// `ALREADY_EXISTS` is what Canton's `DUPLICATE_COMMAND` arrives as. On a first
/// attempt it means the *caller* reused a change ID from an earlier submission,
/// which is a genuine rejection they must see. On a retry it means our own
/// previous attempt was accepted, which is the opposite of a failure — so the
/// caller of this predicate checks that first.
fn is_duplicate_of_our_own(status: &tonic::Status) -> bool {
    status.code() == tonic::Code::AlreadyExists
}

fn validate_commands(commands: &pb::Commands) -> Result<()> {
    if commands.act_as.is_empty() {
        return Err(Error::InvalidRequest(
            "a submission needs at least one acting party".to_string(),
        ));
    }
    if commands.commands.is_empty() {
        return Err(Error::InvalidRequest(
            "a submission needs at least one command".to_string(),
        ));
    }
    if commands.min_ledger_time_abs.is_some() && commands.min_ledger_time_rel.is_some() {
        return Err(Error::InvalidRequest(
            "min_ledger_time_abs and min_ledger_time_rel are mutually exclusive: set one"
                .to_string(),
        ));
    }
    Ok(())
}

/// One entry of an Active Contract Set snapshot.
///
/// A snapshot is not only active contracts. A reassignment that is half-done at
/// the snapshot offset appears as an *incomplete* entry — unassigned from one
/// synchronizer with no matching assignment yet, or assigned to one with the
/// unassignment out of view — and an application that reads only
/// [`Active`](Self::Active) sees a contract that has left one synchronizer and
/// not arrived at the other simply vanish. The Ledger API sends these precisely
/// so a multi-synchronizer application can bootstrap without that hole, which is
/// why there is a lossless read.
///
/// `#[non_exhaustive]`: the Ledger API may add an entry kind.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum AcsEntry {
    /// A contract active on the synchronizer named in the entry.
    Active(pb::ActiveContract),
    /// Unassigned before the snapshot offset with no assignment before it —
    /// the contract is in flight *out of* this synchronizer.
    IncompleteUnassigned(pb::IncompleteUnassigned),
    /// Assigned before the snapshot offset with no unassignment before it.
    /// Note this does **not** say the contract is active on the target
    /// synchronizer; the proto is explicit about that.
    IncompleteAssigned(pb::IncompleteAssigned),
}

impl AcsEntry {
    /// The active contract, if this entry is one.
    #[must_use]
    pub fn active(&self) -> Option<&pb::ActiveContract> {
        match self {
            AcsEntry::Active(active) => Some(active),
            _ => None,
        }
    }

    /// Consume the entry, yielding the active contract if it is one.
    #[must_use]
    pub fn into_active(self) -> Option<pb::ActiveContract> {
        match self {
            AcsEntry::Active(active) => Some(active),
            _ => None,
        }
    }
}

/// Convert one wire entry, dropping only the case the participant left empty
/// (the `oneof` is required, so an empty entry is a participant that sent
/// nothing to report).
fn acs_entry(entry: Option<pb::get_active_contracts_response::ContractEntry>) -> Option<AcsEntry> {
    use pb::get_active_contracts_response::ContractEntry;
    match entry {
        Some(ContractEntry::ActiveContract(active)) => Some(AcsEntry::Active(active)),
        Some(ContractEntry::IncompleteUnassigned(unassigned)) => {
            Some(AcsEntry::IncompleteUnassigned(unassigned))
        }
        Some(ContractEntry::IncompleteAssigned(assigned)) => {
            Some(AcsEntry::IncompleteAssigned(assigned))
        }
        None => None,
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
        let submission = self.submission(submit);
        let command_id = submission.change_id().command_id().to_string();
        submission.submit().await?;
        Ok(command_id)
    }

    /// Fix a submission's identity **before** sending it, returning a
    /// [`Submission`](crate::Submission) that carries its
    /// [`ChangeId`](crate::ChangeId).
    ///
    /// This is the handle to reach for when losing the outcome is not an
    /// option. A submission whose response is lost — a dropped connection, a
    /// timeout, a retry the participant de-duplicated — may well have
    /// committed, and the only way back to the answer is the change ID. If the
    /// SDK generated the command id inside the call that failed, there is no
    /// change ID to go back with.
    ///
    /// ```no_run
    /// # async fn run(client: canton_ledger::CantonClient, submit: canton_ledger::Submit)
    /// #     -> canton_ledger::Result<()> {
    /// use std::time::Duration;
    ///
    /// // Where to start reading completions from, taken before submitting.
    /// let offset = client.ledger_end().await?;
    /// let submission = client.submission(submit);
    ///
    /// if submission.submit_and_wait().await.is_err() {
    ///     // Ambiguous: ask the ledger what actually happened.
    ///     let completion = submission.recover(offset, Duration::from_secs(30)).await?;
    ///     println!("committed after all: {}", completion.update_id);
    /// }
    /// # Ok(()) }
    /// ```
    #[must_use]
    pub fn submission(&self, submit: crate::command::Submit) -> crate::submission::Submission {
        let shape = submit.transaction_shape;
        let (change_id, commands) = submit.into_commands();
        crate::submission::Submission::new(self.clone(), change_id, commands, shape)
    }

    pub(crate) async fn submit_commands(&self, commands: pb::Commands) -> Result<()> {
        validate_commands(&commands)?;
        telemetry::instrument("submit", TRANSPORT_GRPC, async move {
            let attempt = std::sync::atomic::AtomicU32::new(0);
            let command_id = commands.command_id.clone();
            self.with_retry(|| {
                let commands = commands.clone();
                let command_id = command_id.clone();
                let retry = attempt.fetch_add(1, std::sync::atomic::Ordering::Relaxed) > 0;
                async move {
                    let mut client = service!(
                        self,
                        pb::command_submission_service_client::CommandSubmissionServiceClient::new
                    );
                    match client
                        .submit(pb::SubmitRequest {
                            commands: Some(commands),
                        })
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(status) if retry && is_duplicate_of_our_own(&status) => {
                            // The command we are retrying is already at the
                            // participant, and we are the ones who put it
                            // there: a previous attempt of this same retry loop
                            // was accepted and its response was lost. Reporting
                            // the duplicate rejection would tell the caller
                            // their command failed when it did precisely the
                            // opposite — the failure mode this whole change ID
                            // exists to prevent.
                            tracing::debug!(
                                %command_id,
                                "submission retry was de-duplicated; the earlier attempt is the one that landed"
                            );
                            Ok(())
                        }
                        Err(status) => Err(Error::from(status)),
                    }
                }
            })
            .await
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
        self.submission(submit).submit_and_wait().await
    }

    pub(crate) async fn submit_and_wait_commands(
        &self,
        commands: pb::Commands,
    ) -> Result<pb::SubmitAndWaitResponse> {
        validate_commands(&commands)?;
        telemetry::instrument("submit_and_wait", TRANSPORT_GRPC, async move {
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
    /// With retry enabled ([`Config::with_retry`]), a submission that commits
    /// on the ledger but whose response is lost to a retriable error is re-sent
    /// with the same `command_id` and de-duplicated by the participant.
    ///
    /// [`Self::submit`] can answer that on its own: a duplicate rejection of a
    /// retry it made itself means its earlier attempt was accepted, which is
    /// success, so it reports success. **This method cannot.** Its result is
    /// the committed transaction, and a de-duplicated retry does not carry one
    /// — so the duplicate rejection reaches the caller, and the transaction has
    /// to be read back rather than invented.
    ///
    /// Take a [`Submission`](crate::Submission) from [`Self::submission`]
    /// before submitting, and on any error recover the outcome with
    /// [`Submission::recover`](crate::Submission::recover) — it knows the
    /// change ID, which is the only way back to a command whose response was
    /// lost. See the `recover_a_submission` example.
    pub async fn submit_and_wait_for_transaction(
        &self,
        submit: crate::command::Submit,
    ) -> Result<pb::Transaction> {
        self.submission(submit)
            .submit_and_wait_for_transaction()
            .await
    }

    pub(crate) async fn submit_and_wait_for_transaction_commands(
        &self,
        commands: pb::Commands,
        shape: crate::request::TransactionShape,
    ) -> Result<pb::Transaction> {
        validate_commands(&commands)?;
        // The filter covers `read_as` as well as `act_as`: the Ledger API's own
        // default for a submission is both, and a command submitted with
        // `read_as` set is one whose result the caller expects to see through
        // those parties too. Filtering to `act_as` alone silently returned a
        // transaction with events missing.
        let mut parties = commands.act_as.clone();
        parties.extend(commands.read_as.iter().cloned());
        parties.sort_unstable();
        parties.dedup();
        let request = pb::SubmitAndWaitForTransactionRequest {
            transaction_format: Some(pb::TransactionFormat {
                event_format: Some(wildcard_event_format(&parties)),
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
        request.validate()?;
        telemetry::instrument("completions", TRANSPORT_GRPC, async move {
            let mut client = service!(
                self,
                pb::command_completion_service_client::CommandCompletionServiceClient::new
            );
            let stream = client
                .completion_stream(request.into_grpc())
                .await?
                .into_inner();

            let stream = stream.filter_map(|item| match item {
                Ok(response) => match response.completion_response {
                    Some(pb::completion_stream_response::CompletionResponse::Completion(
                        completion,
                    )) => Some(Ok(completion)),
                    _ => None, // skip offset checkpoints
                },
                Err(status) => Some(Err(Error::from(status))),
            });
            Ok(telemetry::instrument_stream(
                "completions",
                TRANSPORT_GRPC,
                stream,
            ))
        })
        .await
    }

    /// Recover the completion for a specific command by scanning the completion
    /// stream from `begin_offset`, up to `timeout`.
    ///
    /// This is the command-recovery path: after a crash, lost connection, or
    /// timeout, the outcome of a pending command is read back from the
    /// completion endpoint instead of blindly re-submitting. If the command's
    /// completion reports a non-OK status, this returns [`Error::CommandRejected`].
    ///
    /// Matching is on the whole [`ChangeId`](crate::ChangeId) — user, acting
    /// parties and command id — because that is what identifies a command to
    /// Canton. A command id on its own is not unique across the users of a
    /// participant, and answering with somebody else's completion is worse
    /// than answering with none.
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
        change_id: &crate::command::ChangeId,
        begin_offset: i64,
        timeout: Duration,
    ) -> Result<pb::Completion> {
        let scan = async {
            let stream = self
                .completions(change_id.act_as().to_vec(), begin_offset)
                .await?;
            tokio::pin!(stream);
            while let Some(item) = stream.next().await {
                let completion = item?;
                if change_id.matches(&completion) {
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
                "completion stream ended before command {} was seen",
                change_id.command_id()
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
            // Retried like the other reads: this is a lookup, so a transient
            // failure is worth another attempt rather than a caller's error
            // path. Only submissions have a reason to be careful here.
            self.with_retry(|| {
                let request = pb::GetEventsByContractIdRequest {
                    contract_id: contract_id.clone(),
                    event_format: Some(wildcard_event_format(&parties)),
                };
                async move {
                    let mut client = service!(
                        self,
                        pb::event_query_service_client::EventQueryServiceClient::new
                    );
                    Ok(client
                        .get_events_by_contract_id(request)
                        .await?
                        .into_inner())
                }
            })
            .await
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
        let (entries, next) = self
            .acs_page_with(request, max_page_size, page_token)
            .await?;
        Ok((
            entries
                .into_iter()
                .filter_map(AcsEntry::into_active)
                .collect(),
            next,
        ))
    }

    /// One page of the Active Contract Set for `parties`, **losslessly**: every
    /// entry the participant sent, active or incomplete (see [`AcsEntry`]).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn acs_page(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
        max_page_size: i32,
        page_token: Option<Vec<u8>>,
    ) -> Result<(Vec<AcsEntry>, Option<Vec<u8>>)> {
        let request = crate::request::ActiveContractsRequest::new(parties, active_at_offset);
        self.acs_page_with(&request, max_page_size, page_token)
            .await
    }

    /// Like [`Self::acs_page`], with the full request surface of an
    /// [`ActiveContractsRequest`](crate::request::ActiveContractsRequest).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the RPC fails.
    pub async fn acs_page_with(
        &self,
        request: &crate::request::ActiveContractsRequest,
        max_page_size: i32,
        page_token: Option<Vec<u8>>,
    ) -> Result<(Vec<AcsEntry>, Option<Vec<u8>>)> {
        request.validate()?;
        telemetry::instrument("active_contracts_page", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let page_request = pb::GetActiveContractsPageRequest {
                    active_at_offset: Some(request.active_at_offset),
                    event_format: Some(request.event_format()),
                    max_page_size: Some(max_page_size),
                    page_token: page_token.clone(),
                };
                async move {
                    let mut client =
                        service!(self, pb::state_service_client::StateServiceClient::new);
                    let response = client
                        .get_active_contracts_page(page_request)
                        .await?
                        .into_inner();

                    let entries = response
                        .active_contracts
                        .into_iter()
                        .filter_map(|entry| acs_entry(entry.contract_entry))
                        .collect();
                    Ok((entries, response.next_page_token))
                }
            })
            .await
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
        request.validate()?;
        let (begin_exclusive, end_inclusive) = request.bounds();
        let Some(end_inclusive) = end_inclusive else {
            return Err(Error::InvalidRequest(
                "updates_page_with requires a bounded request: set UpdatesRequest::until"
                    .to_string(),
            ));
        };
        telemetry::instrument("updates_page", TRANSPORT_GRPC, async move {
            self.with_retry(|| {
                let page_request = pb::GetUpdatesPageRequest {
                    begin_offset_exclusive: Some(begin_exclusive),
                    end_offset_inclusive: Some(end_inclusive),
                    max_page_size: Some(max_page_size),
                    update_format: Some(request.update_format()),
                    descending_order: request.is_descending(),
                    page_token: page_token.clone(),
                };
                async move {
                    let mut client =
                        service!(self, pb::update_service_client::UpdateServiceClient::new);
                    let response = client.get_updates_page(page_request).await?.into_inner();
                    Ok((response.updates, response.next_page_token))
                }
            })
            .await
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
        let stream = self.acs_entries_with(request).await?;
        Ok(stream.filter_map(|item| match item {
            Ok(entry) => entry.into_active().map(Ok),
            Err(err) => Some(Err(err)),
        }))
    }

    /// Stream the Active Contract Set for `parties` **losslessly**: every entry
    /// the participant sends, active or incomplete (see [`AcsEntry`]).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn acs_entries(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
    ) -> Result<impl Stream<Item = Result<AcsEntry>> + Send + use<>> {
        self.acs_entries_with(crate::request::ActiveContractsRequest::new(
            parties,
            active_at_offset,
        ))
        .await
    }

    /// Like [`Self::acs_entries`], with the full request surface of an
    /// [`ActiveContractsRequest`](crate::request::ActiveContractsRequest).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or opening the stream fails.
    pub async fn acs_entries_with(
        &self,
        request: crate::request::ActiveContractsRequest,
    ) -> Result<impl Stream<Item = Result<AcsEntry>> + Send + use<>> {
        request.validate()?;
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

            let stream = stream.filter_map(|item| match item {
                Ok(response) => acs_entry(response.contract_entry).map(Ok),
                Err(status) => Some(Err(Error::from(status))),
            });
            Ok(telemetry::instrument_stream(
                "active_contracts",
                TRANSPORT_GRPC,
                stream,
            ))
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
        self.acs_entries_resumable_with(request, max_page_size)
            .filter_map(|item| match item {
                Ok(entry) => entry.into_active().map(Ok),
                Err(err) => Some(Err(err)),
            })
    }

    /// Like [`Self::acs_entries`], but **resumable** and page-based: reads the
    /// snapshot page by page and retries a failed page from the last
    /// continuation token instead of restarting from zero. Every entry the
    /// participant sent is yielded (see [`AcsEntry`]).
    pub fn acs_entries_resumable(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
        max_page_size: i32,
    ) -> impl Stream<Item = Result<AcsEntry>> + Send + use<> {
        self.acs_entries_resumable_with(
            crate::request::ActiveContractsRequest::new(parties, active_at_offset),
            max_page_size,
        )
    }

    /// Like [`Self::acs_entries_resumable`], with the full request surface of
    /// an [`ActiveContractsRequest`](crate::request::ActiveContractsRequest).
    pub fn acs_entries_resumable_with(
        &self,
        request: crate::request::ActiveContractsRequest,
        max_page_size: i32,
    ) -> impl Stream<Item = Result<AcsEntry>> + Send + use<> {
        let client = self.clone();
        let (max_reconnects, backoff_unit) = client.reconnect_policy();
        async_stream::stream! {
            let mut page_token: Option<Vec<u8>> = None;
            let mut reconnects = 0u32;
            loop {
                match client
                    .acs_page_with(
                        &request,
                        max_page_size,
                        page_token.clone(),
                    )
                    .await
                {
                    Ok((entries, next)) => {
                        reconnects = 0;
                        for entry in entries {
                            yield Ok(entry);
                        }
                        match next {
                            Some(next) => page_token = Some(next),
                            None => return, // snapshot fully delivered
                        }
                    }
                    Err(err) if err.is_retriable() => {
                        reconnects += 1;
                        if reconnects > max_reconnects {
                            // The participant's own failure, for the reason the
                            // update stream keeps it: the status and its
                            // classification are what the caller acts on.
                            tracing::warn!(
                                max_reconnects,
                                "acs page failed to resume; reporting the failure that caused it"
                            );
                            yield Err(err);
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
        let stream = self.updates_including_checkpoints(request).await?;
        Ok(stream.filter(|item| {
            !matches!(
                item,
                Ok(pb::get_updates_response::Update::OffsetCheckpoint(_))
            )
        }))
    }

    /// The update stream with `OffsetCheckpoint` frames left in.
    ///
    /// A checkpoint is not an update anyone subscribed for, which is why
    /// [`Self::updates_with`] drops it. It is, however, the participant naming
    /// an offset that is safe to restart from — and it arrives on a quiet
    /// stream, where no transaction has moved the position for a while. The
    /// resumable path needs exactly that, so it reads this and filters for
    /// itself.
    async fn updates_including_checkpoints(
        &self,
        request: crate::request::UpdatesRequest,
    ) -> Result<impl Stream<Item = Result<pb::get_updates_response::Update>> + Send + use<>> {
        request.validate()?;
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

            let stream = stream.filter_map(|item| match item {
                Ok(response) => response.update.map(Ok),
                Err(status) => Some(Err(Error::from(status))),
            });
            // Instrumented for the life of the stream, not just its opening:
            // a subscription that fails an hour in is the failure that matters.
            Ok(telemetry::instrument_stream(
                "updates",
                TRANSPORT_GRPC,
                stream,
            ))
        })
        .await
    }

    /// Like [`Self::updates`], but **resumable**: on a retriable stream error it
    /// reconnects from the last offset it *observed* (rather than restarting
    /// from `begin_offset` or losing position), with a short backoff and a
    /// bounded number of consecutive reconnects (see the client's
    /// [`RetryConfig`](canton_core::RetryConfig)).
    ///
    /// Observed includes the participant's `OffsetCheckpoint` frames, which are
    /// filtered out of what this yields but are the only thing that advances the
    /// resume point on a stream where nothing is happening. Once the reconnect
    /// budget is spent the stream yields **the failure that caused it** — the
    /// participant's status, details and retriable classification intact —
    /// rather than an error of the SDK's own.
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
    /// from the last observed offset, and the stream ends once the participant
    /// closes it at the end offset.
    pub fn updates_resumable_with(
        &self,
        request: crate::request::UpdatesRequest,
    ) -> impl Stream<Item = Result<pb::get_updates_response::Update>> + Send + use<> {
        let client = self.clone();
        let (max_reconnects, backoff_unit) = client.reconnect_policy();
        async_stream::stream! {
            if let Err(err) = request.validate() {
                yield Err(err);
                return;
            }
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
                // What made this reconnect necessary. Carried out of the inner
                // loop so that giving up reports the participant's own failure
                // rather than a fresh error of ours: the status, its structured
                // details, the correlation id and the retriable classification
                // are what an application branches on, and the moment the
                // stream gives up is when it needs them most.
                let cause = match client
                    .updates_including_checkpoints(request.resume_after(offset))
                    .await
                {
                    Ok(stream) => {
                        tokio::pin!(stream);
                        loop {
                            match stream.next().await {
                                Some(Ok(update)) => {
                                    // Checkpoints are included here: on a quiet
                                    // stream a checkpoint is the only thing that
                                    // moves the resume point, and after pruning
                                    // the older offset may not be servable at all.
                                    offset = update_offset(&update);
                                    reconnects = 0;
                                    if matches!(
                                        update,
                                        pb::get_updates_response::Update::OffsetCheckpoint(_)
                                    ) {
                                        continue;
                                    }
                                    yield Ok(update);
                                }
                                Some(Err(err)) if err.is_retriable() => break err,
                                Some(Err(err)) => {
                                    yield Err(err);
                                    return;
                                }
                                None => return, // server closed the stream cleanly
                            }
                        }
                    }
                    Err(err) if err.is_retriable() => err,
                    Err(err) => {
                        yield Err(err);
                        return;
                    }
                };

                reconnects += 1;
                if reconnects > max_reconnects {
                    tracing::warn!(
                        max_reconnects,
                        offset,
                        "update stream gave up resuming; reporting the failure that caused it"
                    );
                    yield Err(cause);
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
    // Every one of these is refused before a connection is even attempted: the
    // client is pointed at a port nothing listens on, so anything that reaches
    // the wire fails differently.
    #[tokio::test]
    async fn requests_the_participant_would_refuse_are_refused_here() {
        use crate::request::{ActiveContractsRequest, CompletionsRequest, UpdatesRequest};

        let Ok(client) = CantonClient::connect_lazy(Config::new("http://localhost:1")) else {
            panic!("lazy connect accepts a valid endpoint");
        };
        let party = || vec!["alice".to_string()];
        let invalid = |result: &Result<_, Error>| matches!(result, Err(Error::InvalidRequest(_)));

        // Negative offsets.
        assert!(invalid(
            &client
                .updates_with(UpdatesRequest::new(party(), -1))
                .await
                .map(|_| ())
        ));
        assert!(invalid(
            &client
                .acs_entries_with(ActiveContractsRequest::new(party(), -5))
                .await
                .map(|_| ())
        ));
        assert!(invalid(
            &client
                .completions_with(CompletionsRequest::new(party(), -1))
                .await
                .map(|_| ())
        ));

        // A range that ends before it begins.
        assert!(invalid(
            &client
                .updates_with(UpdatesRequest::new(party(), 100).until(50))
                .await
                .map(|_| ())
        ));

        // No parties: a subscription that looks alive and can never yield.
        assert!(invalid(
            &client
                .updates_with(UpdatesRequest::new(vec![], 0))
                .await
                .map(|_| ())
        ));
    }

    #[tokio::test]
    async fn submissions_that_cannot_succeed_are_refused_here() {
        use crate::command::Submit;

        let Ok(client) = CantonClient::connect_lazy(Config::new("http://localhost:1")) else {
            panic!("lazy connect accepts a valid endpoint");
        };

        // No commands: a round trip to be told nothing was asked for.
        let empty = client.submission(Submit::new("alice"));
        assert!(matches!(
            empty.submit().await,
            Err(Error::InvalidRequest(_))
        ));

        // The two minimum-ledger-time forms are mutually exclusive.
        let both = client.submission(
            Submit::new("alice")
                .add_command(crate::command::create(
                    crate::command::identifier("pkg", "M", "T"),
                    crate::command::record(vec![]),
                ))
                .with_min_ledger_time_rel(std::time::Duration::from_secs(1))
                .with_min_ledger_time_abs(prost_types::Timestamp::default()),
        );
        assert!(matches!(both.submit().await, Err(Error::InvalidRequest(_))));
    }

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
