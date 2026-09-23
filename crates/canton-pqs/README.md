# canton-pqs

A typed read client for the Daml **Participant Query Store** (PQS, served by
Scribe).

Part of the [Canton Rust SDK](https://github.com/nodejumper-org/canton-rust-sdk).

## Two ways in, one set of types

PQS streams a participant's contracts into Postgres. It stores payloads in the
Daml JSON encoding — which is what `canton-daml` implements — so a contract read
here deserializes into the **same generated type** as one read from a
transaction stream:

```rust,ignore
let contracts = pqs.active::<AppInstallRequest>().await?;
let payload: &AppInstallRequest = contracts[0].payload();
```

The template names itself: the qname PQS knows it by is
`PACKAGE_NAME:MODULE_NAME:ENTITY_NAME`, read off the generated `Contract` impl,
so nobody types a string. It is the package **name**, which is what survives a
Smart Contract Upgrade rather than pinning one build.

## No hand-written SQL, and no interpolation

Predicates compile to parameterized statements. Every caller-supplied value is a
parameter — and so is every JSON field path, sent as `text[]` and applied with
`#>`. The statement text depends on the *shape* of a query and never on its
data, so a field name taken from user input is data Postgres never parses:

```rust,ignore
let sql = Query::<AppInstallRequest>::active()
    .filter(Predicate::eq(["meta", "values", "reason"], "renewal"))
    .filter(Predicate::signatory(&party))
    .limit(100)
    .compile();
// SELECT * FROM active($1) WHERE payload #> $2 = $3 AND $4 = ANY(signatories) LIMIT $5
```

Two details worth knowing:

- **Ordered comparisons on numbers are numeric.** LF-JSON carries `Int64` and
  `Numeric` as *strings*, so comparing an amount lexically would sort `"9"`
  after `"10"`. Pass a number and the cast is applied; pass a string only where
  the field really is text.
- **`Predicate::contains`** compiles to `@>`, the operator a GIN index on
  `payload` can serve — worth reaching for on a large store.

## Features

| Feature | Default | What it adds |
|---|---|---|
| `tls` | off | `PqsClient::connect_tls`, using the platform's root certificates. |

Off by default because PQS is usually reached inside a trust boundary, and a
crate that forces a TLS stack on a deployment that does not use one is charging
for nothing.

## Licence

Apache-2.0. See `LICENSE`.
