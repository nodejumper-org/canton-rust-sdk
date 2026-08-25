#!/usr/bin/env bash
# Maintenance for the vendored Canton protos: re-hash them after a deliberate
# re-vendor, or compare them against the copies inside a running Canton
# container. See crates/canton-proto/proto/PROVENANCE.md.
set -euo pipefail

proto_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../crates/canton-proto/proto" && pwd)"

usage() {
    cat <<'USAGE'
usage: tools/vendor-protos.sh --rehash
       tools/vendor-protos.sh --compare-image <container>

  --rehash                 rewrite SHA256SUMS from the current tree
  --compare-image <name>   diff the tree against /app/protobuf in a running
                           Canton container, reporting matches, differences,
                           and files absent there
USAGE
}

rehash() {
    cd "$proto_dir"
    find . -name '*.proto' | sed 's|^\./||' | sort | while read -r file; do
        printf '%s  %s\n' "$(shasum -a 256 "$file" | cut -d' ' -f1)" "$file"
    done >SHA256SUMS
    echo "SHA256SUMS: $(wc -l <SHA256SUMS | tr -d ' ') files"
}

compare_image() {
    local container="$1"
    local staging
    staging="$(mktemp -d)"
    trap 'rm -rf "$staging"' EXIT
    docker cp "$container:/app/protobuf" "$staging/" >/dev/null

    local matched=0 differ=0 absent=0
    cd "$proto_dir"
    while read -r file; do
        local theirs
        theirs="$(find "$staging/protobuf" -path "*/$file" -type f | head -1)"
        if [ -z "$theirs" ]; then
            absent=$((absent + 1))
            echo "absent from the image: $file"
        elif cmp -s "$file" "$theirs"; then
            matched=$((matched + 1))
        else
            differ=$((differ + 1))
            echo "differs: $file"
            diff "$file" "$theirs" | grep '^>' | sed 's/^/    image-only: /' || true
        fi
    done < <(find . -name '*.proto' | sed 's|^\./||' | sort)
    echo "identical=$matched differ=$differ absent=$absent"
}

case "${1:-}" in
    --rehash) rehash ;;
    --compare-image) [ $# -eq 2 ] || { usage; exit 2; }; compare_image "$2" ;;
    *) usage; exit 2 ;;
esac
