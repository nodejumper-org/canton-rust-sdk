//! Reading a local development network out of the environment.
//!
//! [canton-devkit] runs a Splice LocalNet — two participants and a
//! super-validator — and exports everything an application needs to talk to it:
//!
//! ```text
//! eval "$(canton-devkit localnet env demo)"   # or: dpm localnet env demo
//! ```
//!
//! That sets a documented set of `CANTON_*` variables, and this module is the
//! one place in the SDK that knows their names. [`Config::from_env`] turns them
//! into a working gRPC configuration, so an application that would otherwise
//! carry endpoint and token plumbing for local development carries none:
//!
//! ```no_run
//! # async fn run() -> canton_core::Result<()> {
//! let config = canton_core::Config::from_env()?;
//! # let _ = config;
//! # Ok(()) }
//! ```
//!
//! # The variables
//!
//! | Variable | Meaning |
//! |---|---|
//! | `CANTON_GRPC_LEDGER_API_URL` | gRPC Ledger API, **scheme-less** `host:port` |
//! | `CANTON_JSON_LEDGER_API_URL` | JSON Ledger API, with an `http(s)://` scheme |
//! | `CANTON_<ROLE>_JWT` | that role's bearer token |
//! | `CANTON_<ALIAS>_PARTY` | an on-ledger party id |
//! | `CANTON_INSTANCE`, `CANTON_SPLICE_VERSION` | which network this is |
//!
//! The unqualified URL variables point at the **app-provider** participant, the
//! usual target for an application. The other participants are reached by role:
//! `CANTON_APP_USER_GRPC_LEDGER_API_URL`, `CANTON_SV_JWT`, and so on.
//! [`Config::from_env_for`] takes the role name and applies the same
//! normalisation the exporter does (upper-case, `-` becomes `_`), so
//! `"app-user"` and `"app_user"` both work.
//!
//! # Two things worth knowing
//!
//! The URLs are **nginx virtual-host names** —
//! `grpc-ledger-api.app-provider.demo.localhost` — not plain hosts. The name is
//! what routes the request, so it has to survive into the `:authority` (gRPC) or
//! `Host` (HTTP) header rather than being resolved away by the caller. Passing
//! the URL through unchanged, as everything here does, is what keeps that true.
//! `*.localhost` resolves to loopback on macOS and on Linux with
//! systemd-resolved; where it does not, an `/etc/hosts` entry is the fix —
//! substituting `127.0.0.1` is not, because the vhost is then lost.
//!
//! The gRPC URL has **no scheme**, because that is what a gRPC client dials.
//! [`Config`] accepts it that way.
//!
//! # Anything else that sets the same names
//!
//! Nothing here is devkit-specific beyond the variable names, and two generic
//! overrides come first for environments that are not a LocalNet at all:
//! `CANTON_ENDPOINT` and `CANTON_TOKEN` win over everything below them.
//!
//! [canton-devkit]: https://github.com/bitdynamics-ab/canton-devkit

use crate::{Config, Error, Result};

/// `CANTON_ENDPOINT` — an explicit gRPC endpoint, overriding the LocalNet one.
const ENDPOINT_OVERRIDE: &str = "CANTON_ENDPOINT";
/// `CANTON_TOKEN` — an explicit bearer token, overriding the role's JWT.
const TOKEN_OVERRIDE: &str = "CANTON_TOKEN";
/// The role the unqualified URL variables point at.
const DEFAULT_ROLE: &str = "app-provider";

/// Where the variables come from.
///
/// The process environment in normal use, a map in the tests. Reading through
/// this rather than calling `std::env::var` directly is what lets the tests
/// cover the precedence rules at all: setting a variable is `unsafe` as of the
/// 2024 edition and this workspace forbids `unsafe`, and tests that mutate the
/// process environment would have to run under a lock to avoid seeing each
/// other's values.
struct Source<F: Fn(&str) -> Option<String>>(F);

impl<F: Fn(&str) -> Option<String>> Source<F> {
    /// Read `name`, treating a set-but-empty value as absent.
    ///
    /// `std::env::var` reports `FOO=` as `Ok("")`. That is how an exporter
    /// spells "not known yet", and taking it as a value produces a client that
    /// authenticates with an empty token and then fails every call with a
    /// permission error naming nothing.
    fn get(&self, name: &str) -> Option<String> {
        (self.0)(name).filter(|value| !value.trim().is_empty())
    }

    fn grpc_endpoint(&self, role: Option<&str>) -> Option<String> {
        self.get(ENDPOINT_OVERRIDE)
            .or_else(|| self.get(&role_variable(role, "GRPC_LEDGER_API_URL")))
    }

    fn json_endpoint(&self, role: Option<&str>) -> Option<String> {
        self.get(&role_variable(role, "JSON_LEDGER_API_URL"))
    }

    fn token(&self, role: Option<&str>) -> Option<String> {
        self.get(TOKEN_OVERRIDE)
            .or_else(|| self.get(&format!("{}_JWT", prefix(role.unwrap_or(DEFAULT_ROLE)))))
    }

    fn config(&self, role: Option<&str>) -> Result<Config> {
        let endpoint = self
            .grpc_endpoint(role)
            .ok_or_else(|| missing_endpoint(role))?;
        let config = Config::new(endpoint);
        Ok(match self.token(role) {
            Some(token) => config.with_token(token),
            // A LocalNet started without authentication exports no JWT. That is
            // a network, not a misconfiguration.
            None => config,
        })
    }
}

/// The process environment.
fn process() -> Source<impl Fn(&str) -> Option<String>> {
    Source(|name: &str| std::env::var(name).ok())
}

/// `CANTON_` + a role or alias, upper-cased with `-` turned into `_`.
///
/// Mirrors the exporter's own normalisation, so `"app-user"`, `"app_user"` and
/// `"APP_USER"` all name the same variables.
fn prefix(role: &str) -> String {
    format!("CANTON_{}", role.to_uppercase().replace('-', "_"))
}

/// `CANTON_<ROLE>_<SUFFIX>`, or the unqualified `CANTON_<SUFFIX>` when no role
/// is named — which is how the exporter spells "the default participant".
fn role_variable(role: Option<&str>, suffix: &str) -> String {
    match role {
        None => format!("CANTON_{suffix}"),
        Some(role) => format!("{}_{suffix}", prefix(role)),
    }
}

/// The error for "there is no network in this environment", written so the
/// reader knows which command produces one.
fn missing_endpoint(role: Option<&str>) -> Error {
    let variable = role_variable(role, "GRPC_LEDGER_API_URL");
    Error::InvalidRequest(format!(
        "no ledger endpoint in the environment: set {variable} (or {ENDPOINT_OVERRIDE}). \
         A local network exports it with `canton-devkit localnet env <instance>`; \
         run that through `eval` first."
    ))
}

/// The gRPC Ledger API URL for `role`, or the default participant's when `role`
/// is `None`. `CANTON_ENDPOINT` overrides both.
#[must_use]
pub fn grpc_endpoint(role: Option<&str>) -> Option<String> {
    process().grpc_endpoint(role)
}

/// The JSON Ledger API base URL for `role`, or the default participant's when
/// `role` is `None`.
#[must_use]
pub fn json_endpoint(role: Option<&str>) -> Option<String> {
    process().json_endpoint(role)
}

/// The bearer token for `role`, or the default participant's when `role` is
/// `None`. `CANTON_TOKEN` overrides both.
///
/// These are development credentials issued by the LocalNet's own issuer, with
/// a long life and no revocation. They are not something to carry into a
/// deployment.
#[must_use]
pub fn token(role: Option<&str>) -> Option<String> {
    process().token(role)
}

/// The on-ledger party id recorded under `alias` — a role (`"app-provider"`) or
/// a name given to a party created later (`"bob"`).
///
/// The party **id**, not the Ledger API user name: this is what goes in
/// `act_as`.
#[must_use]
pub fn party(alias: &str) -> Option<String> {
    process().get(&format!("{}_PARTY", prefix(alias)))
}

/// The instance name the environment was exported from (`CANTON_INSTANCE`).
#[must_use]
pub fn instance() -> Option<String> {
    process().get("CANTON_INSTANCE")
}

/// The Splice release the network is running (`CANTON_SPLICE_VERSION`).
#[must_use]
pub fn splice_version() -> Option<String> {
    process().get("CANTON_SPLICE_VERSION")
}

impl Config {
    /// Build a configuration from a LocalNet exported into the environment —
    /// the **app-provider** participant, with its token.
    ///
    /// See the [module documentation](self) for the variables read and the two
    /// things worth knowing about the URLs.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] when no endpoint variable is set,
    /// naming the variable and the command that exports it. A missing token is
    /// **not** an error: an unauthenticated LocalNet is a normal thing to point
    /// this at.
    pub fn from_env() -> Result<Self> {
        process().config(None)
    }

    /// The same, for a participant other than the default: `"app-user"`,
    /// `"sv"`, or any role the exporter knows.
    ///
    /// # Errors
    /// As [`Config::from_env`], naming that role's variable.
    pub fn from_env_for(role: &str) -> Result<Self> {
        process().config(Some(role))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::Auth;
    use std::collections::HashMap;

    /// The exact strings `canton-devkit localnet env` prints — vhost names, and
    /// a gRPC URL with no scheme. Copied from the exporter rather than
    /// paraphrased: paraphrasing is how the scheme gets quietly added and the
    /// thing this has to cope with quietly disappears.
    fn devkit() -> Source<impl Fn(&str) -> Option<String>> {
        let vars: HashMap<&str, &str> = [
            ("CANTON_INSTANCE", "demo"),
            ("CANTON_SPLICE_VERSION", "0.6.12"),
            (
                "CANTON_GRPC_LEDGER_API_URL",
                "grpc-ledger-api.app-provider.demo.localhost:3901",
            ),
            (
                "CANTON_JSON_LEDGER_API_URL",
                "http://json-ledger-api.app-provider.demo.localhost:3901",
            ),
            (
                "CANTON_APP_USER_GRPC_LEDGER_API_URL",
                "grpc-ledger-api.app-user.demo.localhost:2901",
            ),
            ("CANTON_APP_PROVIDER_JWT", "provider.jwt.token"),
            ("CANTON_APP_USER_JWT", "user.jwt.token"),
            ("CANTON_APP_PROVIDER_PARTY", "app_provider::1220abcd"),
            ("CANTON_BOB_PARTY", "bob::1220abcd"),
            // Left over from an earlier shell, which is the realistic state.
            ("CANTON_ENDPOINT", ""),
            ("CANTON_TOKEN", ""),
        ]
        .into_iter()
        .collect();
        Source(move |name: &str| vars.get(name).map(|value| (*value).to_string()))
    }

    #[test]
    fn a_devkit_environment_becomes_a_working_configuration() {
        let config = devkit().config(None).unwrap();

        // The vhost name has to survive. It is what routes the request through
        // nginx, so rewriting it to 127.0.0.1 would reach the port and be
        // refused by the virtual host.
        assert_eq!(
            config.endpoint(),
            "grpc-ledger-api.app-provider.demo.localhost:3901"
        );
        assert!(matches!(config.auth(), Auth::Static(t) if t == "provider.jwt.token"));

        // The scheme-less form is stored as exported. Supplying the scheme is
        // the channel builder's job, covered where that lives
        // (`config::tests::a_scheme_less_host_and_port_gets_the_scheme_it_implies`);
        // adding it here would mean two places decide what the endpoint is.
        assert!(!config.endpoint().contains("://"));
    }

    #[test]
    fn a_role_selects_that_participant_and_its_token() {
        for spelling in ["app-user", "app_user", "APP-USER"] {
            let config = devkit().config(Some(spelling)).unwrap();
            assert_eq!(
                config.endpoint(),
                "grpc-ledger-api.app-user.demo.localhost:2901",
                "spelling {spelling}"
            );
            assert!(matches!(config.auth(), Auth::Static(t) if t == "user.jwt.token"));
        }
    }

    #[test]
    fn the_json_lane_reads_the_same_environment() {
        assert_eq!(
            devkit().json_endpoint(None).as_deref(),
            Some("http://json-ledger-api.app-provider.demo.localhost:3901")
        );
        // A role with no JSON URL exported yields nothing rather than the
        // default participant's — silently talking to the wrong node is worse
        // than not connecting.
        assert_eq!(devkit().json_endpoint(Some("sv")), None);
    }

    /// A set-but-empty variable is the trap. `std::env::var` reports it as a
    /// value, so an empty `CANTON_ENDPOINT` left behind by a previous shell
    /// would win over the real LocalNet URL and point the client at nothing.
    #[test]
    fn an_empty_variable_does_not_shadow_a_real_one() {
        let source = devkit(); // CANTON_ENDPOINT and CANTON_TOKEN are ""
        let config = source.config(None).unwrap();

        assert_eq!(
            config.endpoint(),
            "grpc-ledger-api.app-provider.demo.localhost:3901"
        );
        assert!(matches!(config.auth(), Auth::Static(t) if t == "provider.jwt.token"));
    }

    #[test]
    fn an_explicit_endpoint_wins_over_the_local_network() {
        let source = Source(|name: &str| {
            Some(
                match name {
                    ENDPOINT_OVERRIDE => "https://ledger.example:443",
                    TOKEN_OVERRIDE => "deployment.token",
                    "CANTON_GRPC_LEDGER_API_URL" => "grpc-ledger-api.demo.localhost:3901",
                    _ => return None,
                }
                .to_string(),
            )
        });

        let config = source.config(None).unwrap();
        assert_eq!(config.endpoint(), "https://ledger.example:443");
        assert!(matches!(config.auth(), Auth::Static(t) if t == "deployment.token"));
    }

    #[test]
    fn an_empty_environment_says_which_command_produces_one() {
        let empty = Source(|_: &str| None);

        let error = empty.config(None).unwrap_err().to_string();
        assert!(error.contains("CANTON_GRPC_LEDGER_API_URL"), "{error}");
        assert!(error.contains("canton-devkit localnet env"), "{error}");

        // The role form names that role's variable, not the default one — the
        // reader is about to go and set it.
        let error = empty.config(Some("sv")).unwrap_err().to_string();
        assert!(error.contains("CANTON_SV_GRPC_LEDGER_API_URL"), "{error}");
    }

    /// A LocalNet started without authentication exports no JWT. Refusing it
    /// would refuse the simplest network there is.
    #[test]
    fn a_network_without_tokens_is_still_a_network() {
        let source = Source(|name: &str| {
            (name == "CANTON_GRPC_LEDGER_API_URL").then(|| "localhost:3901".to_string())
        });

        let config = source.config(None).unwrap();
        assert!(matches!(config.auth(), Auth::None));
    }

    #[test]
    fn party_ids_are_reachable_by_role_and_by_alias() {
        let source = devkit();
        let party = |alias: &str| source.get(&format!("{}_PARTY", prefix(alias)));

        assert_eq!(
            party("app-provider").as_deref(),
            Some("app_provider::1220abcd")
        );
        // A party created later gets the same treatment as a built-in role.
        assert_eq!(party("bob").as_deref(), Some("bob::1220abcd"));
        assert_eq!(party("nobody"), None);
    }
}
