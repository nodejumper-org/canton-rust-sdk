#!/bin/sh
# Wait until the Scribe store answers: it applies its schema and reads the
# ledger from genesis before `latest_offset()` exists, and a query before that
# fails with "function active(...) does not exist".
set -eu
compose="docker compose -f $(dirname "$0")/compose.yaml"
for _ in $(seq 1 90); do
  if $compose exec -T postgres psql -U pqs -d pqs -tAc 'select latest_offset()' 2>/dev/null | grep -qE '^[0-9]+$'; then
    echo "pqs ready at offset $($compose exec -T postgres psql -U pqs -d pqs -tAc 'select latest_offset()')"
    exit 0
  fi
  sleep 2
done
echo "pqs did not come up in 180s; see: $compose logs scribe" >&2
exit 1
