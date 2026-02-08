# Commerce Backlog

This backlog translates `docs/COMMERCE_PLAN.md` into a sequential execution list.

## Sequential Backlog

1. Finalize business/legal baseline
- Confirm seller country/entity (temporary US assumption if unresolved).
- Define legal pages: Terms, Privacy, Refund, License Agreement.
- Confirm support contact channel.

2. ~~Scaffold repository structure for commerce stack~~
- Create `apps/web`, `apps/api`, `infra/docker`.
- Add root-level runbook for local commerce development.

3. ~~Implement local Docker environment~~
- Add `docker-compose.yml` with `web`, `api`, `postgres`, `maildev`, `stripe-cli`.
- Add `.env.example` with required configuration.
- Add scripts for local startup/teardown.

4. ~~Build database foundation~~
- Create migrations/schema for:
- `orders`
- `customers` (email-based)
- `licenses`
- `download_tokens`
- `webhook_events`
- Add seed/test fixtures.

5. ~~Configure Stripe in test mode~~
- Create Stripe Product + one-time Price.
- Configure API keys/webhook secret.
- Document Stripe setup steps for contributors.

6. ~~Build checkout endpoint~~
- Implement `POST /checkout`.
- Create Stripe Checkout Session for guest purchase flow.
- Attach metadata required for fulfillment.

7. ~~Build webhook fulfillment endpoint~~
- Implement `POST /webhooks/stripe`.
- Verify webhook signatures.
- Enforce idempotent event processing.
- Persist order/payment event state.

8. Implement simple licensing
- Generate keys (`GENX-XXXX-XXXX-XXXX` format).
- Store hashed keys (never plaintext).
- Link license record to fulfilled order.

9. Implement secure download flow
- Generate expiring download tokens.
- Implement `GET /download/:token`.
- Validate token and return signed storage URL.
- Add rate limiting and abuse guards.

10. Implement fulfillment email flow
- Add templates for receipt + license + download link.
- Wire to local mail capture (`maildev`) and then real provider.
- Add retry/error handling for failed sends.

11. Build frontend MVP
- Product/marketing page.
- Buy CTA wired to checkout endpoint.
- Success/cancel pages.
- Basic support/FAQ links.

12. Implement artifact management
- Define binary naming/version/checksum conventions.
- Add release artifact upload path (local-first, cloud-ready).

13. Security hardening pass
- Secret management hygiene.
- Rate limits on sensitive endpoints.
- Webhook replay protection verification.
- Audit logging for commerce-critical events.

14. Local end-to-end validation
- Test complete flow in Stripe test mode:
- checkout -> webhook -> order -> license -> email -> download.
- Record test checklist and known gaps.

15. Provision Google Cloud staging
- Cloud Run (`web`, `api`).
- Cloud SQL (Postgres).
- Cloud Storage (artifacts).
- Secret Manager + logging.

16. Deploy and validate staging
- Deploy containers.
- Configure staging Stripe webhooks.
- Run staging commerce smoke tests.

17. Production readiness checklist
- Backups + restore runbook.
- Monitoring and alerting.
- Legal/policy pages finalized.

18. Provision and deploy production
- Deploy production resources/services.
- Configure Stripe live mode keys/webhooks.
- DNS + TLS cutover.

19. Launch validation
- Execute first real purchase test.
- Verify fulfillment email and download path.
- Verify support workflow.

20. Post-launch stabilization
- Triage early incidents.
- Patch high-priority reliability/security items.
- Prepare v2 backlog from wishlist.

## Wish List

- Optional account portal for download history and link re-send.
- Upgrade/crossgrade pricing support.
- Coupons and launch campaigns via Stripe.
- Sales/conversion/refund analytics dashboard.
- Automated plugin release upload + manifest update pipeline.
- Affiliate/referral tracking.
- Optional machine-bound activation.
- In-plugin update check endpoint.
- Multi-language storefront.
- Post-purchase onboarding email sequence.

## Issues / Risks

- Business entity/country unresolved (tax/legal impact).
- Guest checkout may increase support load for lost emails.
- Download links may be forwarded/abused without strong controls.
- Webhook reliability risk without strict idempotency + observability.
- Binary integrity risk without clear checksum/signing UX.
- Secret/config drift risk between local, staging, and production.
- Email deliverability risk (DNS, domain reputation, spam filtering).
- Refund/chargeback workflow not fully implemented yet.
- No explicit disaster recovery runbook yet.
- Launch schedule depends on complete staging/prod smoke tests.
