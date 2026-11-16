import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

// The HTTP/runtime repositories bind TEXT ids, tenant scopes and timestamps and
// cast JSON payloads to JSONB. The retired baseline declared a UUID stub with
// only (name, slug, status), so every route answered 50301. These assertions
// keep the DDL and the SQL in step: a table or column the code queries must be
// created by the baseline, and the core table must stay text-keyed.
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

const baselinePath = path.join(root, "database/ddl/baseline/postgres/0001_shop_baseline.sql");
const migrationPath = path.join(
  root,
  "database/migrations/postgres/0002_shop_http_schema_repair.up.sql",
);
const baseline = readFileSync(baselinePath, "utf8");
const migration = readFileSync(migrationPath, "utf8");

function rustSources() {
  const files = [];
  for (const crate of ["sdkwork-shop-repository-sqlx", "sdkwork-routes-shop-app-api", "sdkwork-routes-shop-backend-api"]) {
    const src = path.join(root, "crates", crate, "src");
    for (const entry of readdirSync(src)) {
      if (entry.endsWith(".rs")) {
        files.push(readFileSync(path.join(src, entry), "utf8"));
      }
    }
  }
  return files.join("\n");
}

const sources = rustSources();

function referencedShopTables() {
  const tables = new Set();
  const pattern = /\b(?:INSERT\s+INTO|FROM|UPDATE|DELETE\s+FROM)\s+(commerce_shop[a-z_]*)\b/giu;
  for (const match of sources.matchAll(pattern)) {
    tables.add(match[1].toLowerCase());
  }
  return [...tables].sort();
}

test("every shop table the repositories query is created by the baseline", () => {
  const missing = referencedShopTables().filter(
    (table) => !new RegExp(`CREATE\\s+TABLE\\s+IF\\s+NOT\\s+EXISTS\\s+${table}\\b`, "iu").test(baseline),
  );
  assert.deepEqual(missing, [], `baseline is missing tables referenced by the shop code: ${missing.join(", ")}`);
});

test("the materialized database contract registers every baseline table", () => {
  const contract = readFileSync(path.join(root, "database/contract/schema.yaml"), "utf8");
  const missing = [
    ...baseline.matchAll(/CREATE\s+TABLE\s+IF\s+NOT\s+EXISTS\s+(commerce_[a-z_]+)\b/giu),
  ]
    .map((match) => match[1])
    .filter((table) => !contract.includes(`name: ${table}`));
  assert.deepEqual(missing, [], `database contract is missing tables: ${missing.join(", ")}`);
});

test("commerce_shop is text-keyed with the commerce columns the routes select", () => {
  const create = /CREATE\s+TABLE\s+IF\s+NOT\s+EXISTS\s+commerce_shop\s*\(([\s\S]*?)\n\);/u.exec(baseline);
  assert.ok(create, "commerce_shop must be declared by the baseline");
  const body = create[1];
  for (const column of [
    "id",
    "tenant_id",
    "organization_id",
    "shop_no",
    "shop_name",
    "storefront_status",
    "operation_status",
    "review_status",
    "data_scope",
    "default_currency_code",
  ]) {
    assert.match(body, new RegExp(`^\\s*${column}\\s+TEXT\\b`, "mu"), `commerce_shop.${column} must be TEXT`);
  }
  assert.match(body, /^\s*version\s+BIGINT\b/mu);
  assert.match(body, /^\s*deleted_at\s+TEXT\b/mu);
});

test("the repair migration upgrades the retired UUID stub", () => {
  assert.match(migration, /shop_make_text/u, "migration must convert legacy UUID keys to TEXT");
  assert.match(migration, /ux_commerce_shop_tenant_shop_no/u);
  assert.match(migration, /CREATE\s+TABLE\s+IF\s+NOT\s+EXISTS\s+commerce_shop_channel/iu);
});
