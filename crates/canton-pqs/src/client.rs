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
    /// [`Error::Connection`] if the connection cannot be established.
    pub async fn connect(config: &str) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(config, tokio_postgres::NoTls)
            .await
            .map_err(|e| Error::Connection(format!("cannot reach PQS: {e}")))?;
        Ok(Self::spawn(client, connection))
    }

    /// Connect over TLS, with the platform's root certificates.
    ///
    /// # Errors
    /// [`Error::Connection`] if the connection or the TLS handshake fails, and
    /// [`Error::InvalidRequest`] if no root certificates can be loaded.
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
            .map_err(|e| Error::Connection(format!("cannot reach PQS over TLS: {e}")))?;
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
/// it wrong is not cosmetic: `Error::Connection` is retriable, so mapping every
/// Postgres error to it makes an application that loops on `is_retriable()`
/// spin forever on a mistyped predicate. A database error carries a SQLSTATE —
/// the server understood the statement and refused it — and no amount of
/// retrying changes that.
fn classify(error: &tokio_postgres::Error) -> Error {
    if error.code().is_some() {
        Error::InvalidRequest(format!("PQS rejected the query: {}", detail(error)))
    } else {
        Error::Connection(format!("PQS query failed: {}", detail(error)))
    }
}

/// A `tokio_postgres::Error` prints as "db error" and keeps what the database
/// actually said in its source. Reporting only the outer message hides the one
/// sentence that says which column or cast was wrong.
fn detail(error: &tokio_postgres::Error) -> String {
    let mut message = error.to_string();
    let mut source = std::error::Error::source(error);
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}
