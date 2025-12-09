# Part of the dev container toolchain: see start.sh (runs from snapshot), stop.sh (commits snapshot).

#!/usr/bin/env bash
# Show filesystem changes in a container relative to its base image
# i.e., what a `docker commit` would capture (excludes bind mounts/volumes).

set -Eeuo pipefail

CONTAINER_NAME=${CONTAINER_NAME:-actor-sandbox-cont}
MODE=summary   # summary | full
FILTER=""      # optional path prefix filter, e.g. /usr/local
LIMIT=0        # 0 = no limit for full mode
SHOW_TYPES=false # try to detect file/dir type via docker exec (can be slow)

usage() {
  cat <<USAGE
Usage: $0 [options]

Options:
  -c, --container NAME   Container name (default: $CONTAINER_NAME)
  -f, --full             Show full changed path list (status + optional type)
  -s, --summary          Show only summary (default)
  -p, --path PREFIX      Filter to paths under PREFIX (e.g., /usr/local)
  -n, --limit N          Limit entries in full mode (0 = unlimited)
  -t, --types            Detect file types via docker exec (slower)
  -h, --help             Show help

Notes:
- This shows differences vs the image the container was created from.
- Data in bind mounts or named volumes is NOT part of `docker commit`, so it is not listed here.
USAGE
}

# Parse args
while [[ $# -gt 0 ]]; do
  case "$1" in
    -c|--container) CONTAINER_NAME="$2"; shift 2;;
    -f|--full) MODE=full; shift;;
    -s|--summary) MODE=summary; shift;;
    -p|--path) FILTER="$2"; shift 2;;
    -n|--limit) LIMIT="$2"; shift 2;;
    -t|--types) SHOW_TYPES=true; shift;;
    -h|--help) usage; exit 0;;
    *) echo "Unknown option: $1" >&2; usage; exit 2;;
  esac
done

# Ensure container exists
if ! docker ps -a --format '{{.Names}}' | grep -Fxq "$CONTAINER_NAME"; then
  echo "Container not found: $CONTAINER_NAME" >&2
  exit 1
fi

# Get image info
IMAGE_ID=$(docker inspect -f '{{.Image}}' "$CONTAINER_NAME")
IMAGE_TAG=$(docker inspect -f '{{.Config.Image}}' "$CONTAINER_NAME")

# Collect diff
DIFF_OUTPUT=$(docker diff "$CONTAINER_NAME" || true)

# Optionally filter by path prefix
if [[ -n "$FILTER" ]]; then
  DIFF_OUTPUT=$(echo "$DIFF_OUTPUT" | awk -v pfx="$FILTER" '$2 ~ "^"pfx')
fi

if [[ -z "$DIFF_OUTPUT" ]]; then
  echo "No filesystem changes relative to base image ($IMAGE_TAG, $IMAGE_ID)."
  exit 0
fi

# Counters
ADDS=$(echo "$DIFF_OUTPUT" | awk '$1=="A"' | wc -l | tr -d ' ')
CHGS=$(echo "$DIFF_OUTPUT" | awk '$1=="C"' | wc -l | tr -d ' ')
DELS=$(echo "$DIFF_OUTPUT" | awk '$1=="D"' | wc -l | tr -d ' ')
TOTAL=$((ADDS+CHGS+DELS))

echo "Container: $CONTAINER_NAME"
echo "Base image: $IMAGE_TAG ($IMAGE_ID)"
if [[ -n "$FILTER" ]]; then echo "Filter: $FILTER"; fi
echo "Changes: $TOTAL  (A: $ADDS, C: $CHGS, D: $DELS)"

# Show top-level path histogram
echo
echo "Top-level paths changed:"
echo "$DIFF_OUTPUT" | awk '{print $2}' | awk -F/ 'NF{print "/"$2}' | \
  sort | uniq -c | sort -nr | sed 's/^/  /'

if [[ "$MODE" == summary ]]; then
  echo
  echo "Use --full to list all changed paths."
  exit 0
fi

# Full listing
echo
printf "%s\n" "Changed entries:" 
LIST="$DIFF_OUTPUT"
if [[ "$LIMIT" -gt 0 ]]; then
  LIST=$(echo "$LIST" | head -n "$LIMIT")
fi

if [[ "$SHOW_TYPES" == true ]]; then
  while read -r line; do
    [ -z "$line" ] && continue
    status=$(echo "$line" | awk '{print $1}')
    path=$(echo "$line" | cut -d' ' -f2-)
    # Query type inside container; suppress errors for deleted items
    typ=$(docker exec "$CONTAINER_NAME" bash -lc "test -d '$path' && echo dir || (test -f '$path' && echo file || (test -L '$path' && echo link || echo ?))" 2>/dev/null || true)
    printf "%s %s [%s]\n" "$status" "$path" "$typ"
  done <<< "$LIST"
else
  echo "$LIST"
fi

if [[ "$LIMIT" -gt 0 ]]; then
  REM=$((TOTAL - LIMIT))
  if [[ $REM -gt 0 ]]; then
    echo "... ($REM more entries omitted; use --limit 0 to show all)"
  fi
fi

# Hints
cat <<HINT

Hints:
- docker diff excludes bind mounts and volumes; docker commit does too.
- To persist these changes, run your stop script (which commits) or manually: docker commit $CONTAINER_NAME ${IMAGE_TAG}-manual
HINT
