use sdkwork_contract_service::CommerceServiceError;
use sqlx::{postgres::PgRow, Column, PgPool, Row};

#[derive(Clone)]
pub enum ShopWriteDb {
    Postgres(PgPool),
}

impl ShopWriteDb {
    pub fn postgres(pool: PgPool) -> Self {
        Self::Postgres(pool)
    }
}

pub async fn upsert_shop_table_row(
    db: &ShopWriteDb,
    table: &str,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    // Platform rows persist the sentinel organization scope (`"0"`) so that
    // personal-login (no-org) sessions never write NULL into the NOT NULL
    // `organization_id` columns (DATABASE_SPEC DB090).
    let organization_id = Some(
        organization_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("0"),
    );
    if table == "commerce_shop_category_binding" {
        return upsert_category_binding_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_brand_authorization" {
        return upsert_brand_authorization_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_qualification" {
        return upsert_qualification_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_customer_service" {
        return upsert_customer_service_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_return_address" {
        return upsert_return_address_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_shipping_template" {
        return upsert_shipping_template_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_channel" {
        return upsert_channel_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_service_area" {
        return upsert_service_area_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_policy" {
        return upsert_policy_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_fulfillment_profile" {
        return upsert_fulfillment_profile_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_settlement_profile" {
        return upsert_settlement_profile_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_business_hour" {
        return upsert_business_hour_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_deposit_account" {
        return upsert_deposit_account_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_verification" {
        return upsert_verification_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_risk_signal" {
        return upsert_risk_signal_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }
    if table == "commerce_shop_application" {
        return upsert_application_db(
            db,
            tenant_id,
            organization_id,
            shop_id,
            explicit_id,
            payload,
        )
        .await;
    }

    Err(CommerceServiceError::validation(format!(
        "unsupported shop subresource table: {table}"
    )))
}
async fn upsert_category_binding_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let shop_category_code = payload
        .get("shopCategoryCode")
        .or_else(|| payload.get("categoryCode"))
        .or_else(|| payload.get("categoryId"))
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &[
            "shop-category-binding",
            tenant_id,
            shop_id,
            shop_category_code,
        ],
    );
    let category_status = payload
        .get("categoryStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let review_status = payload
        .get("reviewStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let qualification_required = payload
        .get("qualificationRequired")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let category_level = payload
        .get("categoryLevel")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let platform_category_code = payload.get("platformCategoryCode").and_then(|v| v.as_str());
    let platform_category_name = payload.get("platformCategoryName").and_then(|v| v.as_str());
    let category_path = payload.get("categoryPath").and_then(|v| v.as_str());
    let qualification_snapshot_json = payload
        .get("qualificationSnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_category_binding (id, tenant_id, organization_id, shop_id, shop_category_code, platform_category_code, platform_category_name, category_path, category_level, category_status, qualification_required, qualification_snapshot_json, review_status, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, $11, $12::jsonb, $13, $14, $14) ON CONFLICT(id) DO UPDATE SET platform_category_code = EXCLUDED.platform_category_code, platform_category_name = EXCLUDED.platform_category_name, category_path = EXCLUDED.category_path, category_level = EXCLUDED.category_level, category_status = EXCLUDED.category_status, qualification_required = EXCLUDED.qualification_required, qualification_snapshot_json = EXCLUDED.qualification_snapshot_json, review_status = EXCLUDED.review_status, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(shop_category_code)
                .bind(platform_category_code)
                .bind(platform_category_name)
                .bind(category_path)
                .bind(category_level)
                .bind(category_status)
                .bind(qualification_required)
                .bind(&qualification_snapshot_json)
                .bind(review_status)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_category_binding", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("category binding missing after upsert"))
}

async fn upsert_brand_authorization_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let brand_code = payload
        .get("brandCode")
        .and_then(|v| v.as_str())
        .unwrap_or("brand");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &["shop-brand-auth", tenant_id, shop_id, brand_code],
    );
    let brand_name = payload
        .get("brandName")
        .and_then(|v| v.as_str())
        .unwrap_or("brand");
    let authorization_type = payload
        .get("authorizationType")
        .and_then(|v| v.as_str())
        .unwrap_or("trademark");
    let authorization_status = payload
        .get("authorizationStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let snapshot_json = payload
        .get("authorizationSnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_brand_authorization (id, tenant_id, organization_id, shop_id, brand_code, brand_name, authorization_type, authorization_status, authorization_snapshot_json, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9::jsonb, $10, $10) ON CONFLICT(id) DO UPDATE SET brand_name = EXCLUDED.brand_name, authorization_type = EXCLUDED.authorization_type, authorization_status = EXCLUDED.authorization_status, authorization_snapshot_json = EXCLUDED.authorization_snapshot_json, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(brand_code)
                .bind(brand_name)
                .bind(authorization_type)
                .bind(authorization_status)
                .bind(&snapshot_json)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_brand_authorization", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("brand authorization missing after upsert"))
}

async fn upsert_qualification_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let qualification_type = payload
        .get("qualificationType")
        .and_then(|v| v.as_str())
        .unwrap_or("business_license");
    let subject_type = payload
        .get("subjectType")
        .and_then(|v| v.as_str())
        .unwrap_or("merchant");
    let subject_id = payload
        .get("subjectId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &[
            "shop-qualification",
            tenant_id,
            shop_id,
            qualification_type,
            subject_type,
            subject_id,
        ],
    );
    let qualification_status = payload
        .get("qualificationStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let snapshot_json = payload
        .get("qualificationSnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_qualification (id, tenant_id, organization_id, shop_id, qualification_type, qualification_status, subject_type, subject_id, qualification_snapshot_json, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9::jsonb, $10, $10) ON CONFLICT(id) DO UPDATE SET qualification_status = EXCLUDED.qualification_status, qualification_snapshot_json = EXCLUDED.qualification_snapshot_json, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(qualification_type)
                .bind(qualification_status)
                .bind(subject_type)
                .bind(subject_id)
                .bind(&snapshot_json)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_qualification", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("qualification missing after upsert"))
}

async fn upsert_customer_service_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let service_channel = payload
        .get("serviceChannel")
        .and_then(|v| v.as_str())
        .unwrap_or("online_chat");
    let contact_ref = payload
        .get("contactRef")
        .and_then(|v| v.as_str())
        .unwrap_or("default-contact");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &[
            "shop-customer-service",
            tenant_id,
            shop_id,
            service_channel,
            contact_ref,
        ],
    );
    let service_status = payload
        .get("serviceStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let contact_label = payload.get("contactLabel").and_then(|v| v.as_str());
    let service_window_json = payload
        .get("serviceWindow")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let service_config_json = payload
        .get("serviceConfig")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let is_default = payload
        .get("isDefault")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let sort_order = payload
        .get("sortOrder")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_customer_service (id, tenant_id, organization_id, shop_id, service_channel, service_status, contact_ref, contact_label, service_window_json, service_config_json, is_default, sort_order, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9::jsonb, $10::jsonb, $11, $12, $13, $13) ON CONFLICT(id) DO UPDATE SET service_status = EXCLUDED.service_status, contact_ref = EXCLUDED.contact_ref, contact_label = EXCLUDED.contact_label, service_window_json = EXCLUDED.service_window_json, service_config_json = EXCLUDED.service_config_json, is_default = EXCLUDED.is_default, sort_order = EXCLUDED.sort_order, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(service_channel)
                .bind(service_status)
                .bind(contact_ref)
                .bind(contact_label)
                .bind(&service_window_json)
                .bind(&service_config_json)
                .bind(is_default)
                .bind(sort_order)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_customer_service", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("customer service missing after upsert"))
}

async fn upsert_return_address_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let address_usage = payload
        .get("addressUsage")
        .and_then(|v| v.as_str())
        .unwrap_or("return");
    let address_key = payload
        .get("addressKey")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &[
            "shop-return-address",
            tenant_id,
            shop_id,
            address_usage,
            address_key,
        ],
    );
    let receiver_name = payload
        .get("receiverName")
        .and_then(|v| v.as_str())
        .unwrap_or("receiver");
    let country_code = payload
        .get("countryCode")
        .and_then(|v| v.as_str())
        .unwrap_or("CN");
    let address_line1 = payload
        .get("addressLine1")
        .and_then(|v| v.as_str())
        .unwrap_or("default address");
    let address_status = payload
        .get("addressStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let phone_hash = payload.get("phoneHash").and_then(|v| v.as_str());
    let region_code = payload.get("regionCode").and_then(|v| v.as_str());
    let city_code = payload.get("cityCode").and_then(|v| v.as_str());
    let district_code = payload.get("districtCode").and_then(|v| v.as_str());
    let postal_code = payload.get("postalCode").and_then(|v| v.as_str());
    let is_default = payload
        .get("isDefault")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let address_snapshot_json = payload
        .get("addressSnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_return_address (id, tenant_id, organization_id, shop_id, address_usage, address_key, receiver_name, phone_hash, country_code, region_code, city_code, district_code, address_line1, postal_code, is_default, address_status, address_snapshot_json, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17::jsonb, $18, $18) ON CONFLICT(id) DO UPDATE SET receiver_name = EXCLUDED.receiver_name, phone_hash = EXCLUDED.phone_hash, country_code = EXCLUDED.country_code, region_code = EXCLUDED.region_code, city_code = EXCLUDED.city_code, district_code = EXCLUDED.district_code, address_line1 = EXCLUDED.address_line1, postal_code = EXCLUDED.postal_code, is_default = EXCLUDED.is_default, address_status = EXCLUDED.address_status, address_snapshot_json = EXCLUDED.address_snapshot_json, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(address_usage)
                .bind(address_key)
                .bind(receiver_name)
                .bind(phone_hash)
                .bind(country_code)
                .bind(region_code)
                .bind(city_code)
                .bind(district_code)
                .bind(address_line1)
                .bind(postal_code)
                .bind(is_default)
                .bind(address_status)
                .bind(&address_snapshot_json)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_return_address", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("return address missing after upsert"))
}

async fn upsert_shipping_template_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let template_code = payload
        .get("templateCode")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &["shop-shipping-template", tenant_id, shop_id, template_code],
    );
    let template_name = payload
        .get("templateName")
        .and_then(|v| v.as_str())
        .unwrap_or("default template");
    let template_status = payload
        .get("templateStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let pricing_mode = payload
        .get("pricingMode")
        .and_then(|v| v.as_str())
        .unwrap_or("fixed");
    let delivery_method = payload
        .get("deliveryMethod")
        .and_then(|v| v.as_str())
        .unwrap_or("standard");
    let base_quantity = payload
        .get("baseQuantity")
        .and_then(|v| v.as_i64())
        .unwrap_or(1);
    let base_fee_amount = payload
        .get("baseFeeAmount")
        .and_then(|v| v.as_str())
        .unwrap_or("0");
    let currency_code = payload
        .get("currencyCode")
        .and_then(|v| v.as_str())
        .unwrap_or("CNY");
    let is_default = payload
        .get("isDefault")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let region_rule_json = payload
        .get("regionRule")
        .cloned()
        .unwrap_or_else(|| serde_json::json!([]))
        .to_string();
    let free_shipping_rule_json = payload
        .get("freeShippingRule")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_shipping_template (id, tenant_id, organization_id, shop_id, template_code, template_name, template_status, pricing_mode, delivery_method, base_quantity, base_fee_amount, currency_code, is_default, region_rule_json, free_shipping_rule_json, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, $11, $12, $13, $14::jsonb, $15::jsonb, $16, $16) ON CONFLICT(id) DO UPDATE SET template_name = EXCLUDED.template_name, template_status = EXCLUDED.template_status, pricing_mode = EXCLUDED.pricing_mode, delivery_method = EXCLUDED.delivery_method, base_quantity = EXCLUDED.base_quantity, base_fee_amount = EXCLUDED.base_fee_amount, currency_code = EXCLUDED.currency_code, is_default = EXCLUDED.is_default, region_rule_json = EXCLUDED.region_rule_json, free_shipping_rule_json = EXCLUDED.free_shipping_rule_json, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(template_code)
                .bind(template_name)
                .bind(template_status)
                .bind(pricing_mode)
                .bind(delivery_method)
                .bind(base_quantity)
                .bind(base_fee_amount)
                .bind(currency_code)
                .bind(is_default)
                .bind(&region_rule_json)
                .bind(&free_shipping_rule_json)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_shipping_template", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("shipping template missing after upsert"))
}

async fn upsert_channel_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let channel_code = payload
        .get("channelCode")
        .and_then(|v| v.as_str())
        .unwrap_or("web");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &["shop-channel", tenant_id, shop_id, channel_code],
    );
    let storefront_status = payload
        .get("storefrontStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let channel_config_json = payload
        .get("channelConfig")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let domain_name = payload.get("domainName").and_then(|v| v.as_str());
    let path_prefix = payload.get("pathPrefix").and_then(|v| v.as_str());
    let theme_code = payload.get("themeCode").and_then(|v| v.as_str());
    let sort_order = payload
        .get("sortOrder")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_channel (id, tenant_id, organization_id, shop_id, channel_code, storefront_status, domain_name, path_prefix, theme_code, channel_config_json, sort_order, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10::jsonb, $11, $12, $12) ON CONFLICT(id) DO UPDATE SET storefront_status = EXCLUDED.storefront_status, domain_name = EXCLUDED.domain_name, path_prefix = EXCLUDED.path_prefix, theme_code = EXCLUDED.theme_code, channel_config_json = EXCLUDED.channel_config_json, sort_order = EXCLUDED.sort_order, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(channel_code)
                .bind(storefront_status)
                .bind(domain_name)
                .bind(path_prefix)
                .bind(theme_code)
                .bind(&channel_config_json)
                .bind(sort_order)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_channel", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("channel missing after upsert"))
}

async fn upsert_service_area_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let area_type = payload
        .get("areaType")
        .and_then(|v| v.as_str())
        .unwrap_or("country");
    let country_code = payload
        .get("countryCode")
        .and_then(|v| v.as_str())
        .unwrap_or("CN");
    let area_key = payload
        .get("areaKey")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &[
            "shop-service-area",
            tenant_id,
            shop_id,
            area_type,
            country_code,
            area_key,
        ],
    );
    let service_status = payload
        .get("serviceStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let service_config_json = payload
        .get("serviceConfig")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let region_code = payload.get("regionCode").and_then(|v| v.as_str());
    let city_code = payload.get("cityCode").and_then(|v| v.as_str());
    let postal_code_pattern = payload.get("postalCodePattern").and_then(|v| v.as_str());
    let delivery_radius_meters = payload.get("deliveryRadiusMeters").and_then(|v| v.as_i64());
    let sort_order = payload
        .get("sortOrder")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_service_area (id, tenant_id, organization_id, shop_id, area_type, country_code, region_code, city_code, area_key, postal_code_pattern, delivery_radius_meters, service_status, service_config_json, sort_order, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, $11, $12, $13::jsonb, $14, $15, $15) ON CONFLICT(id) DO UPDATE SET area_type = EXCLUDED.area_type, country_code = EXCLUDED.country_code, region_code = EXCLUDED.region_code, city_code = EXCLUDED.city_code, area_key = EXCLUDED.area_key, postal_code_pattern = EXCLUDED.postal_code_pattern, delivery_radius_meters = EXCLUDED.delivery_radius_meters, service_status = EXCLUDED.service_status, service_config_json = EXCLUDED.service_config_json, sort_order = EXCLUDED.sort_order, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(area_type)
                .bind(country_code)
                .bind(region_code)
                .bind(city_code)
                .bind(area_key)
                .bind(postal_code_pattern)
                .bind(delivery_radius_meters)
                .bind(service_status)
                .bind(&service_config_json)
                .bind(sort_order)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_service_area", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("service area missing after upsert"))
}

async fn upsert_policy_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let policy_type = payload
        .get("policyType")
        .and_then(|v| v.as_str())
        .unwrap_or("terms");
    let policy_version = payload
        .get("policyVersion")
        .and_then(|v| v.as_i64())
        .unwrap_or(1)
        .to_string();
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &[
            "shop-policy",
            tenant_id,
            shop_id,
            policy_type,
            policy_version.as_str(),
        ],
    );
    let policy_status = payload
        .get("policyStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("draft");
    let policy_version_number = policy_version.parse::<i64>().unwrap_or(1);
    let policy_json = payload
        .get("policy")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let published_at = payload.get("publishedAt").and_then(|v| v.as_str());
    let reviewed_by = payload.get("reviewedBy").and_then(|v| v.as_str());
    let reviewed_at = payload.get("reviewedAt").and_then(|v| v.as_str());

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_policy (id, tenant_id, organization_id, shop_id, policy_type, policy_status, policy_version, policy_json, published_at, reviewed_by, reviewed_at, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8::jsonb, $9, $10, $11, $12, $12) ON CONFLICT(id) DO UPDATE SET policy_status = EXCLUDED.policy_status, policy_version = EXCLUDED.policy_version, policy_json = EXCLUDED.policy_json, published_at = EXCLUDED.published_at, reviewed_by = EXCLUDED.reviewed_by, reviewed_at = EXCLUDED.reviewed_at, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(policy_type)
                .bind(policy_status)
                .bind(policy_version_number)
                .bind(&policy_json)
                .bind(published_at)
                .bind(reviewed_by)
                .bind(reviewed_at)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_policy", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("policy missing after upsert"))
}

async fn upsert_fulfillment_profile_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let id = explicit_id
        .or_else(|| {
            payload
                .get("id")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| stable_storage_id(&["shop-fulfillment", tenant_id, shop_id]));
    let organization_id = organization_id.unwrap_or("default-org");
    let fulfillment_mode = payload
        .get("fulfillmentMode")
        .and_then(|v| v.as_str())
        .unwrap_or("merchant");
    let shipping_origin_region_code = payload
        .get("shippingOriginRegionCode")
        .and_then(|v| v.as_str());
    let service_level_code = payload.get("serviceLevelCode").and_then(|v| v.as_str());
    let after_sales_policy_json = payload
        .get("afterSalesPolicy")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let service_config_json = payload
        .get("serviceConfig")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_fulfillment_profile (id, tenant_id, organization_id, shop_id, fulfillment_mode, shipping_origin_region_code, service_level_code, after_sales_policy_json, service_config_json, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8::jsonb, $9::jsonb, $10, $10) ON CONFLICT(id) DO UPDATE SET fulfillment_mode = EXCLUDED.fulfillment_mode, shipping_origin_region_code = EXCLUDED.shipping_origin_region_code, service_level_code = EXCLUDED.service_level_code, after_sales_policy_json = EXCLUDED.after_sales_policy_json, service_config_json = EXCLUDED.service_config_json, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(fulfillment_mode)
                .bind(shipping_origin_region_code)
                .bind(service_level_code)
                .bind(&after_sales_policy_json)
                .bind(&service_config_json)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_fulfillment_profile", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("fulfillment profile missing after upsert"))
}

async fn upsert_settlement_profile_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let id = explicit_id
        .or_else(|| {
            payload
                .get("id")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| stable_storage_id(&["shop-settlement", tenant_id, shop_id]));
    let organization_id = organization_id.unwrap_or("default-org");
    let settlement_status = payload
        .get("settlementStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let settlement_cycle = payload
        .get("settlementCycle")
        .and_then(|v| v.as_str())
        .unwrap_or("monthly");
    let settlement_currency_code = payload
        .get("settlementCurrencyCode")
        .and_then(|v| v.as_str())
        .unwrap_or("CNY");
    let account_ref = payload.get("accountRef").and_then(|v| v.as_str());
    let risk_hold_days = payload
        .get("riskHoldDays")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let settlement_config_json = payload
        .get("settlementConfig")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let reviewed_by = payload.get("reviewedBy").and_then(|v| v.as_str());
    let reviewed_at = payload.get("reviewedAt").and_then(|v| v.as_str());

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_settlement_profile (id, tenant_id, organization_id, shop_id, settlement_status, settlement_cycle, settlement_currency_code, account_ref, risk_hold_days, settlement_config_json, reviewed_by, reviewed_at, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10::jsonb, $11, $12, $13, $13) ON CONFLICT(id) DO UPDATE SET settlement_status = EXCLUDED.settlement_status, settlement_cycle = EXCLUDED.settlement_cycle, settlement_currency_code = EXCLUDED.settlement_currency_code, account_ref = EXCLUDED.account_ref, risk_hold_days = EXCLUDED.risk_hold_days, settlement_config_json = EXCLUDED.settlement_config_json, reviewed_by = EXCLUDED.reviewed_by, reviewed_at = EXCLUDED.reviewed_at, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(settlement_status)
                .bind(settlement_cycle)
                .bind(settlement_currency_code)
                .bind(account_ref)
                .bind(risk_hold_days)
                .bind(&settlement_config_json)
                .bind(reviewed_by)
                .bind(reviewed_at)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_settlement_profile", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("settlement profile missing after upsert"))
}

async fn upsert_business_hour_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let schedule_type = payload
        .get("scheduleType")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &["shop-business-hour", tenant_id, shop_id, schedule_type],
    );
    let timezone = payload
        .get("timezone")
        .and_then(|v| v.as_str())
        .unwrap_or("Asia/Shanghai");
    let weekly_schedule_json = payload
        .get("weeklySchedule")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let holiday_schedule_json = payload
        .get("holidaySchedule")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let effective_from = payload.get("effectiveFrom").and_then(|v| v.as_str());
    let effective_to = payload.get("effectiveTo").and_then(|v| v.as_str());
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let version = payload.get("version").and_then(|v| v.as_i64()).unwrap_or(0);

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_business_hour (id, tenant_id, organization_id, shop_id, schedule_type, timezone, weekly_schedule_json, holiday_schedule_json, effective_from, effective_to, status, version, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7::jsonb, $8::jsonb, $9, $10, $11, $12, $13, $13) ON CONFLICT(id) DO UPDATE SET schedule_type = EXCLUDED.schedule_type, timezone = EXCLUDED.timezone, weekly_schedule_json = EXCLUDED.weekly_schedule_json, holiday_schedule_json = EXCLUDED.holiday_schedule_json, effective_from = EXCLUDED.effective_from, effective_to = EXCLUDED.effective_to, status = EXCLUDED.status, version = EXCLUDED.version, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(schedule_type)
                .bind(timezone)
                .bind(&weekly_schedule_json)
                .bind(&holiday_schedule_json)
                .bind(effective_from)
                .bind(effective_to)
                .bind(status)
                .bind(version)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_business_hour", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("business hour missing after upsert"))
}

async fn upsert_deposit_account_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let id = explicit_id
        .or_else(|| {
            payload
                .get("id")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| stable_storage_id(&["shop-deposit", tenant_id, shop_id]));
    let organization_id = organization_id.unwrap_or("default-org");
    let deposit_status = payload
        .get("depositStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let currency_code = payload
        .get("currencyCode")
        .and_then(|v| v.as_str())
        .unwrap_or("CNY");
    let required_amount = payload
        .get("requiredAmount")
        .and_then(|v| v.as_str())
        .unwrap_or("0");
    let paid_amount = payload
        .get("paidAmount")
        .and_then(|v| v.as_str())
        .unwrap_or("0");
    let frozen_amount = payload
        .get("frozenAmount")
        .and_then(|v| v.as_str())
        .unwrap_or("0");
    let account_ref = payload.get("accountRef").and_then(|v| v.as_str());
    let due_at = payload.get("dueAt").and_then(|v| v.as_str());
    let reviewed_by = payload.get("reviewedBy").and_then(|v| v.as_str());
    let reviewed_at = payload.get("reviewedAt").and_then(|v| v.as_str());

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_deposit_account (id, tenant_id, organization_id, shop_id, deposit_status, currency_code, required_amount, paid_amount, frozen_amount, account_ref, due_at, reviewed_by, reviewed_at, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $14) ON CONFLICT(id) DO UPDATE SET deposit_status = EXCLUDED.deposit_status, currency_code = EXCLUDED.currency_code, required_amount = EXCLUDED.required_amount, paid_amount = EXCLUDED.paid_amount, frozen_amount = EXCLUDED.frozen_amount, account_ref = EXCLUDED.account_ref, due_at = EXCLUDED.due_at, reviewed_by = EXCLUDED.reviewed_by, reviewed_at = EXCLUDED.reviewed_at, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(deposit_status)
                .bind(currency_code)
                .bind(required_amount)
                .bind(paid_amount)
                .bind(frozen_amount)
                .bind(account_ref)
                .bind(due_at)
                .bind(reviewed_by)
                .bind(reviewed_at)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_deposit_account", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("deposit account missing after upsert"))
}

async fn upsert_application_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let application_no = payload
        .get("applicationNo")
        .and_then(|v| v.as_str())
        .unwrap_or("APP-001");
    let idempotency_key = payload
        .get("idempotencyKey")
        .and_then(|v| v.as_str())
        .unwrap_or(application_no);
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &["shop-application", tenant_id, shop_id, idempotency_key],
    );
    let application_type = payload
        .get("applicationType")
        .and_then(|v| v.as_str())
        .unwrap_or("onboarding");
    let review_status = payload
        .get("reviewStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let legal_entity_snapshot_json = payload
        .get("legalEntitySnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let contact_snapshot_json = payload
        .get("contactSnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let qualification_snapshot_json = payload
        .get("qualificationSnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let submitted_by = payload
        .get("submittedBy")
        .and_then(|v| v.as_str())
        .unwrap_or("system");
    let submitted_at = payload
        .get("submittedAt")
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .unwrap_or_else(|| now.clone());
    let reviewed_by = payload.get("reviewedBy").and_then(|v| v.as_str());
    let reviewed_at = payload.get("reviewedAt").and_then(|v| v.as_str());
    let review_comment = payload.get("reviewComment").and_then(|v| v.as_str());

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_application (id, tenant_id, organization_id, shop_id, application_no, application_type, review_status, legal_entity_snapshot_json, contact_snapshot_json, qualification_snapshot_json, submitted_by, submitted_at, reviewed_by, reviewed_at, review_comment, idempotency_key, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8::jsonb, $9::jsonb, $10::jsonb, $11, $12, $13, $14, $15, $16, $17, $17) ON CONFLICT(id) DO UPDATE SET application_no = EXCLUDED.application_no, application_type = EXCLUDED.application_type, review_status = EXCLUDED.review_status, legal_entity_snapshot_json = EXCLUDED.legal_entity_snapshot_json, contact_snapshot_json = EXCLUDED.contact_snapshot_json, qualification_snapshot_json = EXCLUDED.qualification_snapshot_json, submitted_by = EXCLUDED.submitted_by, submitted_at = EXCLUDED.submitted_at, reviewed_by = EXCLUDED.reviewed_by, reviewed_at = EXCLUDED.reviewed_at, review_comment = EXCLUDED.review_comment, idempotency_key = EXCLUDED.idempotency_key, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(application_no)
                .bind(application_type)
                .bind(review_status)
                .bind(&legal_entity_snapshot_json)
                .bind(&contact_snapshot_json)
                .bind(&qualification_snapshot_json)
                .bind(submitted_by)
                .bind(&submitted_at)
                .bind(reviewed_by)
                .bind(reviewed_at)
                .bind(review_comment)
                .bind(idempotency_key)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_application", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("application missing after upsert"))
}

async fn upsert_verification_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let verification_type = payload
        .get("verificationType")
        .and_then(|v| v.as_str())
        .unwrap_or("business_license");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &["shop-verification", tenant_id, shop_id, verification_type],
    );
    let verification_status = payload
        .get("verificationStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let legal_entity_name = payload.get("legalEntityName").and_then(|v| v.as_str());
    let credential_no_hash = payload.get("credentialNoHash").and_then(|v| v.as_str());
    let credential_media_resource_id = payload
        .get("credentialMediaResourceId")
        .and_then(|v| v.as_str());
    let verification_snapshot_json = payload
        .get("verificationSnapshot")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let expires_at = payload.get("expiresAt").and_then(|v| v.as_str());
    let reviewed_by = payload.get("reviewedBy").and_then(|v| v.as_str());
    let reviewed_at = payload.get("reviewedAt").and_then(|v| v.as_str());

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_verification (id, tenant_id, organization_id, shop_id, verification_type, verification_status, legal_entity_name, credential_no_hash, credential_media_resource_id, verification_snapshot_json, expires_at, reviewed_by, reviewed_at, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10::jsonb, $11, $12, $13, $14, $14) ON CONFLICT(id) DO UPDATE SET verification_status = EXCLUDED.verification_status, legal_entity_name = EXCLUDED.legal_entity_name, credential_no_hash = EXCLUDED.credential_no_hash, credential_media_resource_id = EXCLUDED.credential_media_resource_id, verification_snapshot_json = EXCLUDED.verification_snapshot_json, expires_at = EXCLUDED.expires_at, reviewed_by = EXCLUDED.reviewed_by, reviewed_at = EXCLUDED.reviewed_at, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(verification_type)
                .bind(verification_status)
                .bind(legal_entity_name)
                .bind(credential_no_hash)
                .bind(credential_media_resource_id)
                .bind(&verification_snapshot_json)
                .bind(expires_at)
                .bind(reviewed_by)
                .bind(reviewed_at)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_verification", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("verification missing after upsert"))
}

async fn upsert_risk_signal_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    shop_id: &str,
    explicit_id: Option<String>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let organization_id = organization_id.unwrap_or("default-org");
    let signal_no = payload
        .get("signalNo")
        .and_then(|v| v.as_str())
        .unwrap_or("signal");
    let id = resolve_row_id(
        explicit_id,
        &payload,
        &["shop-risk-signal", tenant_id, shop_id, signal_no],
    );
    let signal_type = payload
        .get("signalType")
        .and_then(|v| v.as_str())
        .unwrap_or("manual");
    let risk_level = payload
        .get("riskLevel")
        .and_then(|v| v.as_str())
        .unwrap_or("low");
    let signal_status = payload
        .get("signalStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("open");
    let source_type = payload.get("sourceType").and_then(|v| v.as_str());
    let source_id = payload.get("sourceId").and_then(|v| v.as_str());
    let risk_score = payload
        .get("riskScore")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let payload_json = payload
        .get("payload")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}))
        .to_string();
    let detected_at = payload
        .get("detectedAt")
        .and_then(|v| v.as_str())
        .unwrap_or(&now);

    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_risk_signal (id, tenant_id, organization_id, shop_id, signal_no, signal_type, risk_level, signal_status, source_type, source_id, risk_score, payload_json, detected_at, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, $11, $12::jsonb, $13, $14, $14) ON CONFLICT(id) DO UPDATE SET signal_type = EXCLUDED.signal_type, risk_level = EXCLUDED.risk_level, signal_status = EXCLUDED.signal_status, source_type = EXCLUDED.source_type, source_id = EXCLUDED.source_id, risk_score = EXCLUDED.risk_score, payload_json = EXCLUDED.payload_json, detected_at = EXCLUDED.detected_at, updated_at = EXCLUDED.updated_at")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(signal_no)
                .bind(signal_type)
                .bind(risk_level)
                .bind(signal_status)
                .bind(source_type)
                .bind(source_id)
                .bind(risk_score)
                .bind(&payload_json)
                .bind(detected_at)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    retrieve_shop_table_row_by_id(db, "commerce_shop_risk_signal", tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("risk signal missing after upsert"))
}

pub async fn resolve_shop_risk_signal_db(
    db: &ShopWriteDb,
    tenant_id: &str,
    shop_id: &str,
    risk_signal_id: &str,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    match db {
        ShopWriteDb::Postgres(pool) => {
            sqlx::query("UPDATE commerce_shop_risk_signal SET signal_status = CAST($1 AS TEXT), resolved_at = CAST($2 AS TEXT), updated_at = CAST($3 AS TEXT) WHERE tenant_id = CAST($4 AS TEXT) AND shop_id = CAST($5 AS TEXT) AND id = CAST($6 AS TEXT)")
                .bind("resolved")
                .bind(&now)
                .bind(&now)
                .bind(tenant_id)
                .bind(shop_id)
                .bind(risk_signal_id)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }
    retrieve_shop_table_row_by_id(db, "commerce_shop_risk_signal", tenant_id, risk_signal_id)
        .await?
        .ok_or_else(|| CommerceServiceError::not_found("risk signal was not found"))
}

pub async fn retrieve_shop_table_row_by_id(
    db: &ShopWriteDb,
    table: &str,
    tenant_id: &str,
    row_id: &str,
) -> Result<Option<serde_json::Value>, CommerceServiceError> {
    match db {
        ShopWriteDb::Postgres(pool) => {
            let sql = format!("SELECT * FROM {table} WHERE tenant_id = CAST($1 AS TEXT) AND id = CAST($2 AS TEXT) LIMIT 1");
            let row = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
                .bind(tenant_id)
                .bind(row_id)
                .fetch_optional(pool)
                .await
                .map_err(storage_error)?;
            Ok(row.map(|r| map_row_json_pg(&r)))
        }
    }
}

pub fn map_row_json_pg(row: &PgRow) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    for column in row.columns() {
        let key = column.name();
        let value = row
            .try_get::<Option<String>, _>(key)
            .ok()
            .flatten()
            .map(serde_json::Value::String)
            .unwrap_or(serde_json::Value::Null);
        object.insert(key.to_owned(), value);
    }
    serde_json::Value::Object(object)
}

pub fn pg_optional_string(row: &PgRow, column: &str) -> Option<String> {
    row.try_get::<Option<String>, _>(column).ok().flatten()
}

pub fn pg_string(row: &PgRow, column: &str) -> String {
    pg_optional_string(row, column).unwrap_or_default()
}

fn storage_error(error: sqlx::Error) -> CommerceServiceError {
    CommerceServiceError::storage(format!("shop storage error: {error}"))
}
pub fn current_timestamp_string() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    format!("{seconds}")
}

pub fn stable_storage_id(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| {
            part.chars()
                .map(|character| {
                    if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                        character
                    } else {
                        '-'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("-")
}

pub fn resolve_row_id(
    explicit_id: Option<String>,
    payload: &serde_json::Value,
    stable_parts: &[&str],
) -> String {
    explicit_id
        .or_else(|| {
            payload
                .get("id")
                .and_then(|value| value.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| stable_storage_id(stable_parts))
}
