-- Seed fixtures for local commerce testing
-- Safe to run repeatedly due to ON CONFLICT clauses where practical.

WITH upsert_customer AS (
    INSERT INTO customers (email, full_name, stripe_customer_id, metadata)
    VALUES (
        'test.buyer@example.com',
        'Test Buyer',
        'cus_test_buyer_001',
        '{"source":"seed"}'::jsonb
    )
    ON CONFLICT (email)
    DO UPDATE SET
        full_name = EXCLUDED.full_name,
        stripe_customer_id = EXCLUDED.stripe_customer_id,
        metadata = customers.metadata || EXCLUDED.metadata,
        updated_at = NOW()
    RETURNING id
),
customer_ref AS (
    SELECT id FROM upsert_customer
    UNION ALL
    SELECT id FROM customers WHERE email = 'test.buyer@example.com' LIMIT 1
),
upsert_order AS (
    INSERT INTO orders (
        customer_id,
        stripe_checkout_session_id,
        stripe_payment_intent_id,
        stripe_charge_id,
        status,
        currency,
        amount_cents,
        product_sku,
        plugin_version,
        fulfilled_at,
        metadata
    )
    SELECT
        id,
        'cs_test_0001',
        'pi_test_0001',
        'ch_test_0001',
        'paid',
        'USD',
        4900,
        'genx-delay-vst3',
        '0.1.0',
        NOW(),
        '{"source":"seed"}'::jsonb
    FROM customer_ref
    ON CONFLICT (stripe_checkout_session_id)
    DO UPDATE SET
        status = EXCLUDED.status,
        fulfilled_at = EXCLUDED.fulfilled_at,
        updated_at = NOW()
    RETURNING id, customer_id
),
order_ref AS (
    SELECT id, customer_id FROM upsert_order
    UNION ALL
    SELECT id, customer_id FROM orders WHERE stripe_checkout_session_id = 'cs_test_0001' LIMIT 1
)
INSERT INTO licenses (
    order_id,
    customer_id,
    license_key_hash,
    license_key_last4,
    status,
    metadata
)
SELECT
    id,
    customer_id,
    encode(digest('GENX-TEST-0000-0001', 'sha256'), 'hex'),
    '0001',
    'active',
    '{"source":"seed"}'::jsonb
FROM order_ref
ON CONFLICT (order_id)
DO UPDATE SET
    status = EXCLUDED.status,
    updated_at = NOW();

WITH order_ref AS (
    SELECT id, customer_id FROM orders WHERE stripe_checkout_session_id = 'cs_test_0001' LIMIT 1
)
INSERT INTO download_tokens (
    order_id,
    customer_id,
    token_hash,
    artifact_path,
    max_downloads,
    download_count,
    expires_at,
    metadata
)
SELECT
    id,
    customer_id,
    encode(digest('download-token-seed-0001', 'sha256'), 'hex'),
    'genx_delay/0.1.0/genx_delay.vst3.zip',
    5,
    0,
    NOW() + INTERVAL '7 days',
    '{"source":"seed"}'::jsonb
FROM order_ref
ON CONFLICT (token_hash)
DO UPDATE SET
    expires_at = EXCLUDED.expires_at,
    revoked = FALSE,
    updated_at = NOW();

INSERT INTO webhook_events (
    stripe_event_id,
    event_type,
    livemode,
    payload,
    processing_status,
    processed_at
)
VALUES (
    'evt_test_checkout_completed_0001',
    'checkout.session.completed',
    FALSE,
    '{"id":"evt_test_checkout_completed_0001","type":"checkout.session.completed"}'::jsonb,
    'processed',
    NOW()
)
ON CONFLICT (stripe_event_id)
DO NOTHING;
