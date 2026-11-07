#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "Building latest WS2300 release binary via Docker and exporting to ${REPO_ROOT}/bin ..."
docker build \
  -f "${REPO_ROOT}/docker/Dockerfile" \
  --target export-bin \
  --output "type=local,dest=${REPO_ROOT}/bin" \
  "${REPO_ROOT}"

chmod +x "${REPO_ROOT}/bin/ws2300"
echo "Done! Binary deployed to:"
ls -lh "${REPO_ROOT}/bin/ws2300"

