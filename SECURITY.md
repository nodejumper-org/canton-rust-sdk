# Security Policy

## Reporting a vulnerability

Please report suspected security vulnerabilities **privately** — do not open
a public issue or pull request.

- Email: **dmitryk@nodejumper.io** (subject: `SECURITY canton-rust-sdk`)
- Include: affected crate(s) and version/commit, a description, reproduction
  steps or a proof of concept, and the impact as you assess it.

You will receive an acknowledgement within **3 business days**. We will work
with you on triage, a fix, and coordinated disclosure; please give us a
reasonable window to release a fix before any public disclosure. Credit is
given in the release notes unless you prefer otherwise.

## Scope

The `canton-*` crates in this repository. Vulnerabilities in Canton itself,
Splice, or other Digital Asset components should be reported to their
respective maintainers.

## Supported versions

Until 1.0, only the **latest released minor version** receives security
fixes. An independent security review of the client, codegen, and token
crates is scheduled at Milestone 3 of the funding proposal; its scope and
remediation summary will be published.

## Handling of secrets

The SDK never logs bearer tokens or client secrets (`Debug` output is
redacted, and the OIDC client secret is not readable through the public
API). If you find a path where credential material can leak into logs,
telemetry, or errors, please report it as a vulnerability.
