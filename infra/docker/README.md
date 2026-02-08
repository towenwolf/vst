# Docker Infrastructure (Task 3)

This folder contains local commerce stack infrastructure for development/testing.

## Files

- `docker-compose.yml` — local service composition
- `.env.example` — environment template for local stack
- `scripts/up.sh` — start stack
- `scripts/down.sh` — stop stack
- `scripts/logs.sh` — tail stack logs

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

4. Tail logs:

```bash
infra/docker/scripts/logs.sh
```

5. Stop stack:

```bash
infra/docker/scripts/down.sh
```

## Notes

- `web` and `api` currently run as scaffold placeholders (`tail -f /dev/null`) until app implementation tasks are completed.
- `stripe-cli` will idle if `STRIPE_API_KEY` is not set.
