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

## Local Usage

1. Create local env:

```bash
cp infra/docker/.env.example infra/docker/.env
```

2. Start stack:

```bash
infra/docker/scripts/up.sh
```

3. Inspect logs:

```bash
infra/docker/scripts/logs.sh
```

4. Stop stack:

```bash
infra/docker/scripts/down.sh
```

## Service Ports (Defaults)

- Web: `localhost:3000`
- API: `localhost:3001`
- Postgres: `localhost:5432`
- Maildev SMTP: `localhost:1025`
- Maildev UI: `localhost:1080`

## Next Backlog Step

Per `docs/COMMERCE_BACKLOG.md`, move to task 4:
- Build database foundation (migrations/schema for orders/customers/licenses/download tokens/webhook events).
