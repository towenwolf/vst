#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
ENV_FILE="$ROOT_DIR/infra/docker/.env"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Missing $ENV_FILE"
  echo "Create it from infra/docker/.env.example first."
  exit 1
fi

set -a
source "$ENV_FILE"
set +a

STRIPE_MODE="${STRIPE_MODE:-mock}"

PRODUCT_NAME="${STRIPE_PRODUCT_NAME:-GenX Delay}"
PRODUCT_DESCRIPTION="${STRIPE_PRODUCT_DESCRIPTION:-GenX Delay VST3 one-time purchase}"
PRODUCT_SKU="${STRIPE_PRODUCT_SKU:-genx-delay-vst3}"
PRICE_UNIT_AMOUNT="${STRIPE_PRICE_UNIT_AMOUNT:-4900}"
PRICE_CURRENCY="${STRIPE_PRICE_CURRENCY:-usd}"

extract_json_id() {
  tr -d '\n' | sed -n 's/.*"id"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p'
}

has_api_error() {
  grep -q '"error"[[:space:]]*:'
}

upsert_env_value() {
  local key="$1"
  local value="$2"
  local tmp_file
  tmp_file="$(mktemp)"

  if grep -q "^${key}=" "$ENV_FILE"; then
    awk -v k="$key" -v v="$value" '
      index($0, k "=") == 1 {
        print k "=" v
        next
      }
      { print }
    ' "$ENV_FILE" > "$tmp_file"
  else
    cat "$ENV_FILE" > "$tmp_file"
    printf "\n%s=%s\n" "$key" "$value" >> "$tmp_file"
  fi

  mv "$tmp_file" "$ENV_FILE"
}

if [[ "$STRIPE_MODE" = "mock" ]]; then
  PRODUCT_ID="${STRIPE_PRODUCT_ID:-prod_mock_genx_delay}"
  PRICE_ID="${STRIPE_PRICE_ID:-price_mock_${PRICE_UNIT_AMOUNT}_${PRICE_CURRENCY}}"
  WEBHOOK_SECRET="${STRIPE_WEBHOOK_SECRET:-whsec_mock_local}"

  upsert_env_value "STRIPE_MODE" "mock"
  upsert_env_value "STRIPE_PRODUCT_ID" "$PRODUCT_ID"
  upsert_env_value "STRIPE_PRICE_ID" "$PRICE_ID"
  upsert_env_value "STRIPE_WEBHOOK_SECRET" "$WEBHOOK_SECRET"

  echo "Mock Stripe setup complete (no Stripe account required)."
  echo "STRIPE_MODE=mock"
  echo "STRIPE_PRODUCT_ID=$PRODUCT_ID"
  echo "STRIPE_PRICE_ID=$PRICE_ID"
  echo "STRIPE_WEBHOOK_SECRET=$WEBHOOK_SECRET"
  echo
  echo "When you create a Stripe account later:"
  echo "  1) set STRIPE_MODE=test and STRIPE_API_KEY=sk_test_... in infra/docker/.env"
  echo "  2) rerun infra/docker/scripts/stripe-setup-test-mode.sh"
  exit 0
fi

if [[ "$STRIPE_MODE" != "test" ]]; then
  echo "STRIPE_MODE must be either mock or test (found: $STRIPE_MODE)"
  exit 1
fi

if [[ -z "${STRIPE_API_KEY:-}" ]]; then
  echo "STRIPE_API_KEY is empty in $ENV_FILE"
  echo "Set STRIPE_MODE=mock for no-account local development."
  exit 1
fi

if [[ "${STRIPE_API_KEY}" != sk_test_* ]]; then
  echo "STRIPE_API_KEY must be a Stripe test-mode secret key (sk_test_...)"
  exit 1
fi

PRODUCT_ID="${STRIPE_PRODUCT_ID:-}"
if [[ -n "$PRODUCT_ID" ]]; then
  PRODUCT_CHECK_RESPONSE="$(
    curl -sS -u "${STRIPE_API_KEY}:" "https://api.stripe.com/v1/products/${PRODUCT_ID}"
  )"
  if printf '%s' "$PRODUCT_CHECK_RESPONSE" | has_api_error; then
    echo "Existing STRIPE_PRODUCT_ID=$PRODUCT_ID is invalid; creating a new product."
    PRODUCT_ID=""
  fi
fi

if [[ -z "$PRODUCT_ID" ]]; then
  PRODUCT_RESPONSE="$(
    curl -sS -u "${STRIPE_API_KEY}:" https://api.stripe.com/v1/products \
      -d "name=${PRODUCT_NAME}" \
      -d "description=${PRODUCT_DESCRIPTION}" \
      -d "metadata[sku]=${PRODUCT_SKU}" \
      -d "metadata[source]=commerce-local-setup"
  )"

  if printf '%s' "$PRODUCT_RESPONSE" | has_api_error; then
    echo "Stripe product creation failed:"
    printf '%s\n' "$PRODUCT_RESPONSE"
    exit 1
  fi

  PRODUCT_ID="$(printf '%s' "$PRODUCT_RESPONSE" | extract_json_id)"
  if [[ -z "$PRODUCT_ID" ]]; then
    echo "Unable to parse Stripe product id from API response."
    exit 1
  fi
fi

PRICE_RESPONSE="$(
  curl -sS -u "${STRIPE_API_KEY}:" https://api.stripe.com/v1/prices \
    -d "product=${PRODUCT_ID}" \
    -d "currency=${PRICE_CURRENCY}" \
    -d "unit_amount=${PRICE_UNIT_AMOUNT}" \
    -d "metadata[sku]=${PRODUCT_SKU}" \
    -d "metadata[source]=commerce-local-setup"
)"

if printf '%s' "$PRICE_RESPONSE" | has_api_error; then
  echo "Stripe price creation failed:"
  printf '%s\n' "$PRICE_RESPONSE"
  exit 1
fi

PRICE_ID="$(printf '%s' "$PRICE_RESPONSE" | extract_json_id)"
if [[ -z "$PRICE_ID" ]]; then
  echo "Unable to parse Stripe price id from API response."
  exit 1
fi

upsert_env_value "STRIPE_PRODUCT_ID" "$PRODUCT_ID"
upsert_env_value "STRIPE_PRICE_ID" "$PRICE_ID"
upsert_env_value "STRIPE_MODE" "test"

echo "Stripe test-mode setup complete."
echo "STRIPE_PRODUCT_ID=$PRODUCT_ID"
echo "STRIPE_PRICE_ID=$PRICE_ID"
echo
echo "Next: capture webhook signing secret and write STRIPE_WEBHOOK_SECRET in infra/docker/.env:"
echo "  stripe listen --forward-to localhost:${API_PORT:-3001}${STRIPE_FORWARD_TO_PATH:-/webhooks/stripe} --print-secret"
