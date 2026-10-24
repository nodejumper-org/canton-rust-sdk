#!/usr/bin/env python3
"""Fail on a link inside the built site that points at nothing.

mdBook turns `.md` links into `.html` but does not check that the target
exists, so a renamed page or a mistyped path only shows up as a 404 for a
reader. This walks every built page and resolves each relative `href`/`src`
against the output directory. A missing file is an error. A missing `#anchor`
on an existing page is reported as a warning, since a heading can be renamed
without breaking the page itself.

Usage: tools/docs-site/check_links.py book/build
"""

from __future__ import annotations

import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit


class Page(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.links: list[str] = []
        self.ids: set[str] = set()

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        for name, value in attrs:
            if value is None:
                continue
            if name == "id":
                self.ids.add(value)
            elif (tag, name) in {("a", "href"), ("img", "src"), ("script", "src"), ("link", "href")}:
                self.links.append(value)


def parse(path: Path) -> Page:
    page = Page()
    page.feed(path.read_text(encoding="utf-8", errors="replace"))
    return page


def main(root: Path) -> int:
    pages = {path: parse(path) for path in root.rglob("*.html")}
    errors, warnings = [], []
    for path, page in pages.items():
        # mdBook's own print page and 404 page link relative to the site root.
        if path.name in {"print.html", "404.html"}:
            continue
        for link in page.links:
            parts = urlsplit(link)
            if parts.scheme or parts.netloc or link.startswith(("mailto:", "javascript:")):
                continue
            target = path if not parts.path else (path.parent / unquote(parts.path)).resolve()
            if target.is_dir():
                target = target / "index.html"
            where = path.relative_to(root)
            if not target.exists():
                errors.append(f"{where}: {link} -> no such file")
                continue
            if parts.fragment and target.suffix == ".html":
                ids = pages[target].ids if target in pages else parse(target).ids
                if unquote(parts.fragment) not in ids:
                    warnings.append(f"{where}: {link} -> no #{parts.fragment} on that page")
    for line in warnings:
        print(f"warning: {line}")
    for line in errors:
        print(f"error: {line}")
    print(f"checked {len(pages)} pages: {len(errors)} broken links, {len(warnings)} missing anchors")
    return 1 if errors else 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    sys.exit(main(Path(sys.argv[1]).resolve()))
