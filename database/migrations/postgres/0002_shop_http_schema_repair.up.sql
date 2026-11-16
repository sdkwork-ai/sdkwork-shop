-- sdkwork:migration
-- id: 0002_shop_http_schema_repair
-- engine: postgres
-- module: sdkwork-shop
-- purpose: Upgrade the legacy UUID-keyed shop stub to the text-keyed rich
--   schema the HTTP and runtime repositories query.
--
--   The retired baseline created `commerce_shop` with UUID keys and only
--   (name, slug, status); the app/backend routes select shop_no, shop_name,
--   storefront_status and the other commerce columns, and bind every id,
--   tenant, timestamp and amount as text. Against the stub every query failed
--   with a type or column error, which the route envelope mapped to
--   `50301 service unavailable` on /app/v3/api/shops.
--
--   This migration is additive and idempotent: it converts the legacy UUID
--   key/timestamp columns to TEXT, drops the single-table foreign keys that
--   the type change invalidates, adds every missing column with a default,
--   and creates the per-shop collections the HTTP repository reads. Fresh
--   installs already have the repaired baseline, so every step is a no-op.
-- reversible: false
-- rollback: forward-fix
-- transactional: true
-- lock: lightweight
-- lock_timeout: 5s
-- statement_timeout: 120s

BEGIN;

-- Generic helpers ------------------------------------------------------------

CREATE OR REPLACE FUNCTION pg_temp.shop_column_type(p_table TEXT, p_column TEXT)
RETURNS TEXT
LANGUAGE sql
AS $$
  SELECT data_type
  FROM information_schema.columns
  WHERE table_schema = current_schema()
    AND table_name = p_table
    AND column_name = p_column
$$;

-- Convert one column to TEXT when it exists and is not already text.
CREATE OR REPLACE FUNCTION pg_temp.shop_make_text(
  p_table TEXT,
  p_column TEXT,
  p_default TEXT DEFAULT NULL
)
RETURNS void
LANGUAGE plpgsql
AS $$
DECLARE
  current_type TEXT;
BEGIN
  current_type := pg_temp.shop_column_type(p_table, p_column);
  IF current_type IS NULL OR current_type = 'text' THEN
    RETURN;
  END IF;
  EXECUTE format(
    'ALTER TABLE %I ALTER COLUMN %I DROP DEFAULT',
    p_table,
    p_column
  );
  EXECUTE format(
    'ALTER TABLE %I ALTER COLUMN %I TYPE TEXT USING %I::text',
    p_table,
    p_column,
    p_column
  );
  IF p_default IS NOT NULL THEN
    EXECUTE format(
      'ALTER TABLE %I ALTER COLUMN %I SET DEFAULT %s',
      p_table,
      p_column,
      p_default
    );
  END IF;
END;
$$;

CREATE OR REPLACE FUNCTION pg_temp.shop_drop_fk(
  p_table TEXT,
  p_constraint TEXT
)
RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM pg_constraint c
    JOIN pg_class t ON t.oid = c.conrelid
    JOIN pg_namespace n ON n.oid = t.relnamespace
    WHERE n.nspname = current_schema()
      AND t.relname = p_table
      AND c.conname = p_constraint
  ) THEN
    EXECUTE format(
      'ALTER TABLE %I DROP CONSTRAINT %I',
      p_table,
      p_constraint
    );
  END IF;
END;
$$;

-- commerce_shop --------------------------------------------------------------

DO $migration$
BEGIN
IF to_regclass('commerce_shop') IS NOT NULL THEN
  PERFORM pg_temp.shop_drop_fk(
    'commerce_shop_application',
    'commerce_shop_application_shop_id_fkey'
  );
  PERFORM pg_temp.shop_drop_fk(
    'commerce_shop_status_event',
    'commerce_shop_status_event_shop_id_fkey'
  );

  PERFORM pg_temp.shop_make_text('commerce_shop', 'id');
  PERFORM pg_temp.shop_make_text('commerce_shop', 'tenant_id');
  PERFORM pg_temp.shop_make_text('commerce_shop', 'organization_id', $$'0'$$);
  PERFORM pg_temp.shop_make_text('commerce_shop', 'created_at', $$''$$);
  PERFORM pg_temp.shop_make_text('commerce_shop', 'updated_at', $$''$$);

  -- The stub declared name/slug/status NOT NULL. The commerce model names the
  -- same concepts shop_name/shop_no/storefront_status; the legacy columns stay
  -- nullable so rich inserts that never mention them keep working.
  IF pg_temp.shop_column_type('commerce_shop', 'name') IS NOT NULL THEN
    ALTER TABLE commerce_shop ALTER COLUMN name DROP NOT NULL;
  END IF;
  IF pg_temp.shop_column_type('commerce_shop', 'slug') IS NOT NULL THEN
    ALTER TABLE commerce_shop ALTER COLUMN slug DROP NOT NULL;
  END IF;
  IF pg_temp.shop_column_type('commerce_shop', 'status') IS NOT NULL THEN
    ALTER TABLE commerce_shop ALTER COLUMN status DROP NOT NULL;
  END IF;

  ALTER TABLE commerce_shop
    ADD COLUMN IF NOT EXISTS shop_no TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS shop_name TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS shop_type TEXT NOT NULL DEFAULT 'physical',
    ADD COLUMN IF NOT EXISTS business_model TEXT NOT NULL DEFAULT 'self_operated',
    ADD COLUMN IF NOT EXISTS storefront_status TEXT NOT NULL DEFAULT 'draft',
    ADD COLUMN IF NOT EXISTS operation_status TEXT NOT NULL DEFAULT 'normal',
    ADD COLUMN IF NOT EXISTS review_status TEXT NOT NULL DEFAULT 'pending',
    ADD COLUMN IF NOT EXISTS data_scope TEXT NOT NULL DEFAULT 'organization',
    ADD COLUMN IF NOT EXISTS logo_media_resource_id TEXT,
    ADD COLUMN IF NOT EXISTS cover_media_resource_id TEXT,
    ADD COLUMN IF NOT EXISTS default_currency_code TEXT NOT NULL DEFAULT 'CNY',
    ADD COLUMN IF NOT EXISTS default_locale TEXT,
    ADD COLUMN IF NOT EXISTS timezone TEXT,
    ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS deleted_at TEXT;

  IF NOT EXISTS (
    SELECT 1 FROM pg_indexes
    WHERE schemaname = current_schema()
      AND indexname = 'ux_commerce_shop_tenant_shop_no'
  ) THEN
    CREATE UNIQUE INDEX ux_commerce_shop_tenant_shop_no
      ON commerce_shop (tenant_id, shop_no)
      WHERE deleted_at IS NULL AND shop_no <> '';
  END IF;
  CREATE INDEX IF NOT EXISTS idx_commerce_shop_tenant_created
    ON commerce_shop (tenant_id, created_at DESC, id DESC);
  CREATE INDEX IF NOT EXISTS idx_commerce_shop_tenant_org
    ON commerce_shop (tenant_id, organization_id);
END IF;

-- commerce_shop_application --------------------------------------------------

IF to_regclass('commerce_shop_application') IS NOT NULL THEN
  PERFORM pg_temp.shop_drop_fk(
    'commerce_shop_application',
    'commerce_shop_application_shop_id_fkey'
  );

  PERFORM pg_temp.shop_make_text('commerce_shop_application', 'id');
  PERFORM pg_temp.shop_make_text('commerce_shop_application', 'shop_id');
  PERFORM pg_temp.shop_make_text('commerce_shop_application', 'applicant_user_id');
  PERFORM pg_temp.shop_make_text('commerce_shop_application', 'submitted_at');

  ALTER TABLE commerce_shop_application
    ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '0',
    ADD COLUMN IF NOT EXISTS application_no TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS application_type TEXT NOT NULL DEFAULT 'initial',
    ADD COLUMN IF NOT EXISTS review_status TEXT NOT NULL DEFAULT 'pending',
    ADD COLUMN IF NOT EXISTS legal_entity_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    ADD COLUMN IF NOT EXISTS contact_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    ADD COLUMN IF NOT EXISTS qualification_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    ADD COLUMN IF NOT EXISTS submitted_by TEXT,
    ADD COLUMN IF NOT EXISTS reviewed_by TEXT,
    ADD COLUMN IF NOT EXISTS reviewed_at TEXT,
    ADD COLUMN IF NOT EXISTS review_comment TEXT,
    ADD COLUMN IF NOT EXISTS idempotency_key TEXT,
    ADD COLUMN IF NOT EXISTS created_at TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS updated_at TEXT NOT NULL DEFAULT '';

  IF pg_temp.shop_column_type('commerce_shop_application', 'applicant_user_id') IS NOT NULL THEN
    ALTER TABLE commerce_shop_application ALTER COLUMN applicant_user_id DROP NOT NULL;
  END IF;
  IF pg_temp.shop_column_type('commerce_shop_application', 'submitted_at') IS NOT NULL THEN
    ALTER TABLE commerce_shop_application ALTER COLUMN submitted_at DROP NOT NULL;
  END IF;

  CREATE INDEX IF NOT EXISTS idx_commerce_shop_application_scope
    ON commerce_shop_application (tenant_id, shop_id, created_at DESC);
END IF;

-- commerce_shop_status_event -------------------------------------------------

IF to_regclass('commerce_shop_status_event') IS NOT NULL THEN
  PERFORM pg_temp.shop_drop_fk(
    'commerce_shop_status_event',
    'commerce_shop_status_event_shop_id_fkey'
  );

  PERFORM pg_temp.shop_make_text('commerce_shop_status_event', 'id');
  PERFORM pg_temp.shop_make_text('commerce_shop_status_event', 'shop_id');
  PERFORM pg_temp.shop_make_text('commerce_shop_status_event', 'actor_user_id');
  PERFORM pg_temp.shop_make_text('commerce_shop_status_event', 'recorded_at');

  ALTER TABLE commerce_shop_status_event
    ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '0',
    ADD COLUMN IF NOT EXISTS event_no TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS event_type TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS actor_type TEXT,
    ADD COLUMN IF NOT EXISTS actor_id TEXT,
    ADD COLUMN IF NOT EXISTS idempotency_key TEXT,
    ADD COLUMN IF NOT EXISTS created_at TEXT NOT NULL DEFAULT '';

  IF pg_temp.shop_column_type('commerce_shop_status_event', 'recorded_at') IS NOT NULL THEN
    ALTER TABLE commerce_shop_status_event ALTER COLUMN recorded_at DROP NOT NULL;
  END IF;

  CREATE INDEX IF NOT EXISTS idx_commerce_shop_status_event_scope
    ON commerce_shop_status_event (tenant_id, shop_id, created_at DESC);
END IF;

-- Per-shop collections -------------------------------------------------------
-- Same definitions as the repaired baseline; idempotent on fresh installs.

CREATE TABLE IF NOT EXISTS commerce_shop_verification (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  verification_type TEXT NOT NULL DEFAULT 'business_license',
  verification_status TEXT NOT NULL DEFAULT 'pending',
  legal_entity_name TEXT,
  credential_no_hash TEXT,
  credential_media_resource_id TEXT,
  verification_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  expires_at TEXT,
  reviewed_by TEXT,
  reviewed_at TEXT,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_qualification (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  qualification_type TEXT NOT NULL DEFAULT '',
  qualification_status TEXT NOT NULL DEFAULT 'active',
  subject_type TEXT,
  subject_id TEXT,
  qualification_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_category_binding (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  shop_category_code TEXT NOT NULL DEFAULT '',
  platform_category_code TEXT,
  platform_category_name TEXT,
  category_path TEXT,
  category_level BIGINT NOT NULL DEFAULT 0,
  category_status TEXT NOT NULL DEFAULT 'active',
  qualification_required BOOLEAN NOT NULL DEFAULT FALSE,
  qualification_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  review_status TEXT NOT NULL DEFAULT 'pending',
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_brand_authorization (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  brand_code TEXT NOT NULL DEFAULT '',
  brand_name TEXT,
  authorization_type TEXT NOT NULL DEFAULT 'trademark',
  authorization_status TEXT NOT NULL DEFAULT 'active',
  authorization_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_customer_service (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  service_channel TEXT NOT NULL DEFAULT 'im',
  service_status TEXT NOT NULL DEFAULT 'active',
  contact_ref TEXT,
  contact_label TEXT,
  service_window_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  service_config_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  sort_order BIGINT NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_return_address (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  address_usage TEXT NOT NULL DEFAULT 'return',
  address_key TEXT,
  receiver_name TEXT,
  phone_hash TEXT,
  country_code TEXT,
  region_code TEXT,
  city_code TEXT,
  district_code TEXT,
  address_line1 TEXT,
  postal_code TEXT,
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  address_status TEXT NOT NULL DEFAULT 'active',
  address_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_shipping_template (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  template_code TEXT NOT NULL DEFAULT '',
  template_name TEXT,
  template_status TEXT NOT NULL DEFAULT 'active',
  pricing_mode TEXT NOT NULL DEFAULT 'fixed',
  delivery_method TEXT,
  base_quantity BIGINT NOT NULL DEFAULT 1,
  base_fee_amount TEXT NOT NULL DEFAULT '0',
  currency_code TEXT NOT NULL DEFAULT 'CNY',
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  region_rule_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  free_shipping_rule_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_channel (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  channel_code TEXT NOT NULL DEFAULT '',
  storefront_status TEXT NOT NULL DEFAULT 'draft',
  domain_name TEXT,
  path_prefix TEXT,
  theme_code TEXT,
  channel_config_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  sort_order BIGINT NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_service_area (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  area_type TEXT NOT NULL DEFAULT 'region',
  country_code TEXT,
  region_code TEXT,
  city_code TEXT,
  area_key TEXT,
  postal_code_pattern TEXT,
  delivery_radius_meters BIGINT,
  service_status TEXT NOT NULL DEFAULT 'active',
  service_config_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  sort_order BIGINT NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_business_hour (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  schedule_type TEXT NOT NULL DEFAULT 'weekly',
  timezone TEXT NOT NULL DEFAULT 'Asia/Shanghai',
  weekly_schedule_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  holiday_schedule_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  effective_from TEXT,
  effective_to TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  version BIGINT NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_fulfillment_profile (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  fulfillment_mode TEXT NOT NULL DEFAULT 'self',
  shipping_origin_region_code TEXT,
  service_level_code TEXT,
  after_sales_policy_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  service_config_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_settlement_profile (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  settlement_status TEXT NOT NULL DEFAULT 'pending',
  settlement_cycle TEXT,
  settlement_currency_code TEXT NOT NULL DEFAULT 'CNY',
  account_ref TEXT,
  risk_hold_days BIGINT NOT NULL DEFAULT 0,
  settlement_config_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  reviewed_by TEXT,
  reviewed_at TEXT,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_deposit_account (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  deposit_status TEXT NOT NULL DEFAULT 'pending',
  currency_code TEXT NOT NULL DEFAULT 'CNY',
  required_amount TEXT NOT NULL DEFAULT '0',
  paid_amount TEXT NOT NULL DEFAULT '0',
  frozen_amount TEXT NOT NULL DEFAULT '0',
  account_ref TEXT,
  due_at TEXT,
  reviewed_by TEXT,
  reviewed_at TEXT,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_policy (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  policy_type TEXT NOT NULL DEFAULT 'service',
  policy_status TEXT NOT NULL DEFAULT 'draft',
  policy_version BIGINT NOT NULL DEFAULT 1,
  policy_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  published_at TEXT,
  reviewed_by TEXT,
  reviewed_at TEXT,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_risk_signal (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  signal_no TEXT,
  signal_type TEXT NOT NULL DEFAULT '',
  risk_level TEXT NOT NULL DEFAULT 'low',
  signal_status TEXT NOT NULL DEFAULT 'open',
  source_type TEXT,
  source_id TEXT,
  risk_score BIGINT NOT NULL DEFAULT 0,
  payload_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  detected_at TEXT,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commerce_shop_readiness (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  readiness_scope TEXT NOT NULL DEFAULT 'shop',
  readiness_status TEXT NOT NULL DEFAULT 'pending',
  blocking_count BIGINT NOT NULL DEFAULT 0,
  warning_count BIGINT NOT NULL DEFAULT 0,
  checklist_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  evaluated_at TEXT,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT '',
  version BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS commerce_shop_metric_snapshot (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  snapshot_date TEXT NOT NULL DEFAULT '',
  metric_code TEXT,
  metric_value TEXT,
  dimension_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_commerce_shop_verification_scope ON commerce_shop_verification (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_qualification_scope ON commerce_shop_qualification (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_category_binding_scope ON commerce_shop_category_binding (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_brand_authorization_scope ON commerce_shop_brand_authorization (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_customer_service_scope ON commerce_shop_customer_service (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_return_address_scope ON commerce_shop_return_address (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_shipping_template_scope ON commerce_shop_shipping_template (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_channel_scope ON commerce_shop_channel (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_service_area_scope ON commerce_shop_service_area (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_business_hour_scope ON commerce_shop_business_hour (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_fulfillment_profile_scope ON commerce_shop_fulfillment_profile (tenant_id, shop_id);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_settlement_profile_scope ON commerce_shop_settlement_profile (tenant_id, shop_id);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_deposit_account_scope ON commerce_shop_deposit_account (tenant_id, shop_id);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_policy_scope ON commerce_shop_policy (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_risk_signal_scope ON commerce_shop_risk_signal (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_readiness_scope ON commerce_shop_readiness (tenant_id, shop_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_metric_snapshot_scope ON commerce_shop_metric_snapshot (tenant_id, shop_id, snapshot_date DESC);

END
$migration$;

DROP FUNCTION pg_temp.shop_make_text(TEXT, TEXT, TEXT);
DROP FUNCTION pg_temp.shop_drop_fk(TEXT, TEXT);
DROP FUNCTION pg_temp.shop_column_type(TEXT, TEXT);

COMMIT;
