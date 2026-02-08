# Commerce API

This folder will contain the commerce backend (`api`) for GenX Delay.

Implemented endpoints:
- `GET /health`
- `POST /checkout`

`POST /checkout` behavior:
- `STRIPE_MODE=mock`: returns a deterministic mock checkout session payload without Stripe account access.
- `STRIPE_MODE=test`: creates a real Stripe Checkout Session in test mode.

Request example:

```json
{
  "customerEmail": "buyer@example.com",
  "quantity": 1,
  "successUrl": "http://localhost:3000/checkout/success",
  "cancelUrl": "http://localhost:3000/checkout/cancel",
  "productSku": "genx-delay-vst3",
  "pluginVersion": "0.1.0"
}
```

Planned responsibilities:
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
