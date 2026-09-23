# Token standard V2 packages

Daml-LF `ArchivePayload` bytes for the CIP-0112 (V2) token-standard packages,
downloaded from a participant with
`cargo run -p canton-admin --example fetch_packages`.

**Why they are here and not a DAR URL.** These packages ship as no DAR anyone
publishes: cn-quickstart carries only the V1 set, the Splice repository holds
Daml sources rather than built artefacts, and there are no release assets. They
exist on the network, vetted by the participants there, and the Ledger API's
`GetPackage` is the way to obtain them.

**Why no checksum file.** A Daml package id *is* the SHA-256 of these bytes, and
each file name ends with the id it hashes to. `canton_lf::decode_package`
re-derives it for a whole `Archive`; for a payload, `AdminClient::get_package`
checks the participant's answer against the id that was asked for.

**Payloads, not `.dalf`.** A DAR entry is a whole `Archive` — payload, hash and
hash function together. The Ledger API returns those as separate response
fields, so what is stored here is the payload alone and
`canton_lf::decode_payload` is what reads it.
