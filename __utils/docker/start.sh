#!/usr/bin/env bash
# Start the dev container with persistent volumes, bind mounts, snapshot support, and tmpfs for runtime dirs.
#
# Key behaviors
# - Uses a saved snapshot image if present (${IMAGE_BASE}:saved), otherwise the base image.
# - Binds project repo to /data, agent utilities to /utils (with rslave for nested SSHFS), and web UI to /theshell.
# - Creates/uses named volumes for /home/codespace, /usr/local, and /etc.
# - Mounts tmpfs for /tmp and /run so runtime noise is not captured in snapshots.
# - Exposes ports: 2223 (mapped to 18523).
#
# Nested SSHFS mounts
# - /utils is mounted with bind-propagation=rslave so SSHFS mounts under the host path appear inside the container.
# - Ensure FUSE mount has allow_other and host path is rshared if needed. See SSHFS.md.
#
# Env vars
# - IMAGE_BASE       base image tag (default: actor-sandbox)
# - CONTAINER_NAME   container name (default: actor-sandbox-cont)
# - SNAP_TAG         snapshot tag to prefer (default: ${IMAGE_BASE}:saved)
#
# Usage
#   ./start.sh
#   IMAGE_BASE=myimg CONTAINER_NAME=mycont ./start.sh

set -euo pipefail

IMAGE_BASE=${IMAGE_BASE:-actor-sandbox}
CONTAINER_NAME=${CONTAINER_NAME:-actor-sandbox-cont}
SNAP_TAG=${SNAP_TAG:-${IMAGE_BASE}:saved}

# Optional extra docker flags (e.g., CAP_SYS_ADMIN fallback for bwrap)
EXTRA_DOCKER_FLAGS=""

# Resource limits
# - MEM_LIMIT: hard memory limit for the container (default 56g)
# - MEM_SWAP_LIMIT: total memory+swap (set equal to MEM_LIMIT to effectively disable swap growth)
# - ROOTFS_SIZE: optional root filesystem size (e.g., 256G). Works only on certain storage drivers.
MEM_LIMIT=${MEM_LIMIT:-56g}
MEM_SWAP_LIMIT=${MEM_SWAP_LIMIT:-${MEM_LIMIT}}
ROOTFS_SIZE=${ROOTFS_SIZE:-}

# Resolve important host paths, allowing overrides
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"


IMAGE_TO_RUN="$IMAGE_BASE"
# if docker image inspect "$SNAP_TAG" >/dev/null 2>&1; then
#   echo "Starting from snapshot image: $SNAP_TAG"
#   IMAGE_TO_RUN="$SNAP_TAG"
# else
#   echo "Snapshot not found; starting from base image: $IMAGE_BASE"
# fi

# Preflight: advise on user namespaces if host disallows unprivileged creation.
# Docker does not allow setting kernel.unprivileged_userns_clone via --sysctl; it must be set on the host.
if sysctl -a 2>/dev/null | grep -q '^kernel\.unprivileged_userns_clone'; then
  if [ "$(sysctl -n kernel.unprivileged_userns_clone 2>/dev/null || echo 1)" = "0" ]; then
    echo "Warning: host kernel.unprivileged_userns_clone=0; unprivileged user namespaces (bwrap) may fail."
    echo "To enable globally on the host: sudo sysctl -w kernel.unprivileged_userns_clone=1"
    echo "Persist: echo kernel.unprivileged_userns_clone=1 | sudo tee /etc/sysctl.d/99-unpriv-userns.conf && sudo sysctl --system"
    if [ "${BWRAP_ALLOW_CAP_SYS_ADMIN:-0}" = "1" ]; then
      echo "Opt-in fallback enabled: adding --cap-add SYS_ADMIN so bwrap can create namespaces (reduced isolation)."
      EXTRA_DOCKER_FLAGS+=" --cap-add SYS_ADMIN"
    fi
  fi
fi

# Start container; use rslave propagation for /utils to see nested SSHFS mounts
# Add ":rshared" instead if you need two-way propagation.
RESOURCE_FLAGS=(
  --memory "${MEM_LIMIT}"
  --memory-swap "${MEM_SWAP_LIMIT}"
)

# Optional rootfs size (only on supported storage drivers, e.g., overlay2 on XFS with pquota)
if [ -n "${ROOTFS_SIZE}" ]; then
  RESOURCE_FLAGS+=( --storage-opt "size=${ROOTFS_SIZE}" )
fi

docker run --rm -d --init -it --name "$CONTAINER_NAME" ${EXTRA_DOCKER_FLAGS} "${RESOURCE_FLAGS[@]}" \
    --userns=host \
    --security-opt apparmor=unconfined \
    --security-opt seccomp="$(pwd)"/seccomp.json \
    --mount type=bind,source="$(pwd)"/../../,target=/data/ \
    --mount type=volume,source=actor-sandhome,target=/home/codespace/ \
    --mount type=volume,source=actor-sandusrlocal,target=/usr/local/ \
    --mount type=volume,source=actor-sandetc,target=/etc/ \
    --tmpfs /tmp:rw,exec,nosuid,nodev \
    --tmpfs /run:rw,nosuid,nodev \
    -p 18523:2223 "$IMAGE_TO_RUN" /bin/bash
