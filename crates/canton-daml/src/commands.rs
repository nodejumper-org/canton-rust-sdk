//! Builders that turn typed templates and choices into Ledger API commands.
//!
//! These construct the `Command` values the client crate (`canton-ledger`)
//! submits — they perform no I/O. A [`Template`] payload becomes a
//! `CreateCommand`; a [`Choice`] argument exercised on a [`ContractId`] becomes
//! an `ExerciseCommand`. Both use the template's upgrade-friendly
//! [`Template::template_id`] (the `#<package-name>` form).

use canton_proto::com::daml::ledger::api::v2 as pb;

use crate::choice::Choice;
use crate::primitives::ContractId;
use crate::template::{Contract, Template, WithKey};
use crate::value::ToValue;

/// A `CreateCommand` for a new contract of template `T` from its payload.
#[must_use]
pub fn create_command<T: Template>(payload: &T) -> pb::Command {
    wrap(pb::command::Command::Create(pb::CreateCommand {
        template_id: Some(T::template_id()),
        create_arguments: Some(into_record(payload.to_value())),
    }))
}

/// An `ExerciseCommand` exercising choice `C` (with `argument`) on the contract
/// `contract_id` of template or interface `T`.
#[must_use]
pub fn exercise_command<T, C>(contract_id: &ContractId<T>, argument: &C) -> pb::Command
where
    T: Contract,
    C: Choice<T> + ToValue,
{
    wrap(pb::command::Command::Exercise(pb::ExerciseCommand {
        template_id: Some(T::template_id()),
        contract_id: contract_id.as_str().to_string(),
        choice: C::NAME.to_string(),
        choice_argument: Some(argument.to_value()),
    }))
}

/// An `ExerciseByKeyCommand` exercising choice `C` (with `argument`) on the
/// contract of template `T` identified by its contract `key`.
#[must_use]
pub fn exercise_by_key_command<T, C>(key: &T::Key, argument: &C) -> pb::Command
where
    T: WithKey,
    C: Choice<T> + ToValue,
{
    wrap(pb::command::Command::ExerciseByKey(
        pb::ExerciseByKeyCommand {
            template_id: Some(T::template_id()),
            contract_key: Some(key.to_value()),
            choice: C::NAME.to_string(),
            choice_argument: Some(argument.to_value()),
        },
    ))
}

/// Wrap a command variant in the top-level `Command` envelope.
fn wrap(command: pb::command::Command) -> pb::Command {
    pb::Command {
        command: Some(command),
    }
}

/// A record `Value` (which a template payload always is) unwrapped to the bare
/// `Record` the Ledger API create argument expects.
fn into_record(value: pb::Value) -> pb::Record {
    match value.sum {
        Some(pb::value::Sum::Record(record)) => record,
        _ => pb::Record::default(),
    }
}
