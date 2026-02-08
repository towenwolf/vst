# Commerce Development Runbook (Local-First)

This runbook is the onboarding guide for the local commerce stack.

## Source Documents

- Plan: `docs/COMMERCE_PLAN.md`
- Backlog: `docs/COMMERCE_BACKLOG.md`
- Hosting/deployment: `docs/HOSTING.md`

## Current Scaffold

- `apps/web` — frontend scaffold
- `apps/api` — backend scaffold
- `infra/docker` — local stack config and scripts

## Prerequisites

- Docker Desktop (or Docker Engine + Compose)
- Stripe CLI
- Node.js (needed when web/api implementation begins)

## Task 3 Deliverables (Completed)

- `infra/docker/docker-compose.yml`
- `infra/docker/.env.example`
- Startup/teardown helper scripts:
  - `infra/docker/scripts/up.sh`
  - `infra/docker/scripts/down.sh`
  - `infra/docker/scripts/logs.sh`

## Task 4 Deliverables (Completed)

- DB migration scaffold:
  - `apps/api/db/migrations/0001_init.sql`
- Seed fixtures:
  - `apps/api/db/seeds/seed_test_data.sql`
- DB helper scripts:
  - `infra/docker/scripts/db-reset.sh`
  - `infra/docker/scripts/db-migrate.sh`
  - `infra/docker/scripts/db-seed.sh`

## Task 5 Deliverables (Completed)

- Stripe setup helper script:
  - `infra/docker/scripts/stripe-setup-test-mode.sh`
- Stripe env template additions:
  - `STRIPE_MODE`
  - `STRIPE_PRODUCT_ID`
  - `STRIPE_PRICE_ID`
  - `STRIPE_PRODUCT_NAME`
  - `STRIPE_PRODUCT_DESCRIPTION`
  - `STRIPE_PRODUCT_SKU`
  - `STRIPE_PRICE_UNIT_AMOUNT`
  - `STRIPE_PRICE_CURRENCY`
- Compose wiring for API Stripe runtime config:
  - `infra/docker/docker-compose.yml`

## Task 6 Deliverables (Completed)

- API runtime entrypoint:
  - `apps/api/src/server.js`
- Checkout endpoint:
  - `POST /checkout`
- Health endpoint:
  - `GET /health`
- Docker wiring to run API process:
  - `infra/docker/docker-compose.yml`

## Task 7 Deliverables (Completed)

- Webhook endpoint:
  - `POST /webhooks/stripe`
- Signature verification:
  - `x-mock-signature` HMAC in mock mode
  - `Stripe-Signature` verification in test mode
- Idempotency persistence:
  - `webhook_events` insert-once by `stripe_event_id`
- Order/payment state persistence:
  - `checkout.session.completed` -> upsert order/customer state
  - `payment_intent.payment_failed` -> mark order failed
  - `charge.refunded` -> mark order refunded

## Local Usage

1. Create local env:

```bash
cp infra/docker/.env.example infra/docker/.env
```

2. Start stack:

```bash
infra/docker/scripts/up.sh
```

3. Initialize database:

```bash
infra/docker/scripts/db-reset.sh
infra/docker/scripts/db-migrate.sh
infra/docker/scripts/db-seed.sh
```

4. Configure Stripe test mode:

```bash
# no Stripe account required:
STRIPE_MODE=mock
infra/docker/scripts/stripe-setup-test-mode.sh

# optional when a real Stripe test account exists:
STRIPE_MODE=test
STRIPE_API_KEY=sk_test_...
infra/docker/scripts/stripe-setup-test-mode.sh

# then capture webhook signing secret and set STRIPE_WEBHOOK_SECRET in infra/docker/.env
stripe listen --forward-to localhost:3001/webhooks/stripe --print-secret
```

5. Inspect logs:

```bash
infra/docker/scripts/logs.sh
```

6. Stop stack:

```bash
infra/docker/scripts/down.sh
```

## Service Ports (Defaults)

- Web: `localhost:3000`
- API: `localhost:3001`
- Postgres: `localhost:5432`
- Maildev SMTP: `localhost:1025`
- Maildev UI: `localhost:1080`

## Next Backlog Steps

Per `docs/COMMERCE_BACKLOG.md`, the MVP critical path is:

- **Task 8:** Implement simple licensing (generate keys on fulfillment).
- **Task 9:** Implement download delivery (signed URLs, `GET /download/:token`).
- **Task 10:** Implement fulfillment email (Resend/Postmark, maildev locally).
- **Task 11:** Build frontend MVP (landing page, buy button, success/cancel pages).
- **Task 12:** Local end-to-end validation.
- **Task 13:** Deploy to Google Cloud (Cloud Run + Cloud SQL + Cloud Storage).
- **Task 14:** Production launch (Stripe live mode, DNS, first real purchase).
