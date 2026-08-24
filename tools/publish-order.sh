#!/usr/bin/env bash
# Print the order the crates must be published in, derived from the manifests.
#
# `cargo publish` verifies a package by building it, which resolves dev
# dependencies too — so the order has to respect those as well. See RELEASING.md.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
python3 - <<'PY'
import re, pathlib

def sections(text):
    current, out = None, {}
    for line in text.splitlines():
        header = re.match(r'^\[([^\]]+)\]', line)
        if header:
            current = header.group(1)
            out.setdefault(current, [])
        elif current:
            out[current].append(line)
    return out

crates = {}
for manifest in sorted(pathlib.Path("crates").glob("*/Cargo.toml")):
    text = manifest.read_text()
    name = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M).group(1)
    if re.search(r'^publish\s*=\s*false', text, re.M):
        continue
    deps = set()
    for section, lines in sections(text).items():
        if section in ("dependencies", "dev-dependencies") or section.startswith("build-dependencies"):
            deps |= {m.group(1) for m in (re.match(r'^(canton[\w-]*)\s*=', l) for l in lines) if m}
    crates[name] = deps

for name in crates:
    crates[name] = {d for d in crates[name] if d in crates and d != name}

order, remaining = [], dict(crates)
while remaining:
    ready = sorted(n for n, d in remaining.items() if not (d - set(order)))
    if not ready:
        raise SystemExit(f"dependency cycle among {sorted(remaining)}")
    order.extend(ready)
    for name in ready:
        remaining.pop(name)

for position, name in enumerate(order, 1):
    print(f"{position:2d}. {name}")
PY
