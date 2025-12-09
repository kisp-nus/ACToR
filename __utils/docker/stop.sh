#!/usr/bin/env bash
# Stop the dev container and snapshot its writable layer to an image.
#
# What it does
# - Optionally cleans runtime temp dirs inside the container (default: ON) before snapshotting.
# - Runs docker commit on the container, storing to SNAP_TAG.
# - Then stops the container (started with --rm in start.sh).
#
# Notes
# - docker commit captures the container's writable layer only; bind mounts and named volumes are not included.
# - Use changes.sh to preview what would be captured before you stop.
# - Cleanup is conservative: only /tmp and /var/tmp are emptied. Caches (pip/npm/cargo/apt) are preserved.
#
# Env vars
# - IMAGE_BASE          base image name (default: actor-sandbox)
# - CONTAINER_NAME      container name (default: actor-sandbox-cont)
# - SNAP_TAG            snapshot tag (default: ${IMAGE_BASE}:saved)
# - CLEAN_BEFORE_COMMIT set to 0 to skip cleaning (default: 1)
#
# Usage
#   ./stop.sh
#   CLEAN_BEFORE_COMMIT=0 ./stop.sh   # skip temp cleaning
#   SNAP_TAG=myimg:saved ./stop.sh

set -euo pipefail

IMAGE_BASE=${IMAGE_BASE:-actor-sandbox}
CONTAINER_NAME=${CONTAINER_NAME:-actor-sandbox-cont}
SNAP_TAG=${SNAP_TAG:-${IMAGE_BASE}:saved}
CLEAN_BEFORE_COMMIT=${CLEAN_BEFORE_COMMIT:-1}

# If container exists, optionally clean temp dirs, then commit
if docker ps -a --format '{{.Names}}' | grep -Fxq "$CONTAINER_NAME"; then
  if [[ "$CLEAN_BEFORE_COMMIT" == "1" ]]; then
    echo "Cleaning /tmp and /var/tmp inside $CONTAINER_NAME before snapshot..."
    # Attempt cleaning only if container is running; skip silently if not
    if docker inspect -f '{{.State.Running}}' "$CONTAINER_NAME" 2>/dev/null | grep -qi true; then
      docker exec "$CONTAINER_NAME" bash -lc '
        set -e
        for d in /tmp /var/tmp; do
          if [ -d "$d" ]; then
            # Remove all entries at top level, including dotfiles, but not . or ..
            find "$d" -mindepth 1 -maxdepth 1 -exec rm -rf {} + 2>/dev/null || true
          fi
        done
      '
    else
      echo "Container not running; skipping temp cleaning."
    fi
  fi

  echo "Committing $CONTAINER_NAME -> $SNAP_TAG"
  docker commit "$CONTAINER_NAME" "$SNAP_TAG" >/dev/null
else
  echo "Container $CONTAINER_NAME not found; skipping commit"
fi

# Stop container (ignored if already stopped/non-existent)
docker stop "$CONTAINER_NAME" 2>/dev/null || true
