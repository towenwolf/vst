# Docker Infrastructure (Task 3 + Task 4 + Task 5)

This folder contains local commerce stack infrastructure for development/testing.

## Files

- `docker-compose.yml` — local service composition
- `.env.example` — environment template for local stack
- `scripts/up.sh` — start stack
- `scripts/down.sh` — stop stack
- `scripts/logs.sh` — tail stack logs
- `scripts/db-reset.sh` — reset DB schema
- `scripts/db-migrate.sh` — apply migrations
- `scripts/db-seed.sh` — load seed fixtures
- `scripts/stripe-setup-test-mode.sh` — configure Stripe in `mock` (no account) or `test` (real Stripe) mode

## Local Services

- `web` (scaffold container)
- `api` (scaffold container)
- `postgres`
- `maildev`
- `stripe-cli`

## Setup

1. Copy env template:

```bash
cp infra/docker/.env.example infra/docker/.env
```

2. Configure Stripe mode in `infra/docker/.env`:

```bash
STRIPE_MODE=mock
```

3. Run Stripe setup helper:

```bash
infra/docker/scripts/stripe-setup-test-mode.sh
```

4. Optional real Stripe setup (only when you have an account):

```bash
STRIPE_MODE=test
STRIPE_API_KEY=sk_test_...
infra/docker/scripts/stripe-setup-test-mode.sh
stripe listen --forward-to localhost:3001/webhooks/stripe --print-secret
```

5. Start stack:

```bash
infra/docker/scripts/up.sh
```

6. Initialize database:

```bash
infra/docker/scripts/db-reset.sh
infra/docker/scripts/db-migrate.sh
infra/docker/scripts/db-seed.sh
```

7. Tail logs:

```bash
infra/docker/scripts/logs.sh
```

8. Stop stack:

```bash
infra/docker/scripts/down.sh
```

## Notes

- `web` and `api` currently run as scaffold placeholders (`tail -f /dev/null`) until app implementation tasks are completed.
- `stripe-cli` will idle unless `STRIPE_API_KEY` is set.
- API Stripe config is sourced from `STRIPE_MODE`, `STRIPE_API_KEY`, `STRIPE_WEBHOOK_SECRET`, `STRIPE_PRODUCT_ID`, `STRIPE_PRICE_ID`.
