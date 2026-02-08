# GenX Delay Hosting and Commerce Plan

> Execution tracker: see `docs/COMMERCE_BACKLOG.md` for the sequential implementation backlog, wishlist, and risk log.
> Parallel marketplace channel: see `docs/MARKETPLACE_PLAN.md` for the Gumroad quick-launch track.

## Scope

Sell/distribute the VST3 plugin through a self-hosted website with:
- One-time purchase
- Guest checkout
- Email delivery links
- Simple licensing
- Local Docker-first development
- Cloud deployment target: Google Cloud
- US-only sales at launch (no Merchant of Record needed)

## Product Decisions (Locked)

- Purchase model: one-time payment
- Checkout model: guest checkout (no required account)
- Fulfillment: email with receipt, license key, and download link
- Licensing: simplest possible (single license key issuance, no machine activation)
- Cloud platform: Google Cloud (Cloud Run + Cloud SQL + Cloud Storage)
- Email provider: Resend or Postmark (transactional)
- Market: US-only at launch (international expansion is a future decision point)

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

## Google Cloud Deployment (Simplified)

Stack (single environment, no separate staging for MVP):
- Cloud Run: API + web (one or two minimal services)
- Cloud SQL (Postgres): smallest managed tier
- Cloud Storage: plugin binary hosting
- Secret Manager: Stripe keys, DB credentials, email API key
- Cloud Logging: observability

Deployment:
1. Provision production GCP project and resources.
2. Deploy containers to Cloud Run.
3. Configure Stripe webhooks (test mode first, then live).
4. Set up DNS + TLS (Cloud Run managed domain or Cloudflare).
5. Validate full purchase flow in Stripe test mode on production infra.
6. Switch Stripe to live mode and launch.

No separate staging environment for MVP. Use Stripe test mode on production
infrastructure to validate before going live. A staging environment can be
added post-launch if needed.

## Business Decisions

- Seller entity: US-based (confirmed for launch).
- Market: US-only at launch. No international VAT/tax complexity.
- International expansion: future decision point. When ready, evaluate
  Lemon Squeezy or FastSpring as Merchant of Record for global tax handling
  vs. adding Stripe Tax to the self-hosted stack.

## Milestones

### Milestone 1: Local Commerce MVP
- Docker stack running (done)
- Stripe test checkout works (done)
- Webhook fulfillment works (done)
- License generation works locally
- Fulfillment email sends locally (maildev)
- Download link works locally

### Milestone 2: Deployed and Live
- Cloud Run + Cloud SQL + Cloud Storage provisioned
- Full purchase flow verified in Stripe test mode on Cloud Run
- Stripe live mode enabled
- Legal/policy pages published
- First real purchase test complete
- Public launch

### Milestone 3: Post-Launch Stabilization
- Uptime monitoring configured
- Cloud SQL backups verified
- Critical bugs patched
- Prepare v2 backlog (international expansion, account portal, etc.)

## Immediate Next Steps

1. Implement licensing in webhook fulfillment flow.
2. Add download delivery endpoint with signed URLs.
3. Add fulfillment email (Resend/Postmark, maildev locally).
4. Polish frontend MVP (landing page, success/cancel pages).
5. Test full flow end-to-end locally.
6. Deploy to Google Cloud and launch.

## Wish List

- Separate staging environment.
- Account portal (optional) for download history and re-send links.
- Upgrade/crossgrade pricing support for future plugin releases.
- Coupon and launch campaign support in Stripe.
- Built-in analytics dashboard for sales, conversion, refunds, and churn signals.
- Automatic release pipeline to upload new plugin builds and update download manifests.
- Basic affiliate/referral tracking.
- Optional machine-bound activation for stronger license enforcement.
- In-app update check endpoint for plugin versions.
- Multi-language storefront support.
- Post-purchase onboarding emails (quick start, install guide, support links).
- International expansion with Merchant of Record (Lemon Squeezy / FastSpring).
- Full security audit (rate limiting, audit logging, webhook replay review).
- Email retry queue and dead-letter handling.

## Issues / Risks

- Guest checkout increases support load for lost-email and link recovery cases.
- Download link abuse risk if links are forwarded; requires strict expiry/rate limits.
- Webhook reliability risk (missed/replayed events) without idempotency + monitoring.
- Email deliverability risk (spam filtering, domain reputation, DNS setup).
- Refund/chargeback handling flow is not yet implemented end-to-end.
- No separate staging environment for MVP launch (mitigated by Stripe test mode on prod).
- No disaster-recovery runbook yet (mitigated by Cloud SQL automated backups post-launch).
