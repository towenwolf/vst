# API App Scaffold

This folder will contain the commerce backend (`api`) for GenX Delay.

Planned responsibilities:
- Stripe checkout session creation
- Stripe webhook handling and fulfillment
- License key issuance
- Download token validation/signing
- Email fulfillment orchestration

Task 5 setup inputs expected by the API container:
- `STRIPE_MODE` (`mock` or `test`)
- `STRIPE_API_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `STRIPE_PRODUCT_ID`
- `STRIPE_PRICE_ID`

Implementation details are tracked in `docs/COMMERCE_PLAN.md` and `docs/COMMERCE_BACKLOG.md`.
