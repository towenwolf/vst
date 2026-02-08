# Docker Infrastructure (Task 3 + Task 4)

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

2. (Optional) set Stripe keys in `infra/docker/.env`.

3. Start stack:

```bash
infra/docker/scripts/up.sh
```

4. Initialize database:

```bash
infra/docker/scripts/db-reset.sh
infra/docker/scripts/db-migrate.sh
infra/docker/scripts/db-seed.sh
```

5. Tail logs:

```bash
infra/docker/scripts/logs.sh
```

6. Stop stack:

```bash
infra/docker/scripts/down.sh
```

## Notes

- `web` and `api` currently run as scaffold placeholders (`tail -f /dev/null`) until app implementation tasks are completed.
- `stripe-cli` will idle if `STRIPE_API_KEY` is not set.
