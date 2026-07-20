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
}

impl Submit {
    /// Start a submission acting as a single party.
    pub fn new(act_as: impl Into<String>) -> Self {
        Self {
            act_as: vec![act_as.into()],
            commands: Vec::new(),
            command_id: None,
            user_id: None,
            read_as: Vec::new(),
            workflow_id: None,
            synchronizer_id: None,
            deduplication: None,
        }
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

    /// Build the wire [`pb::Commands`], filling `command_id` with a fresh UUID
    /// when the caller did not set one. Returns `(command_id, commands)` so the
    /// caller can hand the change ID back for completion-based recovery.
    pub(crate) fn into_commands(self) -> (String, pb::Commands) {
        let command_id = self
            .command_id
            .unwrap_or_else(|| format!("sdk-{}", uuid::Uuid::new_v4()));
        let commands = pb::Commands {
            command_id: command_id.clone(),
            act_as: self.act_as,
            read_as: self.read_as,
            user_id: self.user_id.unwrap_or_default(),
            workflow_id: self.workflow_id.unwrap_or_default(),
            synchronizer_id: self.synchronizer_id.unwrap_or_default(),
            commands: self.commands,
            deduplication_period: self.deduplication,
            ..Default::default()
        };
        (command_id, commands)
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

    #[test]
    fn into_commands_wires_every_field_and_generates_an_id() {
        let (command_id, commands) = Submit::new("alice")
            .with_user_id("user-1")
            .with_read_as(vec!["bob".to_string()])
            .with_workflow_id("wf-1")
            .with_synchronizer_id("sync-1")
            .with_deduplication_duration(std::time::Duration::from_secs(30))
            .add_command(create(identifier("p", "M", "E"), record(vec![])))
            .into_commands();

        assert!(command_id.starts_with("sdk-"), "generated uuid id");
        assert_eq!(commands.command_id, command_id);
        assert_eq!(commands.act_as, vec!["alice".to_string()]);
        assert_eq!(commands.read_as, vec!["bob".to_string()]);
        assert_eq!(commands.user_id, "user-1");
        assert_eq!(commands.workflow_id, "wf-1");
        assert_eq!(commands.synchronizer_id, "sync-1");
        assert_eq!(commands.commands.len(), 1);
        let Some(pb::commands::DeduplicationPeriod::DeduplicationDuration(d)) =
            commands.deduplication_period
        else {
            panic!("expected a deduplication duration");
        };
        assert_eq!(d.seconds, 30);
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
        let (command_id, commands) = Submit::new("alice")
            .with_command_id("cmd-7")
            .with_deduplication_offset(42)
            .into_commands();

        assert_eq!(command_id, "cmd-7");
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
