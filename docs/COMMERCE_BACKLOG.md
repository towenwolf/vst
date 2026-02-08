# Commerce Backlog

This backlog translates `docs/COMMERCE_PLAN.md` into a sequential execution list.

Scope: US-only sales at launch. Self-hosted stack with Stripe. No Merchant of Record
needed until international expansion. If/when international sales become a priority,
evaluate Lemon Squeezy or FastSpring as Merchant of Record to handle global VAT/tax.

## Completed

1. ~~Finalize business/legal baseline~~
2. ~~Scaffold repository structure for commerce stack~~
3. ~~Implement local Docker environment~~
4. ~~Build database foundation~~
5. ~~Configure Stripe in test mode~~
6. ~~Build checkout endpoint~~
7. ~~Build webhook fulfillment endpoint~~

## MVP Backlog

8. Implement simple licensing
   - Generate keys (`GENX-XXXX-XXXX-XXXX` format).
   - Store hashed keys (never plaintext).
   - Link license record to fulfilled order.
   - Wire into webhook fulfillment so license is generated on `checkout.session.completed`.

9. Implement download delivery
   - On fulfillment, generate a signed Cloud Storage URL (expiring, single-use-ish).
   - Implement `GET /download/:token` that validates and redirects to signed URL.
   - Store plugin artifacts in a local volume for dev, Cloud Storage for prod.
   - Basic rate limiting on the download endpoint.

10. Implement fulfillment email
    - Use a transactional email service (Resend or Postmark).
    - Single email template: receipt + license key + download link.
    - Wire to `maildev` locally, real provider in production.
    - Basic error logging on send failure (retry is a wishlist item).

11. Build frontend MVP
    - Product/marketing landing page.
    - Buy button wired to `POST /checkout`.
    - Post-checkout success page (shows license key + download link).
    - Cancel/error page.
    - Minimal legal footer (Terms, Privacy, Refund Policy links).

12. Local end-to-end validation
    - Test full flow in Stripe test mode: checkout -> webhook -> order -> license -> email -> download.
    - Record test checklist and known gaps.

13. Deploy to Google Cloud
    - Single Cloud Run service (API serves both web and API, or two minimal services).
    - Cloud SQL (Postgres managed instance, smallest tier).
    - Cloud Storage bucket for plugin artifacts.
    - Secret Manager for Stripe keys, DB credentials, email API key.
    - Cloud Logging for observability.
    - Configure Stripe webhooks to point at Cloud Run URL.
    - DNS + TLS via Cloud Run managed domain or a simple Cloudflare setup.

14. Production launch
    - Switch Stripe to live mode, configure live webhook endpoint.
    - Finalize legal/policy pages (Terms, Privacy, Refund).
    - Run first real purchase test end-to-end.
    - Verify email delivery, download link, and license key.
    - Go live.

## Post-Launch

15. Post-launch stabilization
    - Monitor for webhook failures, email bounces, download issues.
    - Patch any critical bugs.
    - Set up basic uptime monitoring (Cloud Run health check or UptimeRobot).
    - Set up Cloud SQL automated backups.

## Wish List

- Separate staging environment (for now, use Stripe test mode on production infra).
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
- Email retry/dead-letter queue for failed sends.
- Full security hardening pass (audit logging, webhook replay protection review).
- International sales expansion (evaluate Lemon Squeezy / FastSpring as MoR).
- Stripe Tax integration for multi-state US tax compliance.

## Issues / Risks

- Business entity/country unresolved (tax/legal impact for expansion).
- Guest checkout may increase support load for lost emails.
- Download links may be forwarded/abused without strong controls.
- Webhook reliability risk without strict idempotency + observability.
- Email deliverability risk (DNS, domain reputation, spam filtering).
- Refund/chargeback workflow not fully implemented yet.
- No separate staging environment for MVP (using Stripe test mode on prod infra).
