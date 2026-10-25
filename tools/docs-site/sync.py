#!/usr/bin/env python3
"""Copy the repository's own documents into the docs site, with working links.

The site's guides are written for the site, under `book/src/`. The reference
material (compatibility matrix, ADRs, changelog, maintenance plan...) already
lives in the repository and is read there too, so it is copied in at build time
rather than duplicated. Copying alone would break its links: a path relative to
`docs/` means something else under `book/src/reference/`. Each relative link is
therefore resolved against the file's place in the repository and rewritten to
the copied page when that file is part of the site, or to the file on GitHub
when it is not (source files, test fixtures, workflow files).

Usage:
  tools/docs-site/sync.py                  before `mdbook build`: writes book/src/{reference,project}
  tools/docs-site/sync.py --after-build    after it: points the copied pages' "edit" links at their sources
"""

from __future__ import annotations

import posixpath
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / "book" / "src"
GITHUB = "https://github.com/nodejumper-org/canton-rust-sdk"

# Repository file -> page in the site (relative to book/src).
PAGES: dict[str, str] = {
    "docs/compatibility-matrix.md": "reference/compatibility-matrix.md",
    "docs/daml-lf-type-mapping.md": "reference/daml-lf-type-mapping.md",
    "docs/scu-regeneration.md": "reference/scu-regeneration.md",
    "docs/security/audit-scope.md": "reference/audit-scope.md",
    "docs/verification/token-standard-live-runs.md": "reference/live-runs.md",
    "docs/adr/README.md": "reference/adr/index.md",
    "CHANGELOG.md": "project/changelog.md",
    "MAINTENANCE.md": "project/maintenance.md",
    "SECURITY.md": "project/security.md",
    "CONTRIBUTING.md": "project/contributing.md",
    "RELEASING.md": "project/releasing.md",
}
PAGES.update(
    {
        f"docs/adr/{adr.name}": f"reference/adr/{adr.name}"
        for adr in sorted((ROOT / "docs" / "adr").glob("[0-9]*.md"))
    }
)

# Inline links `[text](target)` and reference definitions `[label]: target`.
INLINE = re.compile(r"(\]\()([^)\s]+)(\))")
REFERENCE = re.compile(r"^(\s*\[[^\]]+\]:\s*)(\S+)", re.M)
EXTERNAL = re.compile(r"^(?:[a-z][a-z0-9+.-]*:|#|/)", re.I)


def rewrite(target: str, source: str, page: str) -> str:
    if EXTERNAL.match(target):
        return target
    path, _, anchor = target.partition("#")
    anchor = f"#{anchor}" if anchor else ""
    resolved = posixpath.normpath(posixpath.join(posixpath.dirname(source), path))
    if resolved in PAGES:
        return posixpath.relpath(PAGES[resolved], posixpath.dirname(page)) + anchor
    if resolved.startswith(".."):
        raise SystemExit(f"{source}: link {target!r} leaves the repository")
    if not (ROOT / resolved).exists():
        raise SystemExit(f"{source}: link {target!r} points at nothing")
    kind = "tree" if (ROOT / resolved).is_dir() else "blob"
    return f"{GITHUB}/{kind}/main/{resolved}{anchor}"


def convert(source: str, page: str) -> str:
    text = (ROOT / source).read_text(encoding="utf-8")
    text = INLINE.sub(lambda m: m[1] + rewrite(m[2], source, page) + m[3], text)
    return REFERENCE.sub(lambda m: m[1] + rewrite(m[2], source, page), text)


def fix_edit_links() -> None:
    """A copied page is not in the repository at book/src/<page>, so mdBook's
    edit link for it would open a 404. Point it at the file it was copied from."""
    build = ROOT / "book" / "build"
    for source, page in PAGES.items():
        html = build / Path(page).with_suffix(".html")
        if html.name == "README.html":
            html = html.with_name("index.html")
        text = html.read_text(encoding="utf-8")
        old = f"{GITHUB}/edit/main/book/src/{page}"
        if old not in text:
            raise SystemExit(f"{html.relative_to(ROOT)}: no edit link {old}")
        html.write_text(text.replace(old, f"{GITHUB}/edit/main/{source}"), encoding="utf-8")
    print(f"pointed {len(PAGES)} edit links at their sources")


def main() -> int:
    if sys.argv[1:] == ["--after-build"]:
        fix_edit_links()
        return 0
    for generated in ("reference", "project"):
        shutil.rmtree(SRC / generated, ignore_errors=True)
    for source, page in PAGES.items():
        out = SRC / page
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(convert(source, page), encoding="utf-8")
    print(f"synced {len(PAGES)} pages into {SRC.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
