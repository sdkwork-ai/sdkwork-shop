# Runbook — sdkwork-shop troubleshooting (EN)

Symptom → `bin/doctor.sh --environment <env>` check → fix.

## 1. Instance unhealthy

```bash
bin/doctor.sh --environment staging
bin/docker-deploy.sh status --environment staging
```

- `health FAIL`: reproduce with `curl -fsS http://127.0.0.1:<mgmt-port>/healthz`; inspect recent errors with `bin/docker-deploy.sh logs --environment staging --tail 200`.

## 2. Port not listening

```bash
bin/doctor.sh --environment staging          # the ports check reports expected ports
```

- Port taken: change `*_HOST_PORT` in the bundle env file and replay `bin/docker-deploy.sh install`.

## 3. Config drift / placeholder secrets

```bash
bin/config.sh diff --environment staging
bin/config.sh validate --environment staging
bin/config.sh set --environment staging --key <KEY> --value '<real-value>'
```

## 4. Image drift (running version differs from the ledger)

```bash
bin/docker-deploy.sh status --environment staging
```

- A mismatch between the reported image and the release ledger is a drift
  alert: `bin/docker-deploy.sh rollback --environment staging --to <ledger-version>`.

## 5. `50301` on `/app/v3/api/shops`

`GET /app/v3/api/shops` answers `503` / `50301` ("A required dependency is
temporarily unavailable") whenever the shop database is still on the retired
UUID-keyed stub. `shop_system_response` maps every non-validation storage error
to `DependencyUnavailable`, so a schema mismatch surfaces as an unavailable
dependency instead of a query error.

- Inspect the database schema: `commerce_shop.tenant_id` must be `text`, and the
  `shop_no` / `shop_name` / `storefront_status` columns must exist. The stub had
  `uuid` keys and only `(name, slug, status)`.
- Repair it through the normal deploy path: `bin/docker-deploy.sh install
  --environment <env>` runs the packaged database-module pass, which applies
  `0002_shop_http_schema_repair`. The migration converts the legacy UUID keys to
  `text`, adds the missing commerce columns and creates the per-shop collections.
- Fresh installs take the repaired baseline directly; no manual DDL is needed.

<!-- generated: scaffold-module-runbooks.mjs -->
