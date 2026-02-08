#!/usr/bin/env node
const http = require('node:http');
const { randomUUID } = require('node:crypto');

const PORT = Number(process.env.PORT || 3001);
const STRIPE_MODE = (process.env.STRIPE_MODE || 'mock').toLowerCase();
const STRIPE_API_KEY = process.env.STRIPE_API_KEY || '';
const STRIPE_PRICE_ID = process.env.STRIPE_PRICE_ID || '';
const DEFAULT_PRODUCT_SKU = process.env.STRIPE_PRODUCT_SKU || 'genx-delay-vst3';
const DEFAULT_PLUGIN_VERSION = process.env.PLUGIN_VERSION || '0.1.0';
const DEFAULT_SUCCESS_URL = process.env.CHECKOUT_SUCCESS_URL || 'http://localhost:3000/checkout/success';
const DEFAULT_CANCEL_URL = process.env.CHECKOUT_CANCEL_URL || 'http://localhost:3000/checkout/cancel';
const MOCK_CHECKOUT_BASE_URL = process.env.MOCK_CHECKOUT_BASE_URL || 'http://localhost:3001/mock-checkout';

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

function readJsonBody(req) {
  return new Promise((resolve, reject) => {
    let raw = '';

    req.on('data', chunk => {
      raw += chunk;
      if (raw.length > 1024 * 32) {
        reject(new Error('Request body too large'));
        req.destroy();
      }
    });

    req.on('end', () => {
      if (!raw) {
        resolve({});
        return;
      }

      try {
        resolve(JSON.parse(raw));
      } catch (_) {
        reject(new Error('Invalid JSON body'));
      }
    });

    req.on('error', reject);
  });
}

function buildMetadata(payload) {
  return {
    product_sku: payload.productSku || DEFAULT_PRODUCT_SKU,
    plugin_version: payload.pluginVersion || DEFAULT_PLUGIN_VERSION,
    checkout_mode: STRIPE_MODE,
    source: 'genx-commerce-api',
  };
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
  let body;
  try {
    body = await readJsonBody(req);
  } catch (error) {
    sendJson(res, 400, { error: error.message });
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
    });
    return;
  }

  if (req.method === 'POST' && url.pathname === '/checkout') {
    await handleCheckout(req, res);
    return;
  }

  sendJson(res, 404, { error: 'Not Found' });
});

server.listen(PORT, () => {
  console.log(`commerce-api listening on :${PORT} (stripe mode: ${STRIPE_MODE})`);
});
