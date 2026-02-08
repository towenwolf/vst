# Marketplace Plan (Gumroad)

> Quick-launch sales channel running in parallel with the self-hosted commerce build.
> See `docs/COMMERCE_PLAN.md` for the self-hosted track.

## Purpose

Get the GenX Delay plugin available for purchase as soon as possible using Gumroad
as a turnkey marketplace, while the self-hosted commerce site is built out in parallel.

Long-term relationship between the two channels (bridge vs. dual-channel) is an
open decision — revisit once the self-hosted site is live and there's sales data
from both.

## Why Gumroad

- No monthly fees — 10% + $0.50 per direct sale.
- Merchant of Record since Jan 2025 — handles all global tax (sales tax, VAT, GST).
- Built-in license key generation for software products.
- Instant digital download delivery.
- No infrastructure to build or maintain.
- Free to set up and list products.

## Fee Breakdown ($49 Plugin)

| Channel | Fee | Net per sale |
|---------|-----|-------------|
| Direct link / profile | 10% + $0.50 ($5.40) | $43.60 |
| Gumroad Discover marketplace | 30% ($14.70) | $34.30 |

Direct links (from your own site, social media, email) are the primary sales
channel. Gumroad Discover is bonus organic traffic at a higher cut.

## Setup Checklist

1. Create Gumroad account and verify email.
2. Link payment method (Stripe, PayPal, or bank account).
3. Create product listing:
   - Product name: GenX Delay
   - Price: $49 (one-time purchase)
   - Upload plugin artifact (VST3 bundle, macOS/Windows as applicable).
   - Write product description, feature summary, and system requirements.
   - Add cover image / screenshots / demo audio or video.
4. Enable license key generation:
   - Go to product content page → three-dot menu → License key.
   - Keys are auto-generated per sale and emailed to buyer.
5. Configure post-purchase experience:
   - Custom receipt message with install instructions.
   - Link to support/contact channel.
6. Test purchase flow:
   - Use Gumroad's test mode or make a real $1 test purchase.
   - Verify download delivery, license key email, and receipt.
7. Publish and share direct link.

## Content Needed

- Product description (features, what it does, who it's for).
- System requirements (OS, DAW compatibility, CPU/RAM).
- Cover image (plugin UI screenshot or marketing graphic).
- Optional: demo audio/video, user testimonials.
- Support contact (email or link to support page).
- Refund policy statement.

## Marketing / Distribution

- Direct link on any existing website, social media, or landing page.
- Share in relevant audio production communities (with community rules in mind).
- Link in plugin UI or documentation if applicable.
- Gumroad Discover provides passive marketplace visibility (30% fee on those sales).

## Open Decisions

- **Pricing:** $49 confirmed, or adjust before listing?
- **Artifacts:** Which platforms to include at launch (macOS only, Windows only, both)?
- **Gumroad Discover:** Opt in for marketplace visibility (30% fee) or keep direct-only (10% fee)?
- **Long-term:** Bridge to self-hosted, dual-channel, or decide later based on data.

## Relationship to Self-Hosted Track

The self-hosted commerce site (`docs/COMMERCE_PLAN.md`) remains the primary long-term
investment. The Gumroad marketplace is a parallel channel that:

- Validates demand and pricing with real sales before the self-hosted site is ready.
- Generates early revenue with near-zero setup effort.
- Provides a fallback if the self-hosted launch timeline slips.
- Can be kept as a secondary channel or sunset once the self-hosted site is live.

Customer data (emails, license keys) lives in Gumroad and would need manual migration
if/when consolidating to the self-hosted platform. Keep this in mind when deciding
the long-term channel strategy.
