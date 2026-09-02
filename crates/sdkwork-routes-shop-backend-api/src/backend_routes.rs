use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use sdkwork_contract_service::CommerceServiceError;
use sdkwork_iam_context_service::IamAppContext;
use serde::Serialize;
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::http_envelope::{
    not_found_response, shop_system_response, success_created_resource, success_list,
    success_paged_list, success_resource, unauthorized_response,
};
use crate::subject::app_runtime_subject_from_extension;
use crate::web_bootstrap::with_backend_request_identity;

use sdkwork_shop_repository_sqlx::shop_subresource_upsert::{
    self, current_timestamp_string, map_row_json_pg, pg_optional_string, pg_string,
    stable_storage_id, ShopWriteDb,
};

fn as_shop_write_db(db: &BackendShopDb) -> ShopWriteDb {
    match db {
        BackendShopDb::Postgres(pool) => ShopWriteDb::Postgres(pool.clone()),
    }
}

#[derive(Clone)]
enum BackendShopDb {
    Postgres(PgPool),
}

#[derive(Clone)]
struct BackendShopAdminState {
    db: BackendShopDb,
}

#[derive(Debug, serde::Deserialize)]
struct ShopListParams {
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ShopSummary {
    id: String,
    tenant_id: String,
    organization_id: String,
    shop_no: String,
    shop_name: String,
    shop_type: String,
    business_model: String,
    storefront_status: String,
    operation_status: String,
    review_status: String,
    default_currency_code: String,
    default_locale: Option<String>,
    timezone: Option<String>,
    version: i64,
    created_at: String,
    updated_at: String,
}

pub fn backend_shop_admin_router_with_postgres_pool(pool: PgPool) -> Router {
    backend_shop_admin_router_with_db(BackendShopDb::Postgres(pool))
}

fn backend_shop_admin_router_with_db(db: BackendShopDb) -> Router {
    with_backend_request_identity(
        Router::new()
            .route("/backend/v3/api/shops", get(list_shops).post(create_shop))
            .route(
                "/backend/v3/api/shops/{shopId}",
                get(retrieve_shop).patch(update_shop),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/readiness",
                get(retrieve_readiness),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/category_bindings",
                get(list_category_bindings).put(upsert_category_bindings),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/brand_authorizations",
                get(list_brand_authorizations).put(upsert_brand_authorizations),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/qualifications",
                get(list_qualifications).put(upsert_qualifications),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/customer_services",
                get(list_customer_services).put(upsert_customer_services),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/return_addresses",
                get(list_return_addresses).put(upsert_return_addresses),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/shipping_templates",
                get(list_shipping_templates).put(upsert_shipping_templates),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/verifications",
                get(list_verifications),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/verifications/{verificationId}",
                patch(update_verification),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/status_events",
                get(list_status_events),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/channels",
                get(list_channels).post(create_channel),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/channels/{channelId}",
                patch(update_channel),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/fulfillment_profile",
                get(retrieve_fulfillment_profile).patch(update_fulfillment_profile),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/settlement_profile",
                get(retrieve_settlement_profile).patch(update_settlement_profile),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/settlement_profile/approve",
                post(approve_settlement_profile),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/settlement_profile/reject",
                post(reject_settlement_profile),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/business_hours",
                get(retrieve_business_hours).patch(update_business_hours),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/service_areas",
                get(list_service_areas).post(create_service_area),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/service_areas/{serviceAreaId}",
                patch(update_service_area),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/policies",
                get(list_policies).post(create_policy),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/policies/{policyId}",
                patch(update_policy),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/deposit_account",
                get(retrieve_deposit_account).patch(update_deposit_account),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/deposit_account/review",
                post(review_deposit_account),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/risk_signals",
                get(list_risk_signals).post(create_risk_signal),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/risk_signals/{riskSignalId}/resolve",
                post(resolve_risk_signal),
            )
            .route(
                "/backend/v3/api/shops/{shopId}/submit_review",
                post(submit_review),
            )
            .route("/backend/v3/api/shops/{shopId}/approve", post(approve_shop))
            .route("/backend/v3/api/shops/{shopId}/reject", post(reject_shop))
            .route("/backend/v3/api/shops/{shopId}/suspend", post(suspend_shop))
            .route("/backend/v3/api/shops/{shopId}/resume", post(resume_shop))
            .route("/backend/v3/api/shops/{shopId}/close", post(close_shop))
            .with_state(BackendShopAdminState { db }),
    )
}

async fn list_shops(
    State(state): State<BackendShopAdminState>,
    Query(params): Query<ShopListParams>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 200);
    match list_shops_db(&state.db, &subject.tenant_id, page, page_size).await {
        Ok((items, total)) => success_paged_list(items, page, page_size, total),
        Err(error) => shop_system_response("shop list is unavailable", error),
    }
}

async fn create_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    match create_shop_db(
        &state.db,
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        payload,
    )
    .await
    {
        Ok(item) => success_created_resource(item),
        Err(error) => shop_system_response("shop create is unavailable", error),
    }
}

async fn retrieve_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    match retrieve_shop_db(&state.db, &subject.tenant_id, &shop_id).await {
        Ok(Some(item)) => success_resource(item),
        Ok(None) => not_found_response("shop was not found"),
        Err(error) => shop_system_response("shop detail is unavailable", error),
    }
}

async fn update_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    match update_shop_db(&state.db, &subject.tenant_id, &shop_id, payload).await {
        Ok(item) => success_resource(item),
        Err(error) => shop_system_response("shop update is unavailable", error),
    }
}

macro_rules! list_table_handler {
    ($name:ident, $table:literal) => {
        async fn $name(
            State(state): State<BackendShopAdminState>,
            runtime_context: Option<Extension<IamAppContext>>,
            Path(shop_id): Path<String>,
        ) -> Response {
            list_shop_table_response(state, runtime_context, shop_id, $table).await
        }
    };
}

macro_rules! upsert_table_handler {
    ($name:ident, $table:literal) => {
        async fn $name(
            State(state): State<BackendShopAdminState>,
            runtime_context: Option<Extension<IamAppContext>>,
            Path(shop_id): Path<String>,
            Json(payload): Json<serde_json::Value>,
        ) -> Response {
            upsert_shop_table_response(
                state,
                runtime_context,
                shop_id,
                None,
                false,
                $table,
                payload,
            )
            .await
        }
    };
}

macro_rules! create_table_handler {
    ($name:ident, $table:literal) => {
        async fn $name(
            State(state): State<BackendShopAdminState>,
            runtime_context: Option<Extension<IamAppContext>>,
            Path(shop_id): Path<String>,
            Json(payload): Json<serde_json::Value>,
        ) -> Response {
            upsert_shop_table_response(state, runtime_context, shop_id, None, true, $table, payload)
                .await
        }
    };
}

macro_rules! upsert_table_handler_with_id {
    ($name:ident, $table:literal, $id_ty:ty) => {
        async fn $name(
            State(state): State<BackendShopAdminState>,
            runtime_context: Option<Extension<IamAppContext>>,
            Path((shop_id, row_id)): Path<(String, $id_ty)>,
            Json(payload): Json<serde_json::Value>,
        ) -> Response {
            upsert_shop_table_response(
                state,
                runtime_context,
                shop_id,
                Some(row_id.to_string()),
                false,
                $table,
                payload,
            )
            .await
        }
    };
}

list_table_handler!(retrieve_readiness, "commerce_shop_readiness");
list_table_handler!(list_category_bindings, "commerce_shop_category_binding");
upsert_table_handler!(upsert_category_bindings, "commerce_shop_category_binding");
list_table_handler!(
    list_brand_authorizations,
    "commerce_shop_brand_authorization"
);
upsert_table_handler!(
    upsert_brand_authorizations,
    "commerce_shop_brand_authorization"
);
list_table_handler!(list_qualifications, "commerce_shop_qualification");
upsert_table_handler!(upsert_qualifications, "commerce_shop_qualification");
list_table_handler!(list_customer_services, "commerce_shop_customer_service");
upsert_table_handler!(upsert_customer_services, "commerce_shop_customer_service");
list_table_handler!(list_return_addresses, "commerce_shop_return_address");
upsert_table_handler!(upsert_return_addresses, "commerce_shop_return_address");
list_table_handler!(list_shipping_templates, "commerce_shop_shipping_template");
upsert_table_handler!(upsert_shipping_templates, "commerce_shop_shipping_template");
list_table_handler!(list_verifications, "commerce_shop_verification");
upsert_table_handler_with_id!(update_verification, "commerce_shop_verification", String);
list_table_handler!(list_status_events, "commerce_shop_status_event");
list_table_handler!(list_channels, "commerce_shop_channel");
create_table_handler!(create_channel, "commerce_shop_channel");
upsert_table_handler_with_id!(update_channel, "commerce_shop_channel", String);
list_table_handler!(
    retrieve_fulfillment_profile,
    "commerce_shop_fulfillment_profile"
);
upsert_table_handler!(
    update_fulfillment_profile,
    "commerce_shop_fulfillment_profile"
);
list_table_handler!(
    retrieve_settlement_profile,
    "commerce_shop_settlement_profile"
);
upsert_table_handler!(
    update_settlement_profile,
    "commerce_shop_settlement_profile"
);
list_table_handler!(retrieve_business_hours, "commerce_shop_business_hour");
upsert_table_handler!(update_business_hours, "commerce_shop_business_hour");
list_table_handler!(list_service_areas, "commerce_shop_service_area");
create_table_handler!(create_service_area, "commerce_shop_service_area");
upsert_table_handler_with_id!(update_service_area, "commerce_shop_service_area", String);
list_table_handler!(list_policies, "commerce_shop_policy");
create_table_handler!(create_policy, "commerce_shop_policy");
upsert_table_handler_with_id!(update_policy, "commerce_shop_policy", String);
list_table_handler!(retrieve_deposit_account, "commerce_shop_deposit_account");
upsert_table_handler!(update_deposit_account, "commerce_shop_deposit_account");
list_table_handler!(list_risk_signals, "commerce_shop_risk_signal");
create_table_handler!(create_risk_signal, "commerce_shop_risk_signal");

async fn resolve_risk_signal(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path((shop_id, risk_signal_id)): Path<(String, String)>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    match shop_subresource_upsert::resolve_shop_risk_signal_db(
        &as_shop_write_db(&state.db),
        &subject.tenant_id,
        &shop_id,
        &risk_signal_id,
    )
    .await
    {
        Ok(item) => success_resource(item),
        Err(error) => shop_system_response("risk signal resolve is unavailable", error),
    }
}

async fn approve_settlement_profile(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    upsert_shop_table_response(
        state,
        runtime_context,
        shop_id,
        None,
        false,
        "commerce_shop_settlement_profile",
        serde_json::json!({ "settlementStatus": "approved" }),
    )
    .await
}

async fn reject_settlement_profile(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    upsert_shop_table_response(
        state,
        runtime_context,
        shop_id,
        None,
        false,
        "commerce_shop_settlement_profile",
        serde_json::json!({ "settlementStatus": "rejected" }),
    )
    .await
}

async fn review_deposit_account(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    upsert_shop_table_response(
        state,
        runtime_context,
        shop_id,
        None,
        false,
        "commerce_shop_deposit_account",
        payload,
    )
    .await
}

async fn submit_review(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    transition_shop_status(
        state,
        runtime_context,
        shop_id,
        "submit_review",
        "pending_review",
        "submitted",
    )
    .await
}

async fn approve_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    transition_shop_status(
        state,
        runtime_context,
        shop_id,
        "approve",
        "active",
        "approved",
    )
    .await
}

async fn reject_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    transition_shop_status(
        state,
        runtime_context,
        shop_id,
        "reject",
        "rejected",
        "rejected",
    )
    .await
}

async fn suspend_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    transition_shop_status(
        state,
        runtime_context,
        shop_id,
        "suspend",
        "suspended",
        "approved",
    )
    .await
}

async fn resume_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    transition_shop_status(
        state,
        runtime_context,
        shop_id,
        "resume",
        "active",
        "approved",
    )
    .await
}

async fn close_shop(
    State(state): State<BackendShopAdminState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    transition_shop_status(
        state,
        runtime_context,
        shop_id,
        "close",
        "closed",
        "approved",
    )
    .await
}

async fn list_shop_table_response(
    state: BackendShopAdminState,
    runtime_context: Option<Extension<IamAppContext>>,
    shop_id: String,
    table: &'static str,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    match list_shop_table_rows_db(&state.db, table, &subject.tenant_id, &shop_id).await {
        Ok(items) => success_list(items),
        Err(error) => shop_system_response("shop resource list is unavailable", error),
    }
}

async fn upsert_shop_table_response(
    state: BackendShopAdminState,
    runtime_context: Option<Extension<IamAppContext>>,
    shop_id: String,
    explicit_id: Option<String>,
    created: bool,
    table: &'static str,
    payload: serde_json::Value,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    match shop_subresource_upsert::upsert_shop_table_row(
        &as_shop_write_db(&state.db),
        table,
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        &shop_id,
        explicit_id,
        payload,
    )
    .await
    {
        Ok(item) if created => success_created_resource(item),
        Ok(item) => success_resource(item),
        Err(error) => shop_system_response("shop resource upsert is unavailable", error),
    }
}

async fn transition_shop_status(
    state: BackendShopAdminState,
    runtime_context: Option<Extension<IamAppContext>>,
    shop_id: String,
    event_type: &'static str,
    operation_status: &'static str,
    review_status: &'static str,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    match transition_shop_db(
        &state.db,
        &ShopStatusTransition {
            tenant_id: &subject.tenant_id,
            organization_id: subject.organization_id.as_deref().unwrap_or("0"),
            shop_id: &shop_id,
            event_type,
            operation_status,
            review_status,
            actor_id: &subject.user_id,
        },
    )
    .await
    {
        Ok(item) => success_resource(item),
        Err(error) => shop_system_response("shop transition is unavailable", error),
    }
}

async fn list_shops_db(
    db: &BackendShopDb,
    tenant_id: &str,
    page: u32,
    page_size: u32,
) -> Result<(Vec<ShopSummary>, u64), CommerceServiceError> {
    let offset = ((page - 1) * page_size) as i64;
    let limit = page_size as i64;
    match db {
        BackendShopDb::Postgres(pool) => {
            let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM commerce_shop WHERE tenant_id = CAST($1 AS TEXT) AND deleted_at IS NULL")
                .bind(tenant_id)
                .fetch_one(pool)
                .await
                .map_err(storage_error)?;
            let rows = sqlx::query("SELECT id, tenant_id, organization_id, shop_no, shop_name, shop_type, business_model, storefront_status, operation_status, review_status, default_currency_code, default_locale, timezone, version, created_at, updated_at FROM commerce_shop WHERE tenant_id = CAST($1 AS TEXT) AND deleted_at IS NULL ORDER BY created_at DESC, id DESC LIMIT $2 OFFSET $3")
                .bind(tenant_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await
                .map_err(storage_error)?;
            Ok((rows.iter().map(map_shop_pg).collect(), total as u64))
        }
    }
}

async fn create_shop_db(
    db: &BackendShopDb,
    tenant_id: &str,
    organization_id: Option<&str>,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let now = current_timestamp_string();
    let org = payload
        .get("organizationId")
        .and_then(|v| v.as_str())
        .or(organization_id)
        .unwrap_or("0");
    let shop_no = payload
        .get("shopNo")
        .and_then(|v| v.as_str())
        .unwrap_or("shop-no");
    let id = payload
        .get("id")
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .unwrap_or_else(|| stable_storage_id(&["shop", tenant_id, shop_no]));
    let shop_name = payload
        .get("shopName")
        .and_then(|v| v.as_str())
        .unwrap_or("shop");
    let shop_type = payload
        .get("shopType")
        .and_then(|v| v.as_str())
        .unwrap_or("official");
    let business_model = payload
        .get("businessModel")
        .and_then(|v| v.as_str())
        .unwrap_or("self_operated");
    let storefront_status = payload
        .get("storefrontStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("draft");
    let operation_status = payload
        .get("operationStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("draft");
    let review_status = payload
        .get("reviewStatus")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let currency = payload
        .get("defaultCurrencyCode")
        .and_then(|v| v.as_str())
        .unwrap_or("CNY");
    let locale = payload.get("defaultLocale").and_then(|v| v.as_str());
    let timezone = payload.get("timezone").and_then(|v| v.as_str());
    match db {
        BackendShopDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop (id, tenant_id, organization_id, shop_no, shop_name, shop_type, business_model, storefront_status, operation_status, review_status, data_scope, default_currency_code, default_locale, timezone, created_at, updated_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, 'organization', $11, $12, $13, $14, $14)")
                .bind(&id).bind(tenant_id).bind(org).bind(shop_no).bind(shop_name).bind(shop_type).bind(business_model).bind(storefront_status).bind(operation_status).bind(review_status).bind(currency).bind(locale).bind(timezone).bind(&now)
                .execute(pool).await.map_err(storage_error)?;
        }
    }
    sync_shop_readiness_db(db, tenant_id, org, &id, operation_status, review_status).await?;
    retrieve_shop_db(db, tenant_id, &id)
        .await?
        .ok_or_else(|| CommerceServiceError::storage("created shop missing"))
}

async fn retrieve_shop_db(
    db: &BackendShopDb,
    tenant_id: &str,
    shop_id: &str,
) -> Result<Option<serde_json::Value>, CommerceServiceError> {
    match db {
        BackendShopDb::Postgres(pool) => {
            let row = sqlx::query("SELECT * FROM commerce_shop WHERE tenant_id = CAST($1 AS TEXT) AND id = CAST($2 AS TEXT) AND deleted_at IS NULL LIMIT 1")
                .bind(tenant_id).bind(shop_id).fetch_optional(pool).await.map_err(storage_error)?;
            Ok(row.map(|v| map_row_json_pg(&v)))
        }
    }
}

async fn update_shop_db(
    db: &BackendShopDb,
    tenant_id: &str,
    shop_id: &str,
    payload: serde_json::Value,
) -> Result<serde_json::Value, CommerceServiceError> {
    let existing = retrieve_shop_db(db, tenant_id, shop_id)
        .await?
        .ok_or_else(|| CommerceServiceError::not_found("shop was not found"))?;
    let now = current_timestamp_string();
    let shop_name = payload
        .get("shopName")
        .and_then(|v| v.as_str())
        .or_else(|| existing.get("shop_name").and_then(|v| v.as_str()))
        .unwrap_or("shop");
    let storefront_status = payload
        .get("storefrontStatus")
        .and_then(|v| v.as_str())
        .or_else(|| existing.get("storefront_status").and_then(|v| v.as_str()))
        .unwrap_or("draft");
    let operation_status = payload
        .get("operationStatus")
        .and_then(|v| v.as_str())
        .or_else(|| existing.get("operation_status").and_then(|v| v.as_str()))
        .unwrap_or("draft");
    let review_status = payload
        .get("reviewStatus")
        .and_then(|v| v.as_str())
        .or_else(|| existing.get("review_status").and_then(|v| v.as_str()))
        .unwrap_or("pending");
    match db {
        BackendShopDb::Postgres(pool) => {
            sqlx::query("UPDATE commerce_shop SET shop_name = $1, storefront_status = $2, operation_status = $3, review_status = $4, updated_at = $5 WHERE tenant_id = CAST($6 AS TEXT) AND id = CAST($7 AS TEXT)")
                .bind(shop_name).bind(storefront_status).bind(operation_status).bind(review_status).bind(&now).bind(tenant_id).bind(shop_id)
                .execute(pool).await.map_err(storage_error)?;
        }
    }
    retrieve_shop_db(db, tenant_id, shop_id)
        .await?
        .ok_or_else(|| CommerceServiceError::not_found("shop was not found"))
}

struct ShopStatusTransition<'a> {
    tenant_id: &'a str,
    organization_id: &'a str,
    shop_id: &'a str,
    event_type: &'a str,
    operation_status: &'a str,
    review_status: &'a str,
    actor_id: &'a str,
}

async fn transition_shop_db(
    db: &BackendShopDb,
    transition: &ShopStatusTransition<'_>,
) -> Result<serde_json::Value, CommerceServiceError> {
    let existing = retrieve_shop_db(db, transition.tenant_id, transition.shop_id)
        .await?
        .ok_or_else(|| CommerceServiceError::not_found("shop was not found"))?;
    let from_status = existing
        .get("operation_status")
        .and_then(|value| value.as_str());

    let updated = update_shop_db(
        db,
        transition.tenant_id,
        transition.shop_id,
        serde_json::json!({
            "storefrontStatus": transition.operation_status,
            "operationStatus": transition.operation_status,
            "reviewStatus": transition.review_status
        }),
    )
    .await?;

    insert_shop_status_event_db(db, transition, from_status, transition.operation_status).await?;

    sync_shop_readiness_db(
        db,
        transition.tenant_id,
        transition.organization_id,
        transition.shop_id,
        transition.operation_status,
        transition.review_status,
    )
    .await?;

    Ok(updated)
}

async fn insert_shop_status_event_db(
    db: &BackendShopDb,
    transition: &ShopStatusTransition<'_>,
    from_status: Option<&str>,
    to_status: &str,
) -> Result<(), CommerceServiceError> {
    let now = current_timestamp_string();
    let id = stable_storage_id(&[
        "shop-status-event",
        transition.tenant_id,
        transition.shop_id,
        transition.event_type,
        &now,
    ]);
    let event_no = format!("EVT-{id}");
    let idempotency_key = stable_storage_id(&[
        transition.tenant_id,
        transition.shop_id,
        transition.event_type,
        &now,
    ]);

    match db {
        BackendShopDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_status_event (id, tenant_id, organization_id, shop_id, event_no, event_type, from_status, to_status, actor_type, actor_id, idempotency_key, created_at) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9, $10, $11, $12)")
                .bind(&id)
                .bind(transition.tenant_id)
                .bind(transition.organization_id)
                .bind(transition.shop_id)
                .bind(&event_no)
                .bind(transition.event_type)
                .bind(from_status)
                .bind(to_status)
                .bind("admin")
                .bind(transition.actor_id)
                .bind(&idempotency_key)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    Ok(())
}

async fn sync_shop_readiness_db(
    db: &BackendShopDb,
    tenant_id: &str,
    organization_id: &str,
    shop_id: &str,
    operation_status: &str,
    review_status: &str,
) -> Result<(), CommerceServiceError> {
    let now = current_timestamp_string();
    let readiness_scope = "shop_launch";
    let id = stable_storage_id(&["shop-readiness", tenant_id, shop_id, readiness_scope]);
    let (readiness_status, blocking_count, warning_count) = match operation_status {
        "active" if review_status == "approved" => ("ready", 0, 0),
        "pending_review" => ("pending_review", 1, 0),
        "rejected" => ("blocked", 2, 0),
        "suspended" => ("suspended", 1, 1),
        "closed" => ("closed", 0, 0),
        _ => ("not_ready", 3, 1),
    };
    let checklist_json = serde_json::json!([
        {
            "code": "qualification",
            "status": if blocking_count > 0 { "pending" } else { "done" }
        },
        {
            "code": "settlement_profile",
            "status": if operation_status == "active" { "done" } else { "pending" }
        },
        { "code": "review", "status": review_status }
    ])
    .to_string();

    match db {
        BackendShopDb::Postgres(pool) => {
            sqlx::query("INSERT INTO commerce_shop_readiness (id, tenant_id, organization_id, shop_id, readiness_scope, readiness_status, blocking_count, warning_count, checklist_json, evaluated_at, created_at, updated_at, version) VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $7, $8, $9::jsonb, $10, $10, $10, 0) ON CONFLICT(id) DO UPDATE SET readiness_status = EXCLUDED.readiness_status, blocking_count = EXCLUDED.blocking_count, warning_count = EXCLUDED.warning_count, checklist_json = EXCLUDED.checklist_json, evaluated_at = EXCLUDED.evaluated_at, updated_at = EXCLUDED.updated_at, version = commerce_shop_readiness.version + 1")
                .bind(&id)
                .bind(tenant_id)
                .bind(organization_id)
                .bind(shop_id)
                .bind(readiness_scope)
                .bind(readiness_status)
                .bind(blocking_count)
                .bind(warning_count)
                .bind(&checklist_json)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(storage_error)?;
        }
    }

    Ok(())
}

async fn list_shop_table_rows_db(
    db: &BackendShopDb,
    table: &str,
    tenant_id: &str,
    shop_id: &str,
) -> Result<Vec<serde_json::Value>, CommerceServiceError> {
    match db {
        BackendShopDb::Postgres(pool) => {
            let sql = format!("SELECT * FROM {table} WHERE tenant_id = CAST($1 AS TEXT) AND shop_id = CAST($2 AS TEXT) ORDER BY created_at DESC, id DESC");
            let rows = sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
                .bind(tenant_id)
                .bind(shop_id)
                .fetch_all(pool)
                .await
                .map_err(storage_error)?;
            Ok(rows.iter().map(map_row_json_pg).collect())
        }
    }
}

fn map_shop_pg(row: &PgRow) -> ShopSummary {
    ShopSummary {
        id: pg_string(row, "id"),
        tenant_id: pg_string(row, "tenant_id"),
        organization_id: pg_string(row, "organization_id"),
        shop_no: pg_string(row, "shop_no"),
        shop_name: pg_string(row, "shop_name"),
        shop_type: pg_string(row, "shop_type"),
        business_model: pg_string(row, "business_model"),
        storefront_status: pg_string(row, "storefront_status"),
        operation_status: pg_string(row, "operation_status"),
        review_status: pg_string(row, "review_status"),
        default_currency_code: pg_string(row, "default_currency_code"),
        default_locale: pg_optional_string(row, "default_locale"),
        timezone: pg_optional_string(row, "timezone"),
        version: row.try_get::<i64, _>("version").unwrap_or(0),
        created_at: pg_string(row, "created_at"),
        updated_at: pg_string(row, "updated_at"),
    }
}

fn storage_error(error: sqlx::Error) -> CommerceServiceError {
    CommerceServiceError::storage(format!("shop storage error: {error}"))
}
