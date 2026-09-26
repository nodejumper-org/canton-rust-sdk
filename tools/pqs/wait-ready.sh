#!/bin/sh
# Wait until the Scribe store answers *and* has caught up: it applies its
# schema and reads the ledger from genesis before `latest_offset()` exists, and
# a query before that fails with "function active(...) does not exist". Once
# the function exists Scribe may still be ingesting, so this also waits for
# the offset to hold still across two checks; a live suite that reads while
# ingestion is mid-way sees a store that disagrees with itself.
set -eu
compose="docker compose -f $(dirname "$0")/compose.yaml"
offset_now() {
  $compose exec -T postgres psql -U pqs -d pqs -tAc 'select latest_offset()' 2>/dev/null | grep -E '^[0-9]+$' || true
}
previous=""
for _ in $(seq 1 120); do
  current=$(offset_now)
  if [ -n "$current" ] && [ "$current" = "$previous" ]; then
    echo "pqs ready at offset $current"
    exit 0
  fi
  previous=$current
  sleep 3
done
echo "pqs did not settle in 360s; see: $compose logs scribe" >&2
exit 1
