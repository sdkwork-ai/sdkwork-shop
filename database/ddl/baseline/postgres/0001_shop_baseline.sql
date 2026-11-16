-- ============================================================================
-- SDKWork shop baseline (commerce_shop* core tables)
-- ============================================================================
-- Ownership
--   sdkwork-shop is the physical owner of the `commerce_shop*` table family. The
--   `commerce_product_*` family belongs to sdkwork-merchandise and the
--   `commerce_order*` / `commerce_checkout*` families belong to sdkwork-order;
--   they are read or written here only through their owning service.
--
-- Storage model
--   Ids, tenant/organization scopes and timestamps are TEXT: the HTTP and
--   runtime repositories bind every one of those values as a Rust `String`
--   (`CAST($n AS TEXT)`, `chrono_like_now()`) and read them back as strings,
--   which is also how the sibling commerce modules key their tables
--   (DATABASE_SPEC DB090 tenant/organization subjects, API_SPEC §13.6 int64
--   wire strings stay strings through this surface). Aggregate JSON payloads
--   are JSONB because the writers cast them (`$n::jsonb`). Counters that the
--   code reads as `i64` (`version`, `sort_order`, `*_level`, `*_count`) are
--   BIGINT; booleans that the code binds as `bool` are BOOLEAN.
--
--   The previous baseline declared a UUID-keyed stub (`id`, `tenant_id`,
--   `organization_id`, `name`, `slug`, `status`) that no HTTP route could
--   query: `tenant_id = CAST($1 AS TEXT)` against a UUID column is an
--   operator error, and the rich columns the routes select did not exist. Every
--   shop route answered `50301`. `0002_shop_http_schema_repair` upgrades an
--   installation that still carries that stub.
-- ============================================================================

CREATE TABLE IF NOT EXISTS commerce_shop (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_no TEXT NOT NULL DEFAULT '',
  shop_name TEXT NOT NULL DEFAULT '',
  shop_type TEXT NOT NULL DEFAULT 'physical',
  business_model TEXT NOT NULL DEFAULT 'self_operated',
  storefront_status TEXT NOT NULL DEFAULT 'draft',
  operation_status TEXT NOT NULL DEFAULT 'normal',
  review_status TEXT NOT NULL DEFAULT 'pending',
  data_scope TEXT NOT NULL DEFAULT 'organization',
  logo_media_resource_id TEXT,
  cover_media_resource_id TEXT,
  default_currency_code TEXT NOT NULL DEFAULT 'CNY',
  default_locale TEXT,
  timezone TEXT,
  version BIGINT NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT '',
  deleted_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_commerce_shop_tenant_shop_no
  ON commerce_shop (tenant_id, shop_no)
  WHERE deleted_at IS NULL AND shop_no <> '';
CREATE INDEX IF NOT EXISTS idx_commerce_shop_tenant_created
  ON commerce_shop (tenant_id, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_commerce_shop_tenant_org
  ON commerce_shop (tenant_id, organization_id);

CREATE TABLE IF NOT EXISTS commerce_shop_application (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  application_no TEXT NOT NULL DEFAULT '',
  application_type TEXT NOT NULL DEFAULT 'initial',
  review_status TEXT NOT NULL DEFAULT 'pending',
  legal_entity_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  contact_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  qualification_snapshot_json JSONB NOT NULL DEFAULT '{}'::jsonb,
  submitted_by TEXT,
  submitted_at TEXT,
  reviewed_by TEXT,
  reviewed_at TEXT,
  review_comment TEXT,
  idempotency_key TEXT,
  created_at TEXT NOT NULL DEFAULT '',
  updated_at TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_commerce_shop_application_scope
  ON commerce_shop_application (tenant_id, shop_id, created_at DESC);

CREATE TABLE IF NOT EXISTS commerce_shop_status_event (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  organization_id TEXT NOT NULL DEFAULT '0',
  shop_id TEXT NOT NULL,
  event_no TEXT NOT NULL DEFAULT '',
  event_type TEXT NOT NULL DEFAULT '',
  from_status TEXT,
  to_status TEXT NOT NULL DEFAULT '',
  actor_type TEXT,
  actor_id TEXT,
  idempotency_key TEXT,
  created_at TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_commerce_shop_status_event_scope
  ON commerce_shop_status_event (tenant_id, shop_id, created_at DESC);

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

CREATE INDEX IF NOT EXISTS idx_commerce_shop_metric_snapshot_scope
  ON commerce_shop_metric_snapshot (tenant_id, shop_id, snapshot_date DESC);

-- Every per-shop collection is read by (tenant_id, shop_id) and ordered by
-- created_at, so each one carries the same covering index.
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
