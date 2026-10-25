# Querying PQS

[`canton-pqs`](https://docs.rs/canton-pqs/latest/canton_pqs/) is a typed read
client for the Daml **Participant Query Store**. You name a template by its
generated Rust type, add predicates, and get rows back as the same generated
payload type the Ledger API path gives you. The common reads need no SQL, and
the values and field paths you pass to a `Query` are bound as parameters, never
spliced into the statement (`Sql::raw`, below, runs its text as given).

## What PQS is

PQS is a Postgres database kept up to date by **Scribe**, a process that follows
one participant's Ledger API and writes what it reads into the database. The
store holds only what Scribe's ledger connection can see. With authentication
on, that means the contracts visible to the token's user. It exposes the data
through SQL functions: `active`, `creates`, `archives`, `exercises`,
`lookup_contract`, and `latest_offset` for how far it has ingested.

Payloads are stored as JSONB in the Daml JSON encoding, which is what
`canton-daml` implements, so a contract read here deserializes into the same
generated type as one read from a transaction stream. Templates are addressed
as `PACKAGE_NAME:MODULE_NAME:ENTITY_NAME`. The client reads that off the type's
[`canton_daml::Contract`](https://docs.rs/canton-daml/latest/canton_daml/trait.Contract.html)
impl. It uses the package **name**, not the id, so a query keeps working after a
Smart Contract Upgrade.

Use PQS to filter in the database, read the active contract set as of an
earlier offset, or find an archived contract by id. It is read-only and trails
the participant by however far Scribe is behind; for everything else see
[Reading the ledger](reading-the-ledger.md). The verified Scribe version is in
the [compatibility matrix](../reference/compatibility-matrix.md#pqs).

## Add the dependency

Through the facade, the crate is `canton::pqs`:

```toml
[dependencies]
canton = "0.3"
# or, to reach a store over TLS:
# canton = { version = "0.3", features = ["pqs-tls"] }
```

| Feature | Crate | What it adds |
|---|---|---|
| `tls` | `canton-pqs` | `PqsClient::connect_tls`, verified against the platform's root certificates |
| `pqs-tls` | `canton` | Forwards `canton-pqs/tls`, so you don't need a second dependency for it |

TLS is off by default because PQS is usually reached inside a trust boundary.
The template types come from a bindings crate: one you generated
([Typed bindings from a DAR](codegen.md)) or a pre-built one such as
`canton-splice-amulet`, which the examples below use.

## Connect

[`PqsClient::connect`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.connect)
takes a libpq connection string (`host=… user=… password=… dbname=…`) or a
`postgres://` URL:

```rust,ignore
use std::time::Duration;
use canton::pqs::PqsClient;

let pqs = PqsClient::connect("postgres://pqs:pqs@localhost:5433/pqs").await?;

// A different deadline for the connect and for every query after it:
let pqs = PqsClient::connect_with_timeout(url, Duration::from_secs(5)).await?;
let pqs = pqs.with_timeout(Duration::from_secs(10));
```

- **One connection, shared.** The client drives a single connection on a
  spawned Tokio task, so create it inside a Tokio runtime. Clones are cheap and
  share the connection, which pipelines concurrent queries; pooling is up to you.
- **Timeouts.** A connection attempt or query that takes longer than
  [`DEFAULT_TIMEOUT`](https://docs.rs/canton-pqs/latest/canton_pqs/constant.DEFAULT_TIMEOUT.html)
  (30 seconds) fails with `Error::Timeout`.
  [`with_timeout`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.with_timeout)
  changes the timeout for the client and every clone of it.
- **Reconnect.** If the store closes the connection (failover, idle timeout,
  restart), the next query opens a fresh connection and runs the statement once
  more. You don't need to handle it.
- **No password in logs.** The client's `Debug` output leaves it out.

### TLS

With the `tls` (or `pqs-tls`) feature,
[`connect_tls`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.connect_tls)
and `connect_tls_with_timeout` take the same connection string, connect over
rustls, and verify the server against the platform's native root certificates.

TLS is required, not preferred. The client forces `sslmode=require`, so a store
that declines TLS gets an error, not a plaintext session, and an
`sslmode=disable` in the string does not turn TLS off. If no native roots can be
loaded or the certificate does not verify, you get `Error::InvalidRequest`. You
can't supply your own CA bundle or a client certificate.

## Read active contracts

[`active`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.active)
returns the active contracts of one template:

```rust,ignore
use canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet;

for c in pqs.active::<Amulet>().await? {
    let payload: &Amulet = c.payload();
    println!("{} owner={} created at {}", c.contract_id().as_str(), payload.owner, c.created_at_offset());
}
```

Each row is a
[`canton_pqs::Contract<T>`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.Contract.html).
This is the row wrapper, not the `canton_daml::Contract` trait. It carries:

| Accessor | What it is |
|---|---|
| `payload()`, `into_payload()` | the decoded `T` |
| `contract_id()` | a typed `ContractId<T>`, usable directly with [`exercise_command`](https://docs.rs/canton-daml/latest/canton_daml/fn.exercise_command.html) |
| `created_at_offset()`, `archived_at_offset()` | ledger offsets; the second is `None` if never archived |
| `was_active_at(offset)` | whether the contract was live at that offset |
| `created_effective_at()`, `archived_effective_at()` | ledger effective times, as `time::OffsetDateTime` |
| `signatories()`, `observers()`, `witnesses()` | party ids |
| `package_name()`, `package_version()`, `package_id()` | which package build created it |

`created_effective_at()` is `None` for contracts Scribe seeded from an ACS
snapshot rather than the update stream (`SCRIBE_PIPELINE_LEDGER_START=Latest`).
Because the query matches on the package name, one result can mix contracts
from different package versions, and `package_version()` tells them apart. If
any row's payload does not decode into `T`, the whole call fails with
`Error::Payload`, and the error names the contract id.

## Filter with predicates

Build a [`Query`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.Query.html),
add [`Predicate`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.Predicate.html)s
with `filter` (they are combined with `AND`), and run it with
[`run`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.run).
This example is from the live suite:

```rust,ignore
{{#include ../../../crates/canton-pqs/tests/live.rs:139:142}}
```

| Predicate | Compiles to | Use it for |
|---|---|---|
| `Predicate::eq(path, value)` | `payload #> $p = $v` | exact JSON equality on a field |
| `Predicate::not_eq(path, value)` | `payload #> $p IS DISTINCT FROM $v` | inequality, **including** rows where the field is absent |
| `Predicate::compare(path, op, value)` | `payload #>> $p > $v`, numeric cast for numbers | ordered comparisons ([`Op`](https://docs.rs/canton-pqs/latest/canton_pqs/enum.Op.html)`::Lt`, `Le`, `Gt`, `Ge`) |
| `Predicate::contains(json)` | `payload @> $v` | JSONB containment |
| `Predicate::signatory(party)`, `observer`, `witness` | `$v = ANY(signatories)` and so on | party columns |
| `Predicate::contract_id(id)` | `contract_id = $v` | one contract |

**Paths** are anything that implements
[`IntoPath`](https://docs.rs/canton-pqs/latest/canton_pqs/trait.IntoPath.html):
a `&str` or `String` for a top-level field, or `["amount", "initialAmount"]`, a
`&[&str]` or a `Vec<String>` for a nested one. A string is always one segment,
so `"a.b"` is not split. Segments are the **Daml** field names as they appear in
the JSON (`initialAmount`), not the Rust field names (`initial_amount`).

**Numbers.** LF-JSON carries `Int64` and `Numeric` as strings, and PQS stores them that way:

- For ordered comparisons, pass a Rust number. The client binds it as text and
  casts both sides to `numeric`, so `100` sorts above `9`. If you pass a string,
  the comparison is textual, so do that only for fields that really are text.
- `eq` and `not_eq` turn a JSON number into its string form, which works for
  `Int64`. A `Numeric` is stored with its full scale (an Amulet amount reads
  `"504680.1600000000"`), so `eq` only matches that exact text. For numeric
  equality, bound the field on both sides with `Op::Ge` and `Op::Le`.
  `contains` does no conversion either, so write numbers in a containment
  document as the stored strings.

**Absent fields.** `not_eq` uses `IS DISTINCT FROM` on purpose. A query that
spans package versions sees payloads with different field sets, and `<>` would
silently drop every row that lacks the field. An ordered comparison does drop
such rows, because "is a missing value greater than 10?" has no answer.

**Containment and party columns.** `contains` compiles to `@>`, the operator a
GIN index on `payload` can serve, and
[`active_signed_by`](https://docs.rs/canton-pqs/latest/canton_pqs/fn.active_signed_by.html)
is a shortcut for "the active contracts this party signs". From the live suite:

```rust,ignore
{{#include ../../../crates/canton-pqs/tests/live.rs:172:188}}
```

**Limit.** `limit(n)` caps the number of rows and adds
`ORDER BY created_at_offset, contract_id`, so two reads at the same offset
return the same rows. Without `limit`, no ordering is applied.

### What reaches Postgres

[`compile`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.Query.html#method.compile)
returns the statement without running it. The text depends only on the shape of
the query. The qname, every value and every field path are bound parameters, so
a field name taken from user input is data that Postgres never parses:

```rust,ignore
use canton::pqs::{Op, Predicate, Query};

// owner: canton_daml::Party
let sql = Query::<Amulet>::active()
    .filter(Predicate::eq("owner", owner.as_str()))
    .filter(Predicate::compare(["amount", "initialAmount"], Op::Gt, 100))
    .limit(50)
    .compile();
// sql.text:
// SELECT * FROM active($1) WHERE payload #> $2 = $3
//   AND (payload #>> $4)::numeric > $5::text::numeric
//   ORDER BY created_at_offset, contract_id LIMIT $6
```

## Read at an offset

[`latest_offset`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.latest_offset)
is how far Scribe has ingested. `Query::active_at(offset)` reads the active
contract set as of that offset, so repeated reads agree with each other while
the ledger keeps moving:

```rust,ignore
{{#include ../../../crates/canton-pqs/tests/live.rs:299:304}}
```

An offset older than the store holds is an error carrying PQS's own message,
not an empty result: "nothing was active" and "I can't tell you" differ. A
contract returned by `active_at(o)` may have `archived_at_offset()` set if it
was archived after `o`. Use `was_active_at(o)`, not
`archived_at_offset().is_none()`, to decide whether it was live. To read back
something you just submitted through the Ledger API, wait until
`latest_offset()` reaches the offset it committed at.

## Look up by id

[`lookup`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.lookup)
reads PQS's `lookup_contract`, which finds a contract whether it is active or
archived. An unknown id returns `Ok(None)`, not an error:

```rust,ignore
{{#include ../../../crates/canton-pqs/tests/live.rs:277:288}}
```

## Creates, archives and exercises

`Query::creates(from, to)` and `Query::archives(from, to)` read the contracts of
a template created or archived in an offset range. `None` on either side uses
the store's own bound: its oldest retained offset (for `archives`, the pruning
boundary if there is one) and its latest offset.

[`exercises::<T, C>(from, to)`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.exercises)
reads the exercises of choice `C` on template `T`, taking the choice name from
the generated `Choice` impl. Each
[`Exercise<C>`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.Exercise.html)
has a typed `argument()`. Its `result()` is untyped JSON, because PQS doesn't
say what type it is. Decode it with `result_as::<R>()`:

```rust,ignore
use canton_splice_amulet::splice_amulet::Splice_Amulet::{Amulet, Amulet_Expire, Amulet_ExpireResult};

// last_seen: i64, the offset you have processed up to
for e in pqs.exercises::<Amulet, Amulet_Expire>(Some(last_seen), None).await? {
    let result: Amulet_ExpireResult = e.result_as()?;
    println!("{} by {:?} at {}", e.contract_id(), e.controllers(), e.exercised_at_offset());
}
```

`creates`, `archives` and `exercises` are covered only by unit tests of the
statements they compile to. The other reads also run in the live suite.

## Other PQS functions: `Sql::raw`

For a PQS function or query shape the crate does not model, use
[`Sql::raw`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.Sql.html#method.raw)
with [`PqsClient::query`](https://docs.rs/canton-pqs/latest/canton_pqs/struct.PqsClient.html#method.query),
which returns `tokio_postgres::Row`s:

```rust,ignore
use canton::pqs::{Param, Query, Sql};

let rows = pqs.query(&Sql::raw(
    "SELECT count(*) AS n FROM active($1)",
    vec![Param::Text(Query::<Amulet>::qname())],
)).await?;
let n: i64 = rows[0].get("n");
```

The parameters are still bound, never interpolated, but the statement text runs
as given. `Sql::raw` is the one place where keeping input out of the statement
is your job. To decode rows yourself, for example to skip payloads from another
package version instead of failing the whole read, run `query(&q.compile())`
and call `canton_pqs::Contract::<T>::from_row` on each row.

## Errors, retries and logging

Errors are the SDK-wide [`canton::Error`](https://docs.rs/canton-core/latest/canton_core/enum.Error.html),
classified so that [`is_retriable()`](https://docs.rs/canton-core/latest/canton_core/enum.Error.html#method.is_retriable)
gives the right answer:

| Situation | Error | Retriable |
|---|---|---|
| store unreachable | `Connection` | yes |
| no answer within the timeout | `Timeout` | yes |
| SQLSTATE class `08`, `40`, `53`, `55`, `57` (connection loss, serialization failure, too many connections, lock unavailable, shutdown) | `Connection` | yes |
| connection string does not parse | `InvalidRequest` | no |
| wrong password, unknown database, `pg_hba.conf` rejection | `InvalidRequest` | no |
| certificate does not verify, or the store refuses TLS | `InvalidRequest` | no |
| any other rejected statement (syntax, unknown function or column, bad cast) | `InvalidRequest` | no |
| a payload that does not decode into `T` | `Payload` | no |
| a column missing from the row | `UnexpectedResponse` | no |

Before Scribe has applied its schema, queries fail with errors such as
`function active(...) does not exist`; wait for the store (see below).

The crate emits `tracing` events only, with no metrics or spans: debug per
query (statement text and parameter count, never values), info on a reconnect,
and warn when the connection task ends with an error.

## What it does not do

- **Write.** PQS is a read model. Submit commands through `canton-ledger`.
- **Stream.** There is no subscription. Poll `latest_offset()`, or use the
  Ledger API's update stream.
- **`OR`, ordering or paging.** Filters are combined with `AND` only. `limit`
  fixes the order but takes no cursor or SQL `OFFSET`. Use `Sql::raw` for
  anything beyond that.

## Run a store locally

[`tools/pqs/compose.yaml`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/tools/pqs/compose.yaml)
runs Postgres and Scribe against any LocalNet participant on the host. By
default it follows the Ledger API at `host.docker.internal:3901`, reads from
the ledger's beginning, and publishes Postgres on port 5433.
[`tools/pqs/wait-ready.sh`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/tools/pqs/wait-ready.sh)
waits until `latest_offset()` answers and has stopped changing between two
checks:

```sh
docker compose -f tools/pqs/compose.yaml up -d
# a participant that wants a token:
# SCRIBE_SOURCE_LEDGER_AUTH=OAuth CANTON_TOKEN=… docker compose -f tools/pqs/compose.yaml up -d
tools/pqs/wait-ready.sh
export CANTON_PQS_URL='postgres://pqs:pqs@localhost:5433/pqs'
CANTON_TEST_REQUIRE_LIVE=1 cargo test -p canton-pqs --all-features --test live -- --nocapture
docker compose -f tools/pqs/compose.yaml down -v
```

The live suite reads `Amulet`, so first tap some Canton Coin in the wallet of
the party the store follows. Otherwise the tests that need a sample contract
fail rather than pass with nothing checked. `SCRIBE_SOURCE_LEDGER_HOST`,
`SCRIBE_SOURCE_LEDGER_PORT`, `SCRIBE_PIPELINE_LEDGER_START` and `SCRIBE_VERSION`
point the store at another participant or pin another Scribe. See
[Local development and testing](../local-development.md) for the participant.
