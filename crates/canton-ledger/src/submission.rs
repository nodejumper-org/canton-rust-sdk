//! A submission whose identity is known before it is sent.
//!
//! Submitting a command is not a request whose failure means "it did not
//! happen". A dropped connection, a timeout, or a retry the participant
//! de-duplicated all produce an error while the command may have committed
//! perfectly well. The only way back to the outcome is the
//! [`ChangeId`](crate::ChangeId), read from the completion stream — so the
//! change ID has to exist *before* the submission, not be a value returned by
//! the call that failed.

use std::time::Duration;

use canton_proto::com::daml::ledger::api::v2 as pb;

use crate::client::CantonClient;
use crate::command::ChangeId;
use crate::request::TransactionShape;
use canton_core::Result;

/// A command with its identity fixed, ready to send and to recover.
///
/// Built by [`CantonClient::submission`]. The three send methods mirror the
/// client's own; [`Submission::recover`] is the one that has no equivalent
/// without a handle, because it needs the change ID the send did not return.
#[derive(Clone, Debug)]
pub struct Submission {
    client: CantonClient,
    change_id: ChangeId,
    commands: pb::Commands,
    shape: TransactionShape,
}

impl Submission {
    pub(crate) fn new(
        client: CantonClient,
        change_id: ChangeId,
        commands: pb::Commands,
        shape: TransactionShape,
    ) -> Self {
        Self {
            client,
            change_id,
            commands,
            shape,
        }
    }

    /// The command's complete identity — available before anything is sent.
    #[must_use]
    pub fn change_id(&self) -> &ChangeId {
        &self.change_id
    }

    /// Submit without waiting (`CommandSubmissionService.Submit`).
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if authentication or the RPC
    /// fails. A failure here is *ambiguous*: use [`Self::recover`].
    pub async fn submit(&self) -> Result<()> {
        self.client.submit_commands(self.commands.clone()).await
    }

    /// Submit and wait for the completion (`CommandService.SubmitAndWait`).
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if authentication fails or the
    /// command is rejected. A transport failure is ambiguous: use
    /// [`Self::recover`].
    pub async fn submit_and_wait(&self) -> Result<pb::SubmitAndWaitResponse> {
        self.client
            .submit_and_wait_commands(self.commands.clone())
            .await
    }

    /// Submit and wait for the committed transaction.
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if authentication fails, the
    /// command is rejected, or the response carries no transaction. A transport
    /// failure is ambiguous: use [`Self::recover`].
    pub async fn submit_and_wait_for_transaction(&self) -> Result<pb::Transaction> {
        self.client
            .submit_and_wait_for_transaction_commands(self.commands.clone(), self.shape)
            .await
    }

    /// Read this command's outcome back from the completion stream, matching on
    /// the whole change ID.
    ///
    /// `begin_offset` must be an offset from **before** the submission —
    /// [`CantonClient::ledger_end`] taken beforehand is the usual source, since
    /// a completion that has already gone past cannot be read again.
    ///
    /// # Errors
    /// Returns [`Error::Timeout`](canton_core::Error::Timeout) if no completion
    /// arrives within `timeout` (which, for a command that never reached the
    /// participant, is the correct answer), or
    /// [`Error::CommandRejected`](canton_core::Error::CommandRejected) if the
    /// ledger rejected it.
    pub async fn recover(&self, begin_offset: i64, timeout: Duration) -> Result<pb::Completion> {
        self.client
            .await_completion(&self.change_id, begin_offset, timeout)
            .await
    }
}

/// A JSON-transport command with its identity fixed, ready to send and to
/// recover — the JSON lane's [`Submission`].
///
/// Built by [`JsonClient::submission`](crate::JsonClient::submission).
#[derive(Clone, Debug)]
pub struct JsonSubmission {
    client: crate::JsonClient,
    commands: crate::JsonCommands,
    change_id: ChangeId,
}

impl JsonSubmission {
    pub(crate) fn new(client: crate::JsonClient, commands: crate::JsonCommands) -> Self {
        let change_id = commands.change_id();
        Self {
            client,
            commands,
            change_id,
        }
    }

    /// The command's complete identity — available before anything is sent.
    #[must_use]
    pub fn change_id(&self) -> &ChangeId {
        &self.change_id
    }

    /// Submit without waiting (`POST /v2/commands/async/submit`).
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if authentication or the
    /// request fails. A failure is *ambiguous*: use [`Self::recover`].
    pub async fn submit(&self) -> Result<()> {
        self.client.submit(&self.commands).await
    }

    /// Submit and wait for the completion.
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if authentication fails or the
    /// command is rejected.
    pub async fn submit_and_wait(&self) -> Result<crate::JsonSubmitAndWaitResponse> {
        self.client.submit_and_wait(&self.commands).await
    }

    /// Submit and wait for the committed transaction.
    ///
    /// # Errors
    /// Returns an [`Error`](canton_core::Error) if authentication fails or the
    /// command is rejected.
    pub async fn submit_and_wait_for_transaction(&self) -> Result<crate::JsonSubmitResponse> {
        self.client
            .submit_and_wait_for_transaction(&self.commands)
            .await
    }

    /// Read this command's outcome back from the completion stream, matching on
    /// the whole change ID.
    ///
    /// The JSON transport carries completions over the WebSocket, so this needs
    /// the `ws` feature. `begin_offset` must be from **before** the submission;
    /// [`JsonClient::ledger_end`](crate::JsonClient::ledger_end) taken
    /// beforehand is the usual source.
    ///
    /// # Errors
    /// Returns [`Error::Timeout`](canton_core::Error::Timeout) if no completion
    /// arrives within `timeout`, or
    /// [`Error::CommandRejected`](canton_core::Error::CommandRejected) if the
    /// ledger rejected the command.
    #[cfg(feature = "ws")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub async fn recover(&self, begin_offset: i64, timeout: Duration) -> Result<serde_json::Value> {
        use canton_core::Error;
        use tokio_stream::StreamExt as _;

        let scan = async {
            let stream = self
                .client
                .ws_completions(self.change_id.act_as().to_vec(), begin_offset)
                .await?;
            tokio::pin!(stream);
            while let Some(item) = stream.next().await {
                let frame = item?;
                let Some(completion) = crate::ws::completion_value(&frame) else {
                    continue;
                };
                if !self.change_id.matches_json(completion) {
                    continue;
                }
                // A non-OK status on the completion is the ledger rejecting the
                // command, which is an answer — the same one the gRPC lane
                // reports as `CommandRejected`.
                if let Some(status) = completion.get("status") {
                    let code = status.get("code").and_then(serde_json::Value::as_i64);
                    if code.is_some_and(|code| code != 0) {
                        return Err(Error::CommandRejected {
                            code: code.unwrap_or_default().to_string(),
                            message: status
                                .get("message")
                                .and_then(serde_json::Value::as_str)
                                .unwrap_or_default()
                                .to_string(),
                        });
                    }
                }
                return Ok(completion.clone());
            }
            Err(Error::UnexpectedResponse(format!(
                "completion stream ended before command {} was seen",
                self.change_id.command_id()
            )))
        };

        tokio::time::timeout(timeout, scan)
            .await
            .map_err(|_| canton_core::Error::Timeout)?
    }
}
