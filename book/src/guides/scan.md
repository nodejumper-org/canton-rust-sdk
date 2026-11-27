# Scan and the validator's scan-proxy

Scan is the service each Super Validator runs to publish what the DSO decided
and what happened on the network: the DSO party, the Amulet rules, the mining
rounds, traffic balances, the update history and snapshots of the active
contracts. [`canton_token::scan::ScanClient`](https://docs.rs/canton-token/latest/canton_token/scan/struct.ScanClient.html)
is a typed client for it. Through the facade it is `canton::token::scan`.

An application needs it for two things. Some choices take contracts the DSO
owns, such as the `AmuletRules` and the open mining round, and the submitting
party cannot see them, so they are passed as disclosed contracts and Scan is
where they are published. And an operator reads its traffic balance, the
update history and the snapshots from it.

## Two ways in

| Constructor | Base URL ends in | Token | Serves |
|---|---|---|---|
| `ScanClient::new` | `/api/scan` | not needed | everything on this page |
| `ScanClient::through_validator` | `/api/validator` | required | the DSO, the rules, the rounds, featured-app rights, transfer preapprovals, the name service, holdings summaries |

A Scan asked directly is one Super Validator's word, and this client does not
compare several Scans. A validator's scan-proxy does: it reads from several
Super Validators' Scans and answers when enough of them agree. Use it where
you have a validator. The reads it does not serve (the update and event
history, the snapshots, traffic, closed rounds, the list of Scans) are refused
with `Error::InvalidRequest` before anything is sent.

The scan-proxy authenticates every request. Give the client a token with
`with_token`, or a source that refreshes one with `with_token_source`, which
takes a `canton_auth::TokenProvider`:

```rust,ignore
use canton::auth::{OidcConfig, TokenProvider};
use canton::token::scan::ScanClient;

let scan = ScanClient::through_validator("https://validator.example/api/validator")?
    .with_token_source(TokenProvider::new(OidcConfig::new(token_url, client_id, secret)));
```

`RegistryClient` takes a token the same way, so it can use the registry routes
a validator serves under `/api/validator/v0/scan-proxy`.

Both clients give up on a request after 30 seconds, never follow a redirect,
bound the size of a reply, and percent-encode every path segment. What Scan
answers goes into commands you sign, so it has to come from the Scan you
named.

The client is written against the documents of Splice 0.10.3. On an older
Scan or validator: `dso()` falls back from `/v1/dso`, which 0.10 added, to the
`/v0/dso` every release serves; the two snapshot page reads (`acs_snapshot`,
`holdings_state`) need Splice 0.7.5, where their paths appeared; and
`holdings_summary` needs 0.6.3. Everything else is served by Splice 0.6.0 and
later.

## Contracts to disclose

```rust,ignore
let rules = scan.amulet_rules().await?;
let round = scan.latest_open_mining_round().await?;

let disclosed = vec![rules.disclosed()?, round.disclosed()?];
```

- `amulet_rules()` returns the `AmuletRules` contract. A Scan is asked for
  it through the external API, where it comes with the DSO's other contracts.
- `latest_open_mining_round()` returns the open round to pass to a choice now.
  Several rounds are open at once and the newest has usually not reached its
  opening time. The one to use is the highest-numbered round whose `opensAt`
  has passed, which is the round Splice's own applications pick. The
  lowest-numbered one is open too, but it is archived next, and a transaction
  built on it fails if the archival lands first.
  `open_and_issuing_mining_rounds()` returns them all, and
  `MiningRounds::latest_open(now)` makes the same choice for a time you give.
- `featured_app_right(provider)` returns a provider's `FeaturedAppRight`, or
  `None`. It comes without a synchronizer: disclose it on the one the rules
  are on, `right.disclosed(&rules_synchronizer)`.
- `transfer_preapproval_by_party(party)` returns the party's
  `TransferPreapproval`, or `None`.

`disclosed()` gives the gRPC `DisclosedContract`, with the created-event blob
as bytes. `disclosed_json()` gives the JSON Ledger API's form, with the blob
as the base64 Scan sent. A contract that is being reassigned between
synchronizers has none to be disclosed on; that is `Error::Connection`, which
is retriable, and it passes in seconds.

## Internal endpoints

Splice tags each Scan endpoint `external` or `internal`. The external ones are
its public API. The internal ones may change in any Splice release, and two of
them are the only source of a contract an application may need:
`featured_app_right` and `transfer_preapproval_by_party`.

Asked of a Scan directly, those two are refused with `Error::InvalidRequest`
unless the client was built with `with_internal_endpoints()`, which is you
accepting that they follow Splice's internal API. Through a validator's
scan-proxy they are part of the proxy's own API and need no opt-in.

`amulet_rules()` works without the opt-in, since the external API publishes
the rules with the rest of the DSO's contracts (`dso()`). With the opt-in it
asks the internal endpoint instead, whose reply is the one contract.

## Traffic, history and snapshots

These are served by a Scan only.

- `member_traffic_status(synchronizer, member)` returns what a participant or
  mediator (`PAR::…`, `MED::…`) has consumed, its limit, and what it has
  purchased. The limit catches up with the purchase once it is processed.
- `updates(after, page_size)` pages through the update history, oldest first.
  The last update's `position()` is the `after` of the next page, and fewer
  than `page_size` updates means the end of what Scan has so far. Every Scan
  returns the same updates in the same order. `update(id)` reads one, and
  `events(after, page_size)` reads each update with the mediator's verdict,
  paged from the last entry's `position()`, which an entry has also when it is
  a verdict with no update.
- `acs_snapshot_timestamp(before, migration_id)` returns the record time of
  the most recent snapshot before a moment. `acs_snapshot(&request)`,
  `holdings_state(&request)` and `holdings_summary(&request)` read it. A
  `SnapshotRequest` names the snapshot and narrows the read:

```rust,ignore
let request = SnapshotRequest::at(migration_id, record_time)
    .with_page_size(500)
    .for_parties([party]);
let mut page = scan.acs_snapshot(&request).await?;
while let Some(token) = page.next_page_token.take() {
    page = scan.acs_snapshot(&request.clone().after(token)).await?;
}
```

Contract payloads, choice arguments and results are `serde_json::Value`: Daml
values of whatever template the contract has, in the compact encoding the
JSON Ledger API uses.

## Everything else

`get_json(version, segments, query)` and `post_json(version, segments, body)`
reach the paths this client has no method for, with the same transport (the
token, no redirects, a bounded reply, encoded segments) and the JSON returned
as it came.

## Testing

`cargo test -p canton-token --test scan` runs the client against a stub and
checks every path and HTTP method it uses against Splice's `scan.yaml` and
`scan-proxy.yaml`, vendored under `testdata/openapi/`. The live suite runs it
against a real Scan, a validator's scan-proxy, or both:

```sh
CANTON_SCAN_URL=https://scan.sv-1.example/api/scan \
  cargo test -p canton-token --test scan_live -- --nocapture

CANTON_SCAN_PROXY_URL=https://validator.example/api/validator CANTON_TOKEN=… \
  cargo test -p canton-token --test scan_live -- --nocapture
```

A Super Validator's Scan on DevNet, TestNet and MainNet answers `/api/scan`
only to addresses the Super Validators have allow-listed, which a validator's
address is. A LocalNet's Scan answers anyone: a Splice LocalNet or Canton
Builder Tool publishes it at `http://scan.localhost:4000/api/scan`, and a
validator's API is on its own port (`http://localhost:3903/api/validator` for
Canton Builder Tool's app-provider, with `canton builder token` as the token).
