#!/usr/bin/env node
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');

const PORT = Number(process.env.PORT || 3000);
const API_BASE_URL = process.env.API_BASE_URL || 'http://localhost:3001';
const PUBLIC_DIR = path.resolve(__dirname, '..', 'public');

const MIME_TYPES = {
  '.html': 'text/html; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.js': 'application/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.svg': 'image/svg+xml',
};

function sendJson(res, statusCode, body) {
  const payload = JSON.stringify(body);
  res.writeHead(statusCode, {
    'Content-Type': 'application/json; charset=utf-8',
    'Content-Length': Buffer.byteLength(payload),
  });
  res.end(payload);
}

function readRawBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    let total = 0;

    req.on('data', chunk => {
      chunks.push(chunk);
      total += chunk.length;
      if (total > 1024 * 64) {
        reject(new Error('Request body too large'));
        req.destroy();
      }
    });

    req.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
    req.on('error', reject);
  });
}

async function handleCheckoutProxy(req, res) {
  let rawBody = '{}';
  try {
    rawBody = await readRawBody(req);
    JSON.parse(rawBody);
  } catch (_) {
    sendJson(res, 400, { error: 'Invalid JSON body' });
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/checkout`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: rawBody,
    });

    const text = await response.text();
    let parsed;
    try {
      parsed = JSON.parse(text);
    } catch {
      parsed = { error: 'Upstream returned non-JSON response', raw: text };
    }

    sendJson(res, response.status, parsed);
  } catch (error) {
    sendJson(res, 502, { error: 'Unable to reach API', detail: error.message });
  }
}

function resolveFilePath(urlPathname) {
  const routeMap = {
    '/': 'index.html',
    '/checkout/success': 'success.html',
    '/checkout/cancel': 'cancel.html',
  };

  if (routeMap[urlPathname]) {
    return path.join(PUBLIC_DIR, routeMap[urlPathname]);
  }

  const safePath = path.normalize(urlPathname).replace(/^\.\.(\/|\\|$)/, '');
  return path.join(PUBLIC_DIR, safePath);
}

function serveStatic(req, res, urlPathname) {
  const filePath = resolveFilePath(urlPathname);

  if (!filePath.startsWith(PUBLIC_DIR)) {
    sendJson(res, 403, { error: 'Forbidden' });
    return;
  }

  fs.readFile(filePath, (err, data) => {
    if (err) {
      sendJson(res, 404, { error: 'Not Found' });
      return;
    }

    const ext = path.extname(filePath).toLowerCase();
    const contentType = MIME_TYPES[ext] || 'application/octet-stream';
    res.writeHead(200, {
      'Content-Type': contentType,
      'Content-Length': data.length,
      'Cache-Control': 'no-cache',
    });
    res.end(data);
  });
}

const server = http.createServer(async (req, res) => {
  if (!req.url) {
    sendJson(res, 404, { error: 'Not Found' });
    return;
  }

  const url = new URL(req.url, `http://${req.headers.host || 'localhost'}`);

  if (req.method === 'GET' && url.pathname === '/health') {
    sendJson(res, 200, { ok: true, service: 'commerce-web' });
    return;
  }

  if (req.method === 'POST' && url.pathname === '/api/checkout') {
    await handleCheckoutProxy(req, res);
    return;
  }

  if (req.method === 'GET') {
    serveStatic(req, res, url.pathname);
    return;
  }

  sendJson(res, 404, { error: 'Not Found' });
});

server.listen(PORT, () => {
  console.log(`commerce-web listening on :${PORT}`);
});
