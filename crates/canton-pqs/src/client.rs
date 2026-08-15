//! The client: run a compiled query, decode the rows.

use std::fmt::Write as _;

use canton_core::{Error, Result};
use canton_daml::Contract as ContractType;
use tokio_postgres::types::ToSql;

use crate::query::{Param, Predicate, Query, Sql};
use crate::row::{Contract, Exercise};

/// A read client for a PQS database.
///
/// Cloning is cheap and clones share the connection, which pipelines
/// concurrent queries — so pooling is the application's decision rather than
/// this crate's, as telemetry is.
#[derive(Clone, Debug)]
pub struct PqsClient {
    client: std::sync::Arc<tokio_postgres::Client>,
}

impl PqsClient {
    /// Connect without TLS.
    ///
    /// `config` is a libpq connection string —
    /// `host=… user=… password=… dbname=…` — or a `postgres://` URL. The
    /// connection is driven on a spawned task; dropping every clone of the
    /// client ends it.
    ///
    /// # Errors
    /// [`Error::Connection`] if the store cannot be reached, and
    /// [`Error::InvalidRequest`] if it refuses the connection for a reason
    /// waiting will not change — a wrong password, an unknown database.
    pub async fn connect(config: &str) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(config, tokio_postgres::NoTls)
            .await
            .map_err(|e| connect_error("cannot reach PQS", &e))?;
        Ok(Self::spawn(client, connection))
    }

    /// Connect over TLS, with the platform's root certificates.
    ///
    /// # Errors
    /// [`Error::Connection`] if the store cannot be reached, and
    /// [`Error::InvalidRequest`] if no root certificates can be loaded, if the
    /// certificate cannot be verified, or if the store refuses the connection
    /// for a reason waiting will not change.
    #[cfg(feature = "tls")]
    pub async fn connect_tls(config: &str) -> Result<Self> {
        let mut roots = rustls::RootCertStore::empty();
        let native = rustls_native_certs::load_native_certs();
        if native.certs.is_empty() {
            return Err(Error::InvalidRequest(
                "no native root certificates were found, so a TLS connection could not be \
                 verified"
                    .to_string(),
            ));
        }
        roots.add_parsable_certificates(native.certs);
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(
            rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth(),
        );
        let (client, connection) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| connect_error("cannot reach PQS over TLS", &e))?;
        Ok(Self::spawn(client, connection))
    }

    fn spawn<S, T>(
        client: tokio_postgres::Client,
        connection: tokio_postgres::Connection<S, T>,
    ) -> Self
    where
        // `Connection<S, T>` is socket first, TLS stream second — the driver
        // is generic over both so this works for `NoTls` and for rustls alike.
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
        T: tokio_postgres::tls::TlsStream + Unpin + Send + 'static,
    {
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                // The connection ending is normal at shutdown; a failure while
                // queries are in flight is not, and it is otherwise silent.
                tracing::warn!(error = %e, "the PQS connection ended");
            }
        });
        Self {
            client: std::sync::Arc::new(client),
        }
    }

    /// Run a query and decode every row.
    ///
    /// # Errors
    /// [`Error::Connection`] if the query fails, or a decoding error if a row
    /// does not match `T`.
    pub async fn run<T>(&self, query: &Query<T>) -> Result<Vec<Contract<T>>>
    where
        T: ContractType + serde::de::DeserializeOwned,
    {
        let rows = self.query(&query.compile()).await?;
        rows.iter().map(Contract::from_row).collect()
    }

    /// The active contracts of a template, unfiltered.
    ///
    /// # Errors
    /// As [`run`](Self::run).
    pub async fn active<T>(&self) -> Result<Vec<Contract<T>>>
    where
        T: ContractType + serde::de::DeserializeOwned,
    {
        self.run(&Query::<T>::active()).await
    }

    /// One contract by id, or `None` if PQS does not have it.
    ///
    /// Reads `lookup_contract`, which finds a contract whether it is active or
    /// archived — unlike a filter on `active()`.
    ///
    /// # Errors
    /// As [`run`](Self::run).
    pub async fn lookup<T>(&self, contract_id: &str) -> Result<Option<Contract<T>>>
    where
        T: ContractType + serde::de::DeserializeOwned,
    {
        let sql = Sql {
            text: "SELECT * FROM lookup_contract($1, $2)".to_string(),
            params: vec![
                Param::Text(contract_id.to_string()),
                Param::Text(Query::<T>::qname()),
            ],
        };
        let rows = self.query(&sql).await?;
        rows.first().map(Contract::from_row).transpose()
    }

    /// The exercises of one choice, in a range of offsets.
    ///
    /// The choice's *name* is taken from the type, as a template's qname is.
    ///
    /// # Errors
    /// As [`run`](Self::run).
    pub async fn exercises<T, C>(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<Exercise<C>>>
    where
        T: ContractType,
        C: canton_daml::Choice<T> + serde::de::DeserializeOwned,
    {
        // PQS names an exercise by template qname and choice.
        let qname = format!("{}:{}", Query::<T>::qname(), C::NAME);
        let mut params = vec![Param::Text(qname)];
        let mut text = "SELECT * FROM exercises($1".to_string();
        if from.is_some() || to.is_some() {
            let from_sql = from.map_or_else(
                || "COALESCE(pruned_offset(), oldest_offset())".to_string(),
                |value| {
                    params.push(Param::Offset(value));
                    format!("${}", params.len())
                },
            );
            let to_sql = to.map_or_else(
                || "latest_offset()".to_string(),
                |value| {
                    params.push(Param::Offset(value));
                    format!("${}", params.len())
                },
            );
            let _ = write!(text, ", {from_sql}, {to_sql}");
        }
        text.push(')');

        let rows = self.query(&Sql { text, params }).await?;
        rows.iter().map(Exercise::from_row).collect()
    }

    /// The latest offset PQS has ingested.
    ///
    /// What a caller reads to know how far behind the store is, and what to
    /// pass to [`Query::active_at`] for a stable read.
    ///
    /// # Errors
    /// As [`run`](Self::run).
    pub async fn latest_offset(&self) -> Result<i64> {
        let rows = self
            .query(&Sql {
                text: "SELECT latest_offset()".to_string(),
                params: Vec::new(),
            })
            .await?;
        let row = rows.first().ok_or_else(|| {
            Error::UnexpectedResponse("PQS answered latest_offset() with no row".to_string())
        })?;
        row.try_get::<_, Option<i64>>(0)
            .map_err(|e| {
                Error::UnexpectedResponse(format!(
                    "PQS returned a latest offset this cannot read: {e}"
                ))
            })?
            .ok_or_else(|| {
                Error::UnexpectedResponse(
                    "PQS reported no latest offset, so it has ingested nothing".to_string(),
                )
            })
    }

    /// Run a compiled statement.
    ///
    /// # Errors
    /// [`Error::Connection`] if the query fails.
    pub async fn query(&self, sql: &Sql) -> Result<Vec<tokio_postgres::Row>> {
        let values: Vec<Box<dyn ToSql + Sync + Send>> = sql
            .params
            .iter()
            .map(|param| -> Box<dyn ToSql + Sync + Send> {
                match param {
                    Param::Json(value) => Box::new(value.clone()),
                    // A numeric is bound as text and cast in the statement:
                    // there is no `numeric` Rust type here, and a float would
                    // not round-trip a decimal exactly — which is the whole
                    // reason LF-JSON carries them as strings.
                    Param::Text(value) | Param::Numeric(value) => Box::new(value.clone()),
                    Param::Path(segments) => Box::new(segments.clone()),
                    Param::Offset(value) => Box::new(*value),
                }
            })
            .collect();
        let borrowed: Vec<&(dyn ToSql + Sync)> = values
            .iter()
            .map(|value| value.as_ref() as &(dyn ToSql + Sync))
            .collect();

        tracing::debug!(sql = %sql.text, params = sql.params.len(), "PQS query");
        self.client
            .query(sql.text.as_str(), &borrowed)
            .await
            .map_err(|e| classify(&e))
    }
}

/// Convenience: the active contracts of a template that a party signs.
///
/// The commonest query there is, and one that is easy to write as a payload
/// filter by mistake — the signatories are a column, and a column comparison
/// is what an index serves.
#[must_use]
pub fn active_signed_by<T: ContractType>(party: &str) -> Query<T> {
    Query::<T>::active().filter(Predicate::signatory(party))
}

/// Tell a failed *statement* from a failed *connection*.
///
/// This decides whether the SDK reports the failure as retriable, and getting
/// it wrong is not cosmetic in either direction. Report a mistyped predicate as
/// retriable and an application looping on `is_retriable()` spins forever on a
/// bug only a human can fix. Report a failover as *non*-retriable and the same
/// application gives up on a five-second restart, having told its operator to
/// go and look at a query that was never wrong.
///
/// A SQLSTATE alone does not separate the two. It says the server understood
/// the statement well enough to refuse it, which is true of a syntax error and
/// equally true of `57P01 admin_shutdown`. What separates them is the
/// SQLSTATE's two-character *class*:
///
/// | Class | Meaning | Verdict |
/// |---|---|---|
/// | `08` | connection exception | retriable |
/// | `40` | transaction rollback — serialization failure, deadlock | retriable |
/// | `53` | insufficient resources — out of memory, too many connections | retriable |
/// | `55` | object not in prerequisite state — lock unavailable | retriable |
/// | `57` | operator intervention — shutdown, cancelled, crash | retriable |
/// | *anything else* | the statement is wrong (`42…`), the data is (`22…`) | caller's |
///
/// Everything transient here is a condition the *same statement* would survive
/// on a later attempt, which is exactly what retriable has to mean.
fn classify(error: &tokio_postgres::Error) -> Error {
    let Some(code) = error.code() else {
        return Error::Connection(format!("PQS query failed: {}", detail(error)));
    };
    if is_transient_sqlstate(code.code()) {
        // Reported as a connection failure because that is the variant
        // `is_retriable` already answers `true` for, and the condition really
        // is the server being momentarily unable rather than unwilling.
        Error::Connection(format!(
            "PQS is momentarily unavailable ({}): {}",
            code.code(),
            detail(error)
        ))
    } else {
        Error::InvalidRequest(format!("PQS rejected the query: {}", detail(error)))
    }
}

/// Classify a failure to *connect*, as [`classify`] does for a failure to
/// query.
///
/// Written because the query path was fixed and this one was not, in the same
/// file. Both had the same two defects. The message dropped everything under
/// the outer error — a `tokio_postgres::Error` prints as `error connecting to
/// server` and keeps *which* server and *why* in its source chain — and the
/// verdict was `Error::Connection` unconditionally, which is retriable. So a
/// wrong password, an unknown database and an unverifiable certificate were all
/// retried forever, with nothing in the message saying which.
///
/// A refused connection carries a SQLSTATE (`28P01` bad password, `3D000` no
/// such database, `28000` rejected by `pg_hba.conf`), so the same class rule
/// separates them from a store that is merely down.
fn connect_error(what: &str, error: &tokio_postgres::Error) -> Error {
    let detail = detail(error);
    if let Some(code) = error.code()
        && !is_transient_sqlstate(code.code())
    {
        return Error::InvalidRequest(format!(
            "{what}: the store refused the connection ({}): {detail}",
            code.code()
        ));
    }
    // A TLS failure arrives with no SQLSTATE — it never reached the protocol —
    // so the chain is the only place it shows. Permanent either way.
    if detail.to_ascii_lowercase().contains("certificate") {
        return Error::InvalidRequest(format!(
            "{what}: its certificate could not be verified: {detail}"
        ));
    }
    Error::Connection(format!("{what}: {detail}"))
}

/// Whether a SQLSTATE names a condition a later attempt could survive.
///
/// Matched on the class rather than on individual codes: PostgreSQL assigns
/// codes within a class by the same rule, so a code added later lands on the
/// right verdict without this list being revisited.
fn is_transient_sqlstate(sqlstate: &str) -> bool {
    matches!(sqlstate.get(..2), Some("08" | "40" | "53" | "55" | "57"))
}

/// A `tokio_postgres::Error` prints as "db error" and keeps what the database
/// actually said in its source. Shared with every other transport in the SDK.
fn detail(error: &tokio_postgres::Error) -> String {
    canton_core::chain(error)
}

#[cfg(test)]
mod tests {
    use super::is_transient_sqlstate;

    /// The codes an operator actually meets. Each of these used to be reported
    /// as "PQS rejected the query", which sent whoever read it to look at a
    /// predicate that was never wrong — and, because `InvalidRequest` is not
    /// retriable, made an application give up on a restart it should have
    /// ridden out.
    #[test]
    fn a_server_that_is_momentarily_unable_is_retriable() {
        for (code, what) in [
            ("57P01", "the admin shut the server down — a failover"),
            ("57P02", "the server crashed"),
            ("57P03", "the server cannot connect yet — still starting"),
            ("40001", "serialization failure under concurrent load"),
            ("40P01", "deadlock detected"),
            ("53300", "too many connections"),
            ("53200", "out of memory"),
            ("55P03", "lock not available"),
            ("08006", "connection failure"),
        ] {
            assert!(
                is_transient_sqlstate(code),
                "{code} ({what}) must be retriable"
            );
        }
    }

    /// And the other direction, which is the reason the whole distinction
    /// exists: retrying a statement the server understood and refused just
    /// spins.
    #[test]
    fn a_statement_the_server_refused_is_the_callers_to_fix() {
        for (code, what) in [
            ("42601", "syntax error"),
            ("42883", "no such function — a wrong qname"),
            ("42703", "no such column"),
            ("22P02", "invalid text representation — a bad cast"),
            ("23505", "unique violation"),
        ] {
            assert!(
                !is_transient_sqlstate(code),
                "{code} ({what}) must not be retriable"
            );
        }
    }

    /// A SQLSTATE is five characters; anything shorter must not index-panic or
    /// be read as a class it does not have.
    #[test]
    fn a_malformed_sqlstate_is_not_mistaken_for_a_transient_one() {
        for code in ["", "5", "4"] {
            assert!(!is_transient_sqlstate(code), "{code:?} is not a class");
        }
    }
}
