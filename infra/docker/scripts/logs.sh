#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
COMPOSE_FILE="$ROOT_DIR/infra/docker/docker-compose.yml"
ENV_FILE="$ROOT_DIR/infra/docker/.env"

exec docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" logs -f "$@"
