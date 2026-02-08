#!/usr/bin/env node
const http = require('node:http');
const { randomUUID, createHmac, timingSafeEqual } = require('node:crypto');
const { Pool } = require('pg');

const PORT = Number(process.env.PORT || 3001);
const DATABASE_URL = process.env.DATABASE_URL || '';
const STRIPE_MODE = (process.env.STRIPE_MODE || 'mock').toLowerCase();
const STRIPE_API_KEY = process.env.STRIPE_API_KEY || '';
const STRIPE_PRICE_ID = process.env.STRIPE_PRICE_ID || '';
const STRIPE_WEBHOOK_SECRET = process.env.STRIPE_WEBHOOK_SECRET || '';
const DEFAULT_PRODUCT_SKU = process.env.STRIPE_PRODUCT_SKU || 'genx-delay-vst3';
const DEFAULT_PLUGIN_VERSION = process.env.PLUGIN_VERSION || '0.1.0';
const DEFAULT_SUCCESS_URL = process.env.CHECKOUT_SUCCESS_URL || 'http://localhost:3000/checkout/success';
const DEFAULT_CANCEL_URL = process.env.CHECKOUT_CANCEL_URL || 'http://localhost:3000/checkout/cancel';
const MOCK_CHECKOUT_BASE_URL = process.env.MOCK_CHECKOUT_BASE_URL || 'http://localhost:3001/mock-checkout';

const pool = DATABASE_URL ? new Pool({ connectionString: DATABASE_URL }) : null;

function sendJson(res, statusCode, body) {
  const payload = JSON.stringify(body);
  res.writeHead(statusCode, {
    'Content-Type': 'application/json; charset=utf-8',
    'Content-Length': Buffer.byteLength(payload),
  });
  res.end(payload);
}

function isValidHttpUrl(value) {
  if (!value || typeof value !== 'string') return false;
  try {
    const parsed = new URL(value);
    return parsed.protocol === 'http:' || parsed.protocol === 'https:';
  } catch (_) {
    return false;
  }
}

function parseQuantity(value) {
  if (value === undefined || value === null) return 1;
  const quantity = Number(value);
  if (!Number.isInteger(quantity) || quantity < 1 || quantity > 10) {
    return null;
  }
  return quantity;
}

function readRawBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    let total = 0;

    req.on('data', chunk => {
      chunks.push(chunk);
      total += chunk.length;
      if (total > 1024 * 256) {
        reject(new Error('Request body too large'));
        req.destroy();
      }
    });

    req.on('end', () => {
      resolve(Buffer.concat(chunks).toString('utf8'));
    });

    req.on('error', reject);
  });
}

function parseJson(rawBody) {
  if (!rawBody) return {};
  return JSON.parse(rawBody);
}

function buildMetadata(payload) {
  return {
    product_sku: payload.productSku || DEFAULT_PRODUCT_SKU,
    plugin_version: payload.pluginVersion || DEFAULT_PLUGIN_VERSION,
    checkout_mode: STRIPE_MODE,
    source: 'genx-commerce-api',
  };
}

function createMockWebhookSignature(rawBody) {
  return createHmac('sha256', STRIPE_WEBHOOK_SECRET).update(rawBody).digest('hex');
}

function safeCompareHex(a, b) {
  const aBuf = Buffer.from(a, 'utf8');
  const bBuf = Buffer.from(b, 'utf8');
  if (aBuf.length !== bBuf.length) {
    return false;
  }
  return timingSafeEqual(aBuf, bBuf);
}

function parseStripeSignatureHeader(headerValue) {
  const parsed = { timestamp: null, signatures: [] };
  if (!headerValue) return parsed;

  headerValue.split(',').forEach(part => {
    const [key, value] = part.split('=');
    if (key === 't') {
      parsed.timestamp = value;
    }
    if (key === 'v1') {
      parsed.signatures.push(value);
    }
  });

  return parsed;
}

function verifyWebhookSignature(rawBody, req) {
  if (!STRIPE_WEBHOOK_SECRET) {
    return { ok: false, error: 'STRIPE_WEBHOOK_SECRET is not configured' };
  }

  if (STRIPE_MODE === 'mock') {
    const header = String(req.headers['x-mock-signature'] || '');
    if (!header) {
      return { ok: false, error: 'Missing x-mock-signature header' };
    }
    const expected = createMockWebhookSignature(rawBody);
    return { ok: safeCompareHex(header, expected), error: 'Invalid mock webhook signature' };
  }

  if (STRIPE_MODE === 'test') {
    const header = String(req.headers['stripe-signature'] || '');
    const parsed = parseStripeSignatureHeader(header);
    if (!parsed.timestamp || parsed.signatures.length === 0) {
      return { ok: false, error: 'Missing or malformed Stripe-Signature header' };
    }

    const signedPayload = `${parsed.timestamp}.${rawBody}`;
    const expected = createHmac('sha256', STRIPE_WEBHOOK_SECRET)
      .update(signedPayload)
      .digest('hex');

    const match = parsed.signatures.some(sig => safeCompareHex(sig, expected));
    return { ok: match, error: 'Invalid Stripe webhook signature' };
  }

  return { ok: false, error: `Unsupported STRIPE_MODE: ${STRIPE_MODE}` };
}

function normalizeCurrency(value) {
  const currency = String(value || 'USD').trim().toUpperCase();
  if (currency.length === 3) return currency;
  return 'USD';
}

function normalizeEmailFromSession(session) {
  const email = session?.customer_details?.email || session?.customer_email;
  if (email && String(email).includes('@')) {
    return String(email);
  }
  return `unknown+${session.id || randomUUID()}@example.local`;
}

async function withTransaction(callback) {
  if (!pool) {
    throw new Error('DATABASE_URL is not configured');
  }

  const client = await pool.connect();
  try {
    await client.query('BEGIN');
    const result = await callback(client);
    await client.query('COMMIT');
    return result;
  } catch (error) {
    await client.query('ROLLBACK');
    throw error;
  } finally {
    client.release();
  }
}

async function insertWebhookEventIfNew(client, event) {
  const result = await client.query(
    `
      INSERT INTO webhook_events (stripe_event_id, event_type, livemode, payload, processing_status)
      VALUES ($1, $2, $3, $4::jsonb, 'received')
      ON CONFLICT (stripe_event_id) DO NOTHING
      RETURNING id
    `,
    [event.id, event.type, Boolean(event.livemode), JSON.stringify(event)],
  );

  return result.rows[0] || null;
}

async function markWebhookEvent(client, stripeEventId, status, errorMessage) {
  await client.query(
    `
      UPDATE webhook_events
      SET processing_status = $2,
          processing_error = $3,
          processed_at = CASE
            WHEN $2 IN ('processed', 'ignored', 'failed') THEN NOW()
            ELSE processed_at
          END,
          updated_at = NOW()
      WHERE stripe_event_id = $1
    `,
    [stripeEventId, status, errorMessage || null],
  );
}

async function upsertOrderFromCheckoutSession(client, event) {
  const session = event.data.object;
  if (!session || !session.id) {
    throw new Error('checkout.session.completed missing session id');
  }

  const customerEmail = normalizeEmailFromSession(session);
  const customerName = session?.customer_details?.name || null;
  const stripeCustomerId = typeof session.customer === 'string' ? session.customer : null;

  const customerResult = await client.query(
    `
      INSERT INTO customers (email, full_name, stripe_customer_id, metadata)
      VALUES ($1, $2, $3, $4::jsonb)
      ON CONFLICT (email)
      DO UPDATE SET
        full_name = COALESCE(EXCLUDED.full_name, customers.full_name),
        stripe_customer_id = COALESCE(EXCLUDED.stripe_customer_id, customers.stripe_customer_id),
        metadata = customers.metadata || EXCLUDED.metadata,
        updated_at = NOW()
      RETURNING id
    `,
    [
      customerEmail,
      customerName,
      stripeCustomerId,
      JSON.stringify({ source: 'stripe-webhook', stripe_event_id: event.id }),
    ],
  );

  const customerId = customerResult.rows[0].id;
  const paymentIntentId = typeof session.payment_intent === 'string' ? session.payment_intent : null;
  const orderStatus = session.payment_status === 'paid' ? 'paid' : 'pending';
  const fulfilledAt = orderStatus === 'paid' ? new Date().toISOString() : null;
  const productSku = session?.metadata?.product_sku || DEFAULT_PRODUCT_SKU;
  const pluginVersion = session?.metadata?.plugin_version || DEFAULT_PLUGIN_VERSION;

  await client.query(
    `
      INSERT INTO orders (
        customer_id,
        stripe_checkout_session_id,
        stripe_payment_intent_id,
        status,
        currency,
        amount_cents,
        product_sku,
        plugin_version,
        fulfilled_at,
        metadata
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::jsonb)
      ON CONFLICT (stripe_checkout_session_id)
      DO UPDATE SET
        customer_id = EXCLUDED.customer_id,
        stripe_payment_intent_id = COALESCE(EXCLUDED.stripe_payment_intent_id, orders.stripe_payment_intent_id),
        status = EXCLUDED.status,
        currency = EXCLUDED.currency,
        amount_cents = EXCLUDED.amount_cents,
        product_sku = EXCLUDED.product_sku,
        plugin_version = EXCLUDED.plugin_version,
        fulfilled_at = EXCLUDED.fulfilled_at,
        metadata = orders.metadata || EXCLUDED.metadata,
        updated_at = NOW()
    `,
    [
      customerId,
      session.id,
      paymentIntentId,
      orderStatus,
      normalizeCurrency(session.currency),
      Number.isInteger(session.amount_total) ? session.amount_total : 0,
      productSku,
      pluginVersion,
      fulfilledAt,
      JSON.stringify({ source: 'stripe-webhook', stripe_event_id: event.id, checkout_mode: STRIPE_MODE }),
    ],
  );

  return { status: 'processed', detail: 'order_upserted' };
}

async function markOrderFailedByPaymentIntent(client, event) {
  const intent = event?.data?.object;
  const paymentIntentId = intent?.id;

  if (!paymentIntentId) {
    return { status: 'ignored', detail: 'missing_payment_intent_id' };
  }

  const result = await client.query(
    `
      UPDATE orders
      SET status = 'failed',
          metadata = metadata || $2::jsonb,
          updated_at = NOW()
      WHERE stripe_payment_intent_id = $1
    `,
    [paymentIntentId, JSON.stringify({ source: 'stripe-webhook', stripe_event_id: event.id })],
  );

  if (result.rowCount === 0) {
    return { status: 'ignored', detail: 'order_not_found_for_payment_intent' };
  }

  return { status: 'processed', detail: 'order_marked_failed' };
}

async function markOrderRefunded(client, event) {
  const charge = event?.data?.object;
  const paymentIntentId = typeof charge?.payment_intent === 'string' ? charge.payment_intent : null;
  const chargeId = typeof charge?.id === 'string' ? charge.id : null;

  if (!paymentIntentId && !chargeId) {
    return { status: 'ignored', detail: 'missing_charge_identifiers' };
  }

  const result = await client.query(
    `
      UPDATE orders
      SET status = 'refunded',
          refunded_at = COALESCE(refunded_at, NOW()),
          stripe_charge_id = COALESCE(stripe_charge_id, $2),
          metadata = metadata || $3::jsonb,
          updated_at = NOW()
      WHERE ($1 IS NOT NULL AND stripe_payment_intent_id = $1)
         OR ($2 IS NOT NULL AND stripe_charge_id = $2)
    `,
    [paymentIntentId, chargeId, JSON.stringify({ source: 'stripe-webhook', stripe_event_id: event.id })],
  );

  if (result.rowCount === 0) {
    return { status: 'ignored', detail: 'order_not_found_for_refund' };
  }

  return { status: 'processed', detail: 'order_marked_refunded' };
}

async function processStripeEvent(client, event) {
  switch (event.type) {
    case 'checkout.session.completed':
      return upsertOrderFromCheckoutSession(client, event);
    case 'payment_intent.payment_failed':
      return markOrderFailedByPaymentIntent(client, event);
    case 'charge.refunded':
      return markOrderRefunded(client, event);
    default:
      return { status: 'ignored', detail: 'event_type_not_handled' };
  }
}

async function createStripeCheckoutSession(input) {
  const params = new URLSearchParams();
  params.set('mode', 'payment');
  params.set('line_items[0][price]', STRIPE_PRICE_ID);
  params.set('line_items[0][quantity]', String(input.quantity));
  params.set('success_url', input.successUrl);
  params.set('cancel_url', input.cancelUrl);
  params.set('client_reference_id', input.clientReferenceId);

  if (input.customerEmail) {
    params.set('customer_email', input.customerEmail);
  }

  Object.entries(input.metadata).forEach(([key, value]) => {
    params.set(`metadata[${key}]`, value);
  });

  const response = await fetch('https://api.stripe.com/v1/checkout/sessions', {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${STRIPE_API_KEY}`,
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: params.toString(),
  });

  const raw = await response.text();
  let parsed = null;
  try {
    parsed = JSON.parse(raw);
  } catch (_) {
    parsed = { raw };
  }

  if (!response.ok) {
    const message = parsed && parsed.error && parsed.error.message
      ? parsed.error.message
      : 'Stripe Checkout session creation failed';
    const error = new Error(message);
    error.statusCode = response.status;
    throw error;
  }

  return {
    id: parsed.id,
    url: parsed.url,
  };
}

async function handleCheckout(req, res) {
  let body = {};
  try {
    body = parseJson(await readRawBody(req));
  } catch (error) {
    sendJson(res, 400, { error: error.message === 'Request body too large' ? error.message : 'Invalid JSON body' });
    return;
  }

  const quantity = parseQuantity(body.quantity);
  const successUrl = body.successUrl || DEFAULT_SUCCESS_URL;
  const cancelUrl = body.cancelUrl || DEFAULT_CANCEL_URL;

  if (quantity === null) {
    sendJson(res, 400, { error: 'quantity must be an integer between 1 and 10' });
    return;
  }

  if (!isValidHttpUrl(successUrl) || !isValidHttpUrl(cancelUrl)) {
    sendJson(res, 400, { error: 'successUrl and cancelUrl must be valid http/https URLs' });
    return;
  }

  if (body.customerEmail && !String(body.customerEmail).includes('@')) {
    sendJson(res, 400, { error: 'customerEmail must be a valid email address' });
    return;
  }

  const metadata = buildMetadata(body);
  const clientReferenceId = body.clientReferenceId || randomUUID();

  if (STRIPE_MODE === 'mock') {
    const checkoutSessionId = `cs_mock_${randomUUID().replace(/-/g, '').slice(0, 24)}`;
    const checkoutUrl = `${MOCK_CHECKOUT_BASE_URL}/${checkoutSessionId}`;

    sendJson(res, 201, {
      provider: 'mock',
      checkoutSessionId,
      checkoutUrl,
      clientReferenceId,
      metadata,
    });
    return;
  }

  if (STRIPE_MODE !== 'test') {
    sendJson(res, 500, { error: `Unsupported STRIPE_MODE: ${STRIPE_MODE}` });
    return;
  }

  if (!STRIPE_API_KEY || !STRIPE_API_KEY.startsWith('sk_test_')) {
    sendJson(res, 500, { error: 'STRIPE_API_KEY must be set to a test secret key in test mode' });
    return;
  }

  if (!STRIPE_PRICE_ID) {
    sendJson(res, 500, { error: 'STRIPE_PRICE_ID must be set in test mode' });
    return;
  }

  try {
    const session = await createStripeCheckoutSession({
      quantity,
      successUrl,
      cancelUrl,
      customerEmail: body.customerEmail,
      clientReferenceId,
      metadata,
    });

    sendJson(res, 201, {
      provider: 'stripe',
      checkoutSessionId: session.id,
      checkoutUrl: session.url,
      clientReferenceId,
      metadata,
    });
  } catch (error) {
    sendJson(res, error.statusCode || 502, { error: error.message });
  }
}

async function handleStripeWebhook(req, res) {
  let rawBody = '';
  try {
    rawBody = await readRawBody(req);
  } catch (error) {
    sendJson(res, 400, { error: error.message });
    return;
  }

  const signature = verifyWebhookSignature(rawBody, req);
  if (!signature.ok) {
    sendJson(res, 400, { error: signature.error });
    return;
  }

  let event;
  try {
    event = parseJson(rawBody);
  } catch (_) {
    sendJson(res, 400, { error: 'Invalid webhook JSON payload' });
    return;
  }

  if (!event.id || !event.type || !event.data || !event.data.object) {
    sendJson(res, 400, { error: 'Webhook event is missing required fields' });
    return;
  }

  try {
    const result = await withTransaction(async client => {
      const inserted = await insertWebhookEventIfNew(client, event);
      if (!inserted) {
        return { duplicate: true, status: 'ignored', detail: 'already_processed' };
      }

      try {
        const processing = await processStripeEvent(client, event);
        await markWebhookEvent(client, event.id, processing.status, null);
        return { duplicate: false, ...processing };
      } catch (error) {
        await markWebhookEvent(client, event.id, 'failed', error.message);
        throw error;
      }
    });

    sendJson(res, 200, {
      received: true,
      idempotent: result.duplicate,
      processingStatus: result.status,
      detail: result.detail,
      eventId: event.id,
    });
  } catch (error) {
    sendJson(res, 500, {
      error: 'Webhook processing failed',
      detail: error.message,
      eventId: event.id,
    });
  }
}

const server = http.createServer(async (req, res) => {
  if (!req.url) {
    sendJson(res, 404, { error: 'Not Found' });
    return;
  }

  const url = new URL(req.url, `http://${req.headers.host || 'localhost'}`);

  if (req.method === 'GET' && url.pathname === '/health') {
    sendJson(res, 200, {
      ok: true,
      service: 'commerce-api',
      stripeMode: STRIPE_MODE,
      dbConfigured: Boolean(DATABASE_URL),
    });
    return;
  }

  if (req.method === 'POST' && url.pathname === '/checkout') {
    await handleCheckout(req, res);
    return;
  }

  if (req.method === 'POST' && url.pathname === '/webhooks/stripe') {
    await handleStripeWebhook(req, res);
    return;
  }

  sendJson(res, 404, { error: 'Not Found' });
});

server.listen(PORT, () => {
  console.log(`commerce-api listening on :${PORT} (stripe mode: ${STRIPE_MODE})`);
});
