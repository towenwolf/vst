# Commerce Database Foundation

Task 4 artifacts for the commerce API.

## Structure

- `migrations/0001_init.sql` — initial schema bootstrap
- `seeds/seed_test_data.sql` — local test fixtures

## Entities

- `customers`
- `orders`
- `licenses`
- `download_tokens`
- `webhook_events`

## Notes

- PostgreSQL extensions used: `pgcrypto`, `citext`
- `updated_at` is maintained by a shared trigger function
- Keys/tokens are represented as hashes in schema fields intended for secure storage
