#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
ENV_FILE="$ROOT_DIR/infra/docker/.env"
SEED_FILE="$ROOT_DIR/apps/api/db/seeds/seed_test_data.sql"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Missing $ENV_FILE"
  echo "Create it from infra/docker/.env.example first."
  exit 1
fi

if [[ ! -f "$SEED_FILE" ]]; then
  echo "Missing seed file: $SEED_FILE"
  exit 1
fi

set -a
source "$ENV_FILE"
set +a

CONTAINER="commerce-postgres"
DB_NAME="${POSTGRES_DB:-commerce}"
DB_USER="${POSTGRES_USER:-commerce}"

exec docker exec -i "$CONTAINER" psql -v ON_ERROR_STOP=1 -U "$DB_USER" -d "$DB_NAME" < "$SEED_FILE"
