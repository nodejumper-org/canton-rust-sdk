//! Command construction (dynamic / untyped).
//!
//! Until the M2 code generator produces typed bindings, commands are built from
//! the wire protobuf types directly. These helpers keep that construction
//! readable; the generated typed API will layer over the same wire values.

use canton_proto::com::daml::ledger::api::v2 as pb;

/// A ledger command submission: which parties act, which commands to run, and
/// the submission metadata (change ID, de-duplication period, workflow).
///
/// The command id is one component of the change ID
/// (`user_id`, `act_as`, `command_id`) that the Ledger API de-duplicates on.
/// Leave it unset for a fresh UUID, or set it explicitly to make a submission
/// idempotent across retries. `user_id` defaults to the one derived from the
/// bearer token.
#[derive(Clone, Debug)]
pub struct Submit {
    pub(crate) act_as: Vec<String>,
    pub(crate) commands: Vec<pb::Command>,
    pub(crate) command_id: Option<String>,
    pub(crate) user_id: Option<String>,
    pub(crate) read_as: Vec<String>,
    pub(crate) workflow_id: Option<String>,
    pub(crate) synchronizer_id: Option<String>,
    pub(crate) deduplication: Option<pb::commands::DeduplicationPeriod>,
    pub(crate) transaction_shape: crate::request::TransactionShape,
    pub(crate) submission_id: Option<String>,
    pub(crate) disclosed_contracts: Vec<pb::DisclosedContract>,
    pub(crate) package_id_selection_preference: Vec<String>,
    pub(crate) min_ledger_time_abs: Option<prost_types::Timestamp>,
    pub(crate) min_ledger_time_rel: Option<std::time::Duration>,
    pub(crate) prefetch_contract_keys: Vec<pb::PrefetchContractKey>,
    pub(crate) taps_max_passes: Option<u32>,
}

/// The complete identity of a submitted command — Canton's **change ID**.
///
/// The Ledger API does not identify a command by its `command_id` alone: it
/// de-duplicates on the triple (`user_id`, `act_as`, `command_id`), and two
/// applications sharing a participant can legitimately use the same command id.
/// Anything that goes looking for a command's outcome afterwards — a completion
/// after a lost response, say — has to match on all three or it can find
/// somebody else's answer.
///
/// A [`Submission`](crate::Submission) hands one of these back *before* the
/// command is sent, which is the point: after an ambiguous failure the id is
/// the only way back to the outcome, and a generated id the caller never saw
/// is no way back at all.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChangeId {
    user_id: String,
    act_as: Vec<String>,
    command_id: String,
}

impl ChangeId {
    /// Assemble a change ID from its three parts.
    ///
    /// An empty `user_id` means "whichever user the bearer token resolves to",
    /// which is what the participant fills in when a submission leaves it
    /// unset — see [`Self::matches`].
    #[must_use]
    pub fn new(
        user_id: impl Into<String>,
        act_as: Vec<String>,
        command_id: impl Into<String>,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            act_as,
            command_id: command_id.into(),
        }
    }

    /// The command id.
    #[must_use]
    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    /// The submitting user id; empty when it was left to the token.
    #[must_use]
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// The acting parties.
    #[must_use]
    pub fn act_as(&self) -> &[String] {
        &self.act_as
    }

    /// Whether `completion` is the completion of *this* command.
    ///
    /// The command id must match, and the acting parties must be the same set —
    /// order is not meaningful, and Canton may echo them in its own.
    ///
    /// Two deliberate asymmetries. A `user_id` this side left empty is not
    /// compared: the participant resolved it from the token and the client
    /// genuinely does not know it, so requiring equality would reject every
    /// completion. And a completion that carries no `act_as` at all is not
    /// rejected on that ground — some rejections are reported without one, and
    /// refusing to recognise a rejection is worse than the narrow risk of a
    /// same-user, same-command-id collision it guards against.
    #[must_use]
    pub fn matches(&self, completion: &pb::Completion) -> bool {
        self.matches_parts(
            &completion.command_id,
            &completion.user_id,
            &completion.act_as,
        )
    }

    /// [`Self::matches`] for a completion from the JSON transport, which
    /// carries the same three fields under their camelCase names.
    ///
    /// The rule lives here rather than in the JSON client so that the two
    /// transports cannot come to disagree about what identifies a command.
    #[must_use]
    pub fn matches_json(&self, completion: &serde_json::Value) -> bool {
        let command_id = completion
            .get("commandId")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        let user_id = completion
            .get("userId")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        let act_as: Vec<String> = completion
            .get("actAs")
            .and_then(serde_json::Value::as_array)
            .map(|parties| {
                parties
                    .iter()
                    .filter_map(|party| party.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        self.matches_parts(command_id, user_id, &act_as)
    }

    fn matches_parts(&self, command_id: &str, user_id: &str, act_as: &[String]) -> bool {
        if command_id != self.command_id {
            return false;
        }
        if !self.user_id.is_empty() && user_id != self.user_id {
            return false;
        }
        if act_as.is_empty() {
            return true;
        }
        let normalise = |parties: &[String]| {
            let mut parties: Vec<String> = parties.to_vec();
            parties.sort_unstable();
            parties.dedup();
            parties
        };
        normalise(act_as) == normalise(&self.act_as)
    }
}

impl Submit {
    /// Start a submission acting as a single party.
    #[must_use]
    pub fn new(act_as: impl Into<String>) -> Self {
        Self::new_multi(vec![act_as.into()])
    }

    /// Start a submission acting as multiple parties (multi-party
    /// authorization, e.g. proposal-accept or DvP patterns).
    #[must_use]
    pub fn new_multi(act_as: Vec<String>) -> Self {
        Self {
            act_as,
            commands: Vec::new(),
            command_id: None,
            user_id: None,
            read_as: Vec::new(),
            workflow_id: None,
            synchronizer_id: None,
            deduplication: None,
            transaction_shape: crate::request::TransactionShape::default(),
            submission_id: None,
            disclosed_contracts: Vec::new(),
            package_id_selection_preference: Vec::new(),
            min_ledger_time_abs: None,
            min_ledger_time_rel: None,
            prefetch_contract_keys: Vec::new(),
            taps_max_passes: None,
        }
    }

    /// Add a command to the submission.
    #[must_use]
    pub fn add_command(mut self, command: pb::Command) -> Self {
        self.commands.push(command);
        self
    }

    /// Set an explicit command id (for de-duplication / retry idempotency).
    #[must_use]
    pub fn with_command_id(mut self, command_id: impl Into<String>) -> Self {
        self.command_id = Some(command_id.into());
        self
    }

    /// Set the acting user id — the first component of the change ID. Defaults
    /// to the user derived from the bearer token.
    #[must_use]
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Add read-as parties (data visible to these parties may be read during
    /// interpretation).
    #[must_use]
    pub fn with_read_as(mut self, read_as: Vec<String>) -> Self {
        self.read_as = read_as;
        self
    }

    /// Set the workflow id (an opaque correlation id carried on the resulting
    /// transaction).
    #[must_use]
    pub fn with_workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    /// Pin the submission to a specific synchronizer.
    #[must_use]
    pub fn with_synchronizer_id(mut self, synchronizer_id: impl Into<String>) -> Self {
        self.synchronizer_id = Some(synchronizer_id.into());
        self
    }

    /// Set the de-duplication period as a wall-clock duration: a submission
    /// with the same change ID within this window is rejected as a duplicate.
    #[must_use]
    pub fn with_deduplication_duration(mut self, duration: std::time::Duration) -> Self {
        self.deduplication = Some(pb::commands::DeduplicationPeriod::DeduplicationDuration(
            prost_types::Duration {
                seconds: i64::try_from(duration.as_secs()).unwrap_or(i64::MAX),
                nanos: i32::try_from(duration.subsec_nanos()).unwrap_or(0),
            },
        ));
        self
    }

    /// Set the de-duplication period as a ledger offset: submissions with the
    /// same change ID since that offset are rejected as duplicates.
    #[must_use]
    pub fn with_deduplication_offset(mut self, offset: i64) -> Self {
        self.deduplication = Some(pb::commands::DeduplicationPeriod::DeduplicationOffset(
            offset,
        ));
        self
    }

    /// Select the shape of the transaction returned by
    /// [`submit_and_wait_for_transaction`] (default:
    /// [`TransactionShape::LedgerEffects`], the full as-executed view;
    /// [`TransactionShape::AcsDelta`] returns the net create/archive change).
    /// Ignored by the submission-only paths, which return no transaction.
    ///
    /// [`submit_and_wait_for_transaction`]: crate::CantonClient::submit_and_wait_for_transaction
    /// [`TransactionShape::LedgerEffects`]: crate::request::TransactionShape::LedgerEffects
    /// [`TransactionShape::AcsDelta`]: crate::request::TransactionShape::AcsDelta
    #[must_use]
    pub fn with_transaction_shape(mut self, shape: crate::request::TransactionShape) -> Self {
        self.transaction_shape = shape;
        self
    }

    /// Set an explicit submission id, to correlate this particular submission
    /// attempt in completions (unlike the command id, it identifies one
    /// attempt, not the change). Defaults to participant-generated.
    #[must_use]
    pub fn with_submission_id(mut self, submission_id: impl Into<String>) -> Self {
        self.submission_id = Some(submission_id.into());
        self
    }

    /// Attach a disclosed contract: an off-ledger contract (obtained as a
    /// `created_event_blob`, e.g. via
    /// `UpdatesRequest::with_created_event_blobs` /
    /// `ActiveContractsRequest::with_created_event_blobs`) made readable to
    /// this submission's interpretation. May be called repeatedly.
    #[must_use]
    pub fn add_disclosed_contract(mut self, contract: pb::DisclosedContract) -> Self {
        self.disclosed_contracts.push(contract);
        self
    }

    /// Restrict package selection for interpretation to these package ids
    /// (at most one preference per package name) — the SCU upgrade pin.
    #[must_use]
    pub fn with_package_id_selection_preference(mut self, package_ids: Vec<String>) -> Self {
        self.package_id_selection_preference = package_ids;
        self
    }

    /// Set the lower bound for the ledger-effective time as an absolute
    /// timestamp (mutually exclusive with
    /// [`Self::with_min_ledger_time_rel`] — the participant rejects both).
    #[must_use]
    pub fn with_min_ledger_time_abs(mut self, time: prost_types::Timestamp) -> Self {
        self.min_ledger_time_abs = Some(time);
        self
    }

    /// Set the lower bound for the ledger-effective time relative to the
    /// participant's local clock (mutually exclusive with
    /// [`Self::with_min_ledger_time_abs`]).
    #[must_use]
    pub fn with_min_ledger_time_rel(mut self, duration: std::time::Duration) -> Self {
        self.min_ledger_time_rel = Some(duration);
        self
    }

    /// Hint contract keys to resolve eagerly before interpretation (a
    /// performance knob for key-heavy workflows).
    #[must_use]
    pub fn with_prefetch_contract_keys(mut self, keys: Vec<pb::PrefetchContractKey>) -> Self {
        self.prefetch_contract_keys = keys;
        self
    }

    /// Cap the topology-aware package selection passes (defaults to the
    /// participant's configured value).
    #[must_use]
    pub fn with_taps_max_passes(mut self, passes: u32) -> Self {
        self.taps_max_passes = Some(passes);
        self
    }

    /// Build the wire [`pb::Commands`], filling `command_id` with a fresh UUID
    /// when the caller did not set one. Returns the [`ChangeId`] alongside, so
    /// the identity of the command is known before it is sent and can be used
    /// for completion-based recovery afterwards.
    pub(crate) fn into_commands(self) -> (ChangeId, pb::Commands) {
        let command_id = self
            .command_id
            .unwrap_or_else(|| format!("sdk-{}", uuid::Uuid::new_v4()));
        let change_id = ChangeId::new(
            self.user_id.clone().unwrap_or_default(),
            self.act_as.clone(),
            command_id.clone(),
        );
        let commands = pb::Commands {
            command_id: command_id.clone(),
            act_as: self.act_as,
            read_as: self.read_as,
            user_id: self.user_id.unwrap_or_default(),
            workflow_id: self.workflow_id.unwrap_or_default(),
            synchronizer_id: self.synchronizer_id.unwrap_or_default(),
            commands: self.commands,
            deduplication_period: self.deduplication,
            submission_id: self.submission_id.unwrap_or_default(),
            disclosed_contracts: self.disclosed_contracts,
            package_id_selection_preference: self.package_id_selection_preference,
            min_ledger_time_abs: self.min_ledger_time_abs,
            min_ledger_time_rel: self.min_ledger_time_rel.map(|d| prost_types::Duration {
                seconds: i64::try_from(d.as_secs()).unwrap_or(i64::MAX),
                nanos: i32::try_from(d.subsec_nanos()).unwrap_or(0),
            }),
            prefetch_contract_keys: self.prefetch_contract_keys,
            taps_max_passes: self.taps_max_passes,
        };
        (change_id, commands)
    }
}

/// Build a create command for `template_id` with the given `arguments` record.
#[must_use]
pub fn create(template_id: pb::Identifier, arguments: pb::Record) -> pb::Command {
    pb::Command {
        command: Some(pb::command::Command::Create(pb::CreateCommand {
            template_id: Some(template_id),
            create_arguments: Some(arguments),
        })),
    }
}

/// Build an exercise command: exercise `choice` (with `argument`) on the
/// contract `contract_id` of type `template_id`.
#[must_use]
pub fn exercise(
    template_id: pb::Identifier,
    contract_id: impl Into<String>,
    choice: impl Into<String>,
    argument: pb::Value,
) -> pb::Command {
    pb::Command {
        command: Some(pb::command::Command::Exercise(pb::ExerciseCommand {
            template_id: Some(template_id),
            contract_id: contract_id.into(),
            choice: choice.into(),
            choice_argument: Some(argument),
        })),
    }
}

/// A template/type identifier (`package_id`, `Module.Path`, `EntityName`).
#[must_use]
pub fn identifier(
    package_id: impl Into<String>,
    module_name: impl Into<String>,
    entity_name: impl Into<String>,
) -> pb::Identifier {
    pb::Identifier {
        package_id: package_id.into(),
        module_name: module_name.into(),
        entity_name: entity_name.into(),
    }
}

/// A record value from labelled fields.
#[must_use]
pub fn record(fields: Vec<(&str, pb::Value)>) -> pb::Record {
    pb::Record {
        record_id: None,
        fields: fields
            .into_iter()
            .map(|(label, value)| pb::RecordField {
                label: label.to_string(),
                value: Some(value),
            })
            .collect(),
    }
}

/// Value constructors for the dynamic command path.
pub mod value {
    use canton_proto::com::daml::ledger::api::v2 as pb;

    fn wrap(sum: pb::value::Sum) -> pb::Value {
        pb::Value { sum: Some(sum) }
    }

    /// A `Party` value.
    #[must_use]
    pub fn party(party: impl Into<String>) -> pb::Value {
        wrap(pb::value::Sum::Party(party.into()))
    }

    /// A `Text` value.
    #[must_use]
    pub fn text(text: impl Into<String>) -> pb::Value {
        wrap(pb::value::Sum::Text(text.into()))
    }

    /// A nested record value.
    #[must_use]
    pub fn record(record: pb::Record) -> pb::Value {
        wrap(pb::value::Sum::Record(record))
    }

    /// A `TextMap` (`Map Text a`) value from key/value pairs.
    #[must_use]
    pub fn text_map(entries: Vec<(&str, pb::Value)>) -> pb::Value {
        wrap(pb::value::Sum::TextMap(pb::TextMap {
            entries: entries
                .into_iter()
                .map(|(key, value)| pb::text_map::Entry {
                    key: key.to_string(),
                    value: Some(value),
                })
                .collect(),
        }))
    }

    /// An empty `TextMap` value (e.g. an empty Splice `Metadata`).
    #[must_use]
    pub fn empty_text_map() -> pb::Value {
        text_map(Vec::new())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn create_builds_a_create_command_with_template_and_args() {
        let command = create(
            identifier("pkg-1", "Licensing.AppInstall", "AppInstallRequest"),
            record(vec![("owner", value::party("alice"))]),
        );

        let Some(pb::command::Command::Create(create_cmd)) = command.command else {
            panic!("expected a create command");
        };
        let template = create_cmd.template_id.unwrap();
        assert_eq!(template.package_id, "pkg-1");
        assert_eq!(template.module_name, "Licensing.AppInstall");
        assert_eq!(template.entity_name, "AppInstallRequest");

        let args = create_cmd.create_arguments.unwrap();
        assert_eq!(args.fields.len(), 1);
        assert_eq!(args.fields[0].label, "owner");
        assert!(matches!(
            args.fields[0].value.as_ref().unwrap().sum,
            Some(pb::value::Sum::Party(_))
        ));
    }

    #[test]
    fn exercise_builds_an_exercise_command() {
        let command = exercise(
            identifier("pkg-1", "M", "T"),
            "cid-1",
            "Accept",
            value::record(record(vec![])),
        );
        let Some(pb::command::Command::Exercise(ex)) = command.command else {
            panic!("expected an exercise command");
        };
        assert_eq!(ex.contract_id, "cid-1");
        assert_eq!(ex.choice, "Accept");
        assert_eq!(ex.template_id.unwrap().entity_name, "T");
        assert!(ex.choice_argument.is_some());
    }

    #[test]
    fn submit_collects_multiple_commands_in_order() {
        let submit = Submit::new("alice")
            .add_command(create(identifier("p", "M", "A"), record(vec![])))
            .add_command(create(identifier("p", "M", "B"), record(vec![])));
        assert_eq!(submit.commands.len(), 2);
    }

    #[test]
    fn submit_builder_collects_parties_commands_and_id() {
        let submit = Submit::new("alice")
            .with_command_id("cmd-42")
            .add_command(create(identifier("p", "M", "E"), record(vec![])));

        assert_eq!(submit.act_as, vec!["alice".to_string()]);
        assert_eq!(submit.command_id.as_deref(), Some("cmd-42"));
        assert_eq!(submit.commands.len(), 1);
    }

    fn completion(command_id: &str, user_id: &str, act_as: &[&str]) -> pb::Completion {
        pb::Completion {
            command_id: command_id.to_string(),
            user_id: user_id.to_string(),
            act_as: act_as.iter().map(|p| (*p).to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn a_change_id_matches_only_its_own_completion() {
        let change_id = ChangeId::new("app-1", vec!["alice".to_string()], "cmd-1");

        assert!(change_id.matches(&completion("cmd-1", "app-1", &["alice"])));
        // Same command id, different application on the same participant: this
        // is the collision that matching on the command id alone answers wrongly.
        assert!(!change_id.matches(&completion("cmd-1", "app-2", &["alice"])));
        // Same command id, different acting party.
        assert!(!change_id.matches(&completion("cmd-1", "app-1", &["bob"])));
        assert!(!change_id.matches(&completion("cmd-2", "app-1", &["alice"])));
    }

    #[test]
    fn the_json_matcher_reads_the_same_identity_from_camel_case() {
        let change_id = ChangeId::new("app-1", vec!["alice".to_string()], "cmd-1");
        let completion = |command_id: &str, user_id: &str, act_as: &[&str]| {
            serde_json::json!({
                "commandId": command_id,
                "userId": user_id,
                "actAs": act_as,
            })
        };

        assert!(change_id.matches_json(&completion("cmd-1", "app-1", &["alice"])));
        assert!(!change_id.matches_json(&completion("cmd-1", "app-2", &["alice"])));
        assert!(!change_id.matches_json(&completion("cmd-1", "app-1", &["bob"])));
        assert!(!change_id.matches_json(&completion("cmd-2", "app-1", &["alice"])));
        // Order is not meaningful over either transport.
        let unordered = ChangeId::new(
            "app-1",
            vec!["alice".to_string(), "bob".to_string()],
            "cmd-1",
        );
        assert!(unordered.matches_json(&completion("cmd-1", "app-1", &["bob", "alice"])));
    }

    #[test]
    fn the_two_transports_cannot_disagree_about_what_identifies_a_command() {
        // The rule lives in one place; this is the assertion that says so. If
        // the JSON reader ever grows its own opinion, these diverge.
        let change_id = ChangeId::new("app-1", vec!["alice".to_string()], "cmd-1");
        for (command_id, user_id, act_as) in [
            ("cmd-1", "app-1", vec!["alice"]),
            ("cmd-1", "app-2", vec!["alice"]),
            ("cmd-1", "app-1", vec!["bob"]),
            ("cmd-2", "app-1", vec!["alice"]),
            ("cmd-1", "app-1", vec!["alice", "bob"]),
        ] {
            let grpc = completion(command_id, user_id, &act_as);
            let json = serde_json::json!({
                "commandId": command_id, "userId": user_id, "actAs": act_as,
            });
            assert_eq!(
                change_id.matches(&grpc),
                change_id.matches_json(&json),
                "transports disagree on ({command_id}, {user_id}, {act_as:?})"
            );
        }
    }

    #[test]
    fn a_json_completion_missing_its_fields_is_read_the_same_way_as_grpc() {
        // Absent `actAs` reads as empty, which the shared rule treats as
        // "unknown, do not reject" — the same as an empty repeated field on
        // the wire.
        let change_id = ChangeId::new("", vec!["alice".to_string()], "cmd-1");
        assert!(change_id.matches_json(&serde_json::json!({ "commandId": "cmd-1" })));
        assert!(!change_id.matches_json(&serde_json::json!({ "commandId": "other" })));
        // A body that is not a completion at all matches nothing.
        assert!(!change_id.matches_json(&serde_json::json!({})));
    }

    #[test]
    fn acting_parties_are_a_set_not_a_sequence() {
        let change_id = ChangeId::new(
            "app-1",
            vec!["alice".to_string(), "bob".to_string()],
            "cmd-1",
        );
        assert!(change_id.matches(&completion("cmd-1", "app-1", &["bob", "alice"])));
        assert!(!change_id.matches(&completion("cmd-1", "app-1", &["alice"])));
    }

    #[test]
    fn an_unknown_user_id_is_not_compared() {
        // Left to the bearer token: the participant resolved it and the client
        // never learned which user that was, so requiring equality here would
        // reject the very completion it is looking for.
        let change_id = ChangeId::new("", vec!["alice".to_string()], "cmd-1");
        assert!(change_id.matches(&completion("cmd-1", "whoever", &["alice"])));

        // A completion reported without acting parties still matches: some
        // rejections arrive that way, and failing to recognise a rejection is
        // the worse outcome.
        assert!(change_id.matches(&completion("cmd-1", "whoever", &[])));
    }

    #[test]
    fn into_commands_wires_every_field_and_generates_an_id() {
        let disclosed = pb::DisclosedContract {
            contract_id: "cid-1".to_string(),
            ..Default::default()
        };
        let (change_id, commands) = Submit::new("alice")
            .with_user_id("user-1")
            .with_read_as(vec!["bob".to_string()])
            .with_workflow_id("wf-1")
            .with_synchronizer_id("sync-1")
            .with_deduplication_duration(std::time::Duration::from_secs(30))
            .with_submission_id("sub-1")
            .add_disclosed_contract(disclosed)
            .with_package_id_selection_preference(vec!["pkg-1".to_string()])
            .with_min_ledger_time_rel(std::time::Duration::from_millis(1500))
            .with_prefetch_contract_keys(vec![pb::PrefetchContractKey::default()])
            .with_taps_max_passes(3)
            .add_command(create(identifier("p", "M", "E"), record(vec![])))
            .into_commands();

        assert!(
            change_id.command_id().starts_with("sdk-"),
            "generated uuid id"
        );
        assert_eq!(commands.command_id, change_id.command_id());
        assert_eq!(change_id.user_id(), "user-1");
        assert_eq!(change_id.act_as(), ["alice".to_string()]);
        assert_eq!(commands.act_as, vec!["alice".to_string()]);
        assert_eq!(commands.read_as, vec!["bob".to_string()]);
        assert_eq!(commands.user_id, "user-1");
        assert_eq!(commands.workflow_id, "wf-1");
        assert_eq!(commands.synchronizer_id, "sync-1");
        assert_eq!(commands.commands.len(), 1);
        assert_eq!(commands.submission_id, "sub-1");
        assert_eq!(commands.disclosed_contracts.len(), 1);
        assert_eq!(commands.disclosed_contracts[0].contract_id, "cid-1");
        assert_eq!(
            commands.package_id_selection_preference,
            vec!["pkg-1".to_string()]
        );
        let Some(rel) = commands.min_ledger_time_rel else {
            panic!("expected a relative min ledger time");
        };
        assert_eq!((rel.seconds, rel.nanos), (1, 500_000_000));
        assert_eq!(commands.prefetch_contract_keys.len(), 1);
        assert_eq!(commands.taps_max_passes, Some(3));
        let Some(pb::commands::DeduplicationPeriod::DeduplicationDuration(d)) =
            commands.deduplication_period
        else {
            panic!("expected a deduplication duration");
        };
        assert_eq!(d.seconds, 30);
    }

    #[test]
    fn into_commands_wires_an_absolute_min_ledger_time() {
        let (_, commands) = Submit::new("alice")
            .with_min_ledger_time_abs(prost_types::Timestamp {
                seconds: 1_700_000_000,
                nanos: 0,
            })
            .into_commands();
        let Some(abs) = commands.min_ledger_time_abs else {
            panic!("expected an absolute min ledger time");
        };
        assert_eq!(abs.seconds, 1_700_000_000);
    }

    #[test]
    fn new_multi_carries_every_acting_party() {
        let (_, commands) = Submit::new_multi(vec!["a".to_string(), "b".to_string()])
            .add_command(create(identifier("p", "M", "E"), record(vec![])))
            .into_commands();
        assert_eq!(commands.act_as, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn into_commands_preserves_an_explicit_id_and_offset_dedup() {
        let (change_id, commands) = Submit::new("alice")
            .with_command_id("cmd-7")
            .with_deduplication_offset(42)
            .into_commands();

        assert_eq!(change_id.command_id(), "cmd-7");
        // Left to the token: the participant resolves it, so the client cannot
        // record a user id it does not know.
        assert!(change_id.user_id().is_empty());
        assert_eq!(commands.command_id, "cmd-7");
        assert!(matches!(
            commands.deduplication_period,
            Some(pb::commands::DeduplicationPeriod::DeduplicationOffset(42))
        ));
        // Unset optionals stay empty (token-derived user id, no workflow).
        assert!(commands.user_id.is_empty());
        assert!(commands.workflow_id.is_empty());
    }

    #[test]
    fn empty_text_map_is_a_textmap_with_no_entries() {
        let Some(pb::value::Sum::TextMap(map)) = value::empty_text_map().sum else {
            panic!("expected a text map value");
        };
        assert!(map.entries.is_empty());
    }

    #[test]
    fn text_map_preserves_entries() {
        let Some(pb::value::Sum::TextMap(map)) = value::text_map(vec![("k", value::text("v"))]).sum
        else {
            panic!("expected a text map value");
        };
        assert_eq!(map.entries.len(), 1);
        assert_eq!(map.entries[0].key, "k");
    }
}
