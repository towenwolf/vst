#!/usr/bin/env bash
set -euo pipefail

API_BASE_URL="${API_BASE_URL:-http://localhost:3001}"
WEBHOOK_SECRET="${WEBHOOK_SECRET:-whsec_mock_local}"

if ! command -v curl >/dev/null 2>&1; then
  echo "curl is required"
  exit 1
fi

if ! command -v openssl >/dev/null 2>&1; then
  echo "openssl is required"
  exit 1
fi

if ! command -v node >/dev/null 2>&1; then
  echo "node is required"
  exit 1
fi

run_id="$(date +%s)"
checkout_session_id="cs_test_checkout_${run_id}"
payment_intent_id="pi_test_checkout_${run_id}"

echo "Running API smoke tests against: ${API_BASE_URL}"

assert_json_field_eq() {
  local json="$1"
  local field_path="$2"
  local expected="$3"
  node -e '
const payload = JSON.parse(process.argv[1]);
const path = process.argv[2].split(".");
const expected = process.argv[3];
let v = payload;
for (const part of path) {
  v = v?.[part];
}
if (String(v) !== expected) {
  console.error(`Assertion failed for ${process.argv[2]}: expected ${expected}, got ${v}`);
  process.exit(1);
}
' "$json" "$field_path" "$expected"
}

post_json() {
  local url="$1"
  local payload="$2"
  curl -sS -X POST "$url" -H 'Content-Type: application/json' -d "$payload"
}

# 1) Health
health_response="$(curl -sS "${API_BASE_URL}/health")"
assert_json_field_eq "$health_response" "ok" "true"
echo "PASS health"

# 2) Checkout success
checkout_response="$(post_json "${API_BASE_URL}/checkout" '{"customerEmail":"buyer@example.com","quantity":1,"successUrl":"http://localhost:3000/checkout/success","cancelUrl":"http://localhost:3000/checkout/cancel","productSku":"genx-delay-vst3","pluginVersion":"0.1.0"}')"
assert_json_field_eq "$checkout_response" "provider" "mock"
echo "PASS checkout success"

# 3) Checkout validation
checkout_invalid_response="$(post_json "${API_BASE_URL}/checkout" '{"quantity":0}')"
assert_json_field_eq "$checkout_invalid_response" "error" "quantity must be an integer between 1 and 10"
echo "PASS checkout validation"

# 4) Webhook checkout.session.completed (signed)
checkout_event_payload="{\"id\":\"evt_test_checkout_${run_id}\",\"type\":\"checkout.session.completed\",\"livemode\":false,\"data\":{\"object\":{\"id\":\"${checkout_session_id}\",\"customer_email\":\"buyer@example.com\",\"customer_details\":{\"email\":\"buyer@example.com\",\"name\":\"Buyer One\"},\"payment_status\":\"paid\",\"currency\":\"usd\",\"amount_total\":4900,\"payment_intent\":\"${payment_intent_id}\",\"metadata\":{\"product_sku\":\"genx-delay-vst3\",\"plugin_version\":\"0.1.0\"}}}}"
checkout_event_sig="$(printf '%s' "$checkout_event_payload" | openssl dgst -sha256 -hmac "$WEBHOOK_SECRET" -binary | xxd -p -c 256)"
checkout_event_response="$(curl -sS -X POST "${API_BASE_URL}/webhooks/stripe" -H 'Content-Type: application/json' -H "x-mock-signature: ${checkout_event_sig}" -d "$checkout_event_payload")"
assert_json_field_eq "$checkout_event_response" "received" "true"
assert_json_field_eq "$checkout_event_response" "idempotent" "false"
assert_json_field_eq "$checkout_event_response" "processingStatus" "processed"
echo "PASS webhook checkout completed"

# 5) Replay idempotency
checkout_event_replay_response="$(curl -sS -X POST "${API_BASE_URL}/webhooks/stripe" -H 'Content-Type: application/json' -H "x-mock-signature: ${checkout_event_sig}" -d "$checkout_event_payload")"
assert_json_field_eq "$checkout_event_replay_response" "idempotent" "true"
assert_json_field_eq "$checkout_event_replay_response" "processingStatus" "ignored"
echo "PASS webhook idempotency"

# 6) payment_intent.payment_failed
failed_event_payload="{\"id\":\"evt_test_failed_${run_id}\",\"type\":\"payment_intent.payment_failed\",\"livemode\":false,\"data\":{\"object\":{\"id\":\"${payment_intent_id}\"}}}"
failed_event_sig="$(printf '%s' "$failed_event_payload" | openssl dgst -sha256 -hmac "$WEBHOOK_SECRET" -binary | xxd -p -c 256)"
failed_event_response="$(curl -sS -X POST "${API_BASE_URL}/webhooks/stripe" -H 'Content-Type: application/json' -H "x-mock-signature: ${failed_event_sig}" -d "$failed_event_payload")"
assert_json_field_eq "$failed_event_response" "processingStatus" "processed"
echo "PASS webhook payment failed"

# 7) Signature failure
bad_sig_response="$(curl -sS -X POST "${API_BASE_URL}/webhooks/stripe" -H 'Content-Type: application/json' -H 'x-mock-signature: bad' -d '{"id":"evt_bad_sig_'"${run_id}"'","type":"checkout.session.completed","livemode":false,"data":{"object":{"id":"cs_bad"}}}')"
assert_json_field_eq "$bad_sig_response" "error" "Invalid mock webhook signature"
echo "PASS webhook signature rejection"

echo "All API smoke tests passed."
