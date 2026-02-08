# GenX Delay Hosting and Commerce Plan

## Scope

Sell/distribute the VST3 plugin through a website with:
- One-time purchase
- Guest checkout
- Email delivery links
- Simple licensing
- Local Docker-first development
- Cloud deployment target: Google Cloud

## Product Decisions (Locked)

- Purchase model: one-time payment
- Checkout model: guest checkout (no required account)
- Fulfillment: email links for download
- Licensing: simplest possible (single license key issuance)
- Cloud platform: Google Cloud

## Architecture

- `web` (frontend): product/marketing + buy flow entry
- `api` (backend): Stripe checkout, webhooks, order/license fulfillment, download token handling
- `db` (Postgres): orders, customers (email-based), licenses, download tokens, webhook events
- `storage` (Cloud Storage): plugin artifacts (VST3 bundles, checksums, version metadata)
- `email` provider: transactional email for receipt + download/license link
- `payments`: Stripe Checkout + Webhooks

## Local-First Implementation (Docker)

Use Docker Compose for full local integration testing.

Services:
- `web`
- `api`
- `postgres`
- `maildev` (or equivalent local SMTP capture)
- `stripe-cli` (forward Stripe webhooks to local API)

Deliverables:
1. `docker-compose.yml` for all required services.
2. `.env.example` with all required variables.
3. Seed/migration scripts for local DB setup.
4. End-to-end local test flow:
   - checkout session creation
   - webhook receipt and verification
   - order persistence
   - license generation
   - email dispatch
   - download link validation

## Commerce Flow

1. User clicks Buy from website.
2. API creates Stripe Checkout Session.
3. User pays in Stripe-hosted checkout.
4. Stripe sends `checkout.session.completed` webhook.
5. API verifies webhook signature and idempotency.
6. API creates/updates order record.
7. API generates license key.
8. API generates time-limited signed download token.
9. API emails receipt + license key + download link.
10. User clicks link, API validates token, returns signed Cloud Storage URL.

## Licensing (Simple)

- Key format example: `GENX-XXXX-XXXX-XXXX`.
- Store hashed key in DB (not plaintext).
- No machine activation in initial release.
- No always-online requirement in initial release.
- Optional future phase: plugin-side key validation endpoint or offline verifier.

## Security and Reliability Requirements

- Verify Stripe webhook signatures.
- Use idempotent webhook processing.
- Rate limit fulfillment/download endpoints.
- Use expiring download tokens/URLs.
- Store secrets in environment/secret manager, never in repo.
- Log webhook and fulfillment failures.
- Back up DB and storage metadata.

## Google Cloud Deployment Plan

Recommended stack:
- Cloud Run: `web` and `api` containers
- Cloud SQL (Postgres): persistent data
- Cloud Storage: plugin binary hosting
- Secret Manager: Stripe/email/database secrets
- Cloud Logging + Error Reporting: observability

Deployment phases:
1. Provision staging project/resources.
2. Deploy containers to staging Cloud Run.
3. Configure staging Stripe keys/webhooks.
4. Validate purchase flow end-to-end in Stripe test mode.
5. Provision production resources.
6. Configure production Stripe live mode.
7. Cut over DNS/domain.
8. Run launch checklist and announce.

## Open Business Decision

- Seller country/business entity is not finalized.
- This impacts tax/legal setup (Stripe Tax, invoices, terms).
- Temporary assumption for technical work: US-based setup unless changed.

## Milestones

### Milestone 1: Local Commerce MVP
- Docker stack running
- Stripe test checkout works
- Webhook fulfillment works
- License + email + download links work locally

### Milestone 2: Staging on Google Cloud
- Web/API deployed to Cloud Run
- Cloud SQL + Cloud Storage integrated
- Staging Stripe webhook flow verified

### Milestone 3: Production Readiness
- Security hardening complete
- Monitoring/alerts configured
- Backup strategy verified
- Legal/policy pages published

### Milestone 4: Launch
- Stripe live mode enabled
- First real purchase test complete
- Public release of website + plugin distribution

## Immediate Next Steps

1. Scaffold `web` + `api` projects and Docker Compose.
2. Implement `/checkout` and `/webhooks/stripe` endpoints.
3. Add DB schema for orders/licenses/download tokens.
4. Add email templates for fulfillment.
5. Add signed download token flow backed by local storage, then Cloud Storage.
