use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use sdkwork_contract_service::CommerceServiceError;
use sdkwork_iam_context_service::IamAppContext;
use sdkwork_merchandise_repository_sqlx::PostgresCommerceCatalogStore;
use sdkwork_merchandise_service::{
    ArchiveSpuCommand, CreateProductSpuCommand, ProductSpuListQuery, PublishSpuCommand,
    UpdateProductSpuCommand,
};
use sdkwork_shop_repository_sqlx::PostgresCommerceShopStore;
use sdkwork_shop_service::{
    ShopDetailQuery, ShopListQuery, ShopPage, ShopScopeQuery, ShopSummaryView,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::http_envelope::{
    not_found_response, shop_system_response, success_created_resource, success_list,
    success_paged_list, success_resource, unauthorized_response, validation_response,
};
use crate::subject::app_runtime_subject_from_extension;
use sdkwork_merchandise_web_support::{
    map_spu, CommerceCatalogStore, CreateSpuBody, UpdateSpuBody,
};

pub type CommerceShopFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, CommerceServiceError>> + Send + 'a>>;

pub trait CommerceShopStore: Send + Sync {
    fn list_shops<'a>(
        &'a self,
        query: ShopListQuery,
    ) -> CommerceShopFuture<'a, ShopPage<ShopSummaryView>>;
    fn retrieve_shop<'a>(
        &'a self,
        query: ShopDetailQuery,
    ) -> CommerceShopFuture<'a, Option<ShopSummaryView>>;
    fn retrieve_current_shop<'a>(
        &'a self,
        scope: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Option<ShopSummaryView>>;
    fn list_dashboard_snapshots<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn list_category_bindings<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_category_bindings<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_brand_authorizations<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_brand_authorizations<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_qualifications<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_qualifications<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_customer_services<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_customer_services<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_return_addresses<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_return_addresses<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_shipping_templates<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_shipping_templates<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_applications<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_applications<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_verifications<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn list_status_events<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn list_channels<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_channels<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn find_fulfillment_profile<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Option<serde_json::Value>>;
    fn upsert_fulfillment_profile<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn find_settlement_profile<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Option<serde_json::Value>>;
    fn upsert_settlement_profile<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn find_business_hours<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Option<serde_json::Value>>;
    fn upsert_business_hours<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn find_readiness<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Option<serde_json::Value>>;
    fn find_deposit_account<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Option<serde_json::Value>>;
    fn list_service_areas<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_service_areas<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_policies<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn upsert_policies<'a>(
        &'a self,
        scope: ShopScopeQuery,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_risk_signals<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, Vec<serde_json::Value>>;
    fn list_shop_orders<'a>(
        &'a self,
        scope: ShopScopeQuery,
        page: u32,
        page_size: u32,
    ) -> CommerceShopFuture<'a, ShopPage<serde_json::Value>>;
    fn retrieve_shop_order<'a>(
        &'a self,
        scope: ShopScopeQuery,
        order_id: String,
    ) -> CommerceShopFuture<'a, Option<serde_json::Value>>;
    fn create_shop_fulfillment<'a>(
        &'a self,
        scope: ShopScopeQuery,
        order_id: String,
        payload: serde_json::Value,
    ) -> CommerceShopFuture<'a, serde_json::Value>;
    fn list_settlements<'a>(
        &'a self,
        query: ShopScopeQuery,
    ) -> CommerceShopFuture<'a, ShopPage<serde_json::Value>>;
}

macro_rules! impl_shop_store_forward {
    ($store:ty) => {
        impl CommerceShopStore for $store {
            fn list_shops<'a>(
                &'a self,
                query: ShopListQuery,
            ) -> CommerceShopFuture<'a, ShopPage<ShopSummaryView>> {
                Box::pin(async move { self.list_shops(query).await })
            }
            fn retrieve_shop<'a>(
                &'a self,
                query: ShopDetailQuery,
            ) -> CommerceShopFuture<'a, Option<ShopSummaryView>> {
                Box::pin(async move { self.retrieve_shop(query).await })
            }
            fn retrieve_current_shop<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Option<ShopSummaryView>> {
                Box::pin(async move { self.retrieve_current_shop(scope).await })
            }
            fn list_dashboard_snapshots<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_dashboard_snapshots(scope).await })
            }
            fn list_category_bindings<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_category_bindings(scope).await })
            }
            fn upsert_category_bindings<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_category_bindings(scope, payload).await })
            }
            fn list_brand_authorizations<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_brand_authorizations(scope).await })
            }
            fn upsert_brand_authorizations<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_brand_authorizations(scope, payload).await })
            }
            fn list_qualifications<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_qualifications(scope).await })
            }
            fn upsert_qualifications<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_qualifications(scope, payload).await })
            }
            fn list_customer_services<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_customer_services(scope).await })
            }
            fn upsert_customer_services<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_customer_services(scope, payload).await })
            }
            fn list_return_addresses<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_return_addresses(scope).await })
            }
            fn upsert_return_addresses<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_return_addresses(scope, payload).await })
            }
            fn list_shipping_templates<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_shipping_templates(scope).await })
            }
            fn upsert_shipping_templates<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_shipping_templates(scope, payload).await })
            }
            fn list_applications<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_applications(scope).await })
            }
            fn upsert_applications<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_applications(scope, payload).await })
            }
            fn list_verifications<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_verifications(scope).await })
            }
            fn list_status_events<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_status_events(scope).await })
            }
            fn list_channels<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_channels(scope).await })
            }
            fn upsert_channels<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_channels(scope, payload).await })
            }
            fn find_fulfillment_profile<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Option<serde_json::Value>> {
                Box::pin(async move { self.find_fulfillment_profile(scope).await })
            }
            fn upsert_fulfillment_profile<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_fulfillment_profile(scope, payload).await })
            }
            fn find_settlement_profile<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Option<serde_json::Value>> {
                Box::pin(async move { self.find_settlement_profile(scope).await })
            }
            fn upsert_settlement_profile<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_settlement_profile(scope, payload).await })
            }
            fn find_business_hours<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Option<serde_json::Value>> {
                Box::pin(async move { self.find_business_hours(scope).await })
            }
            fn upsert_business_hours<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_business_hours(scope, payload).await })
            }
            fn find_readiness<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Option<serde_json::Value>> {
                Box::pin(async move { self.find_readiness(scope).await })
            }
            fn find_deposit_account<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Option<serde_json::Value>> {
                Box::pin(async move { self.find_deposit_account(scope).await })
            }
            fn list_service_areas<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_service_areas(scope).await })
            }
            fn upsert_service_areas<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_service_areas(scope, payload).await })
            }
            fn list_policies<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_policies(scope).await })
            }
            fn upsert_policies<'a>(
                &'a self,
                scope: ShopScopeQuery,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move { self.upsert_policies(scope, payload).await })
            }
            fn list_risk_signals<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, Vec<serde_json::Value>> {
                Box::pin(async move { self.list_risk_signals(scope).await })
            }
            fn list_shop_orders<'a>(
                &'a self,
                scope: ShopScopeQuery,
                page: u32,
                page_size: u32,
            ) -> CommerceShopFuture<'a, ShopPage<serde_json::Value>> {
                Box::pin(async move { self.list_shop_orders(scope, page, page_size).await })
            }
            fn retrieve_shop_order<'a>(
                &'a self,
                scope: ShopScopeQuery,
                order_id: String,
            ) -> CommerceShopFuture<'a, Option<serde_json::Value>> {
                Box::pin(async move { self.retrieve_shop_order(scope, &order_id).await })
            }
            fn create_shop_fulfillment<'a>(
                &'a self,
                scope: ShopScopeQuery,
                order_id: String,
                payload: serde_json::Value,
            ) -> CommerceShopFuture<'a, serde_json::Value> {
                Box::pin(async move {
                    self.create_shop_fulfillment(scope, &order_id, payload)
                        .await
                })
            }
            fn list_settlements<'a>(
                &'a self,
                scope: ShopScopeQuery,
            ) -> CommerceShopFuture<'a, ShopPage<serde_json::Value>> {
                Box::pin(async move { self.list_settlements(scope).await })
            }
        }
    };
}

impl_shop_store_forward!(PostgresCommerceShopStore);

#[derive(Clone)]
struct AppShopState {
    shop: Arc<dyn CommerceShopStore>,
    catalog: Arc<dyn CommerceCatalogStore>,
}

#[derive(Debug, Deserialize)]
struct ShopListParams {
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ShopSummaryResponse {
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
    data_scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_media_resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cover_media_resource_id: Option<String>,
    default_currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
    version: i64,
    created_at: String,
    updated_at: String,
}

pub fn app_shop_router_with_postgres_pool(pool: PgPool) -> Router {
    build_app_shop_router(
        Arc::new(PostgresCommerceShopStore::new(pool.clone())),
        Arc::new(PostgresCommerceCatalogStore::new(pool)),
    )
}

pub fn build_app_shop_router(
    shop: Arc<dyn CommerceShopStore>,
    catalog: Arc<dyn CommerceCatalogStore>,
) -> Router {
    Router::new()
        .route("/app/v3/api/shops", get(list_shops))
        .route("/app/v3/api/shops/{shopId}", get(retrieve_shop))
        .route("/app/v3/api/shops/current", get(retrieve_current_shop))
        .route(
            "/app/v3/api/shops/current/dashboard",
            get(retrieve_current_dashboard),
        )
        .route(
            "/app/v3/api/shops/current/readiness",
            get(get_current_readiness),
        )
        .route(
            "/app/v3/api/shops/current/category_bindings",
            get(list_current_category_bindings).put(upsert_current_category_bindings),
        )
        .route(
            "/app/v3/api/shops/current/brand_authorizations",
            get(list_current_brand_authorizations).put(upsert_current_brand_authorizations),
        )
        .route(
            "/app/v3/api/shops/current/qualifications",
            get(list_current_qualifications).put(upsert_current_qualifications),
        )
        .route(
            "/app/v3/api/shops/current/customer_services",
            get(list_current_customer_services).put(upsert_current_customer_services),
        )
        .route(
            "/app/v3/api/shops/current/return_addresses",
            get(list_current_return_addresses).put(upsert_current_return_addresses),
        )
        .route(
            "/app/v3/api/shops/current/shipping_templates",
            get(list_current_shipping_templates).put(upsert_current_shipping_templates),
        )
        .route(
            "/app/v3/api/shops/current/fulfillment_profile",
            get(get_current_fulfillment_profile).patch(patch_current_fulfillment_profile),
        )
        .route(
            "/app/v3/api/shops/current/settlement_profile",
            get(get_current_settlement_profile).patch(patch_current_settlement_profile),
        )
        .route(
            "/app/v3/api/shops/current/business_hours",
            get(get_current_business_hours).patch(patch_current_business_hours),
        )
        .route(
            "/app/v3/api/shops/current/applications",
            get(list_current_applications),
        )
        .route(
            "/app/v3/api/shops/current/verifications",
            get(list_current_verifications),
        )
        .route(
            "/app/v3/api/shops/current/status_events",
            get(list_current_status_events),
        )
        .route(
            "/app/v3/api/shops/current/channels",
            get(list_current_channels),
        )
        .route(
            "/app/v3/api/shops/current/service_areas",
            get(list_current_service_areas),
        )
        .route(
            "/app/v3/api/shops/current/policies",
            get(list_current_policies),
        )
        .route(
            "/app/v3/api/shops/current/risk_signals",
            get(list_current_risk_signals),
        )
        .route(
            "/app/v3/api/shops/current/channels/{channelId}",
            patch(patch_current_channel),
        )
        .route(
            "/app/v3/api/shops/current/service_areas",
            post(create_current_service_area),
        )
        .route(
            "/app/v3/api/shops/current/service_areas/{serviceAreaId}",
            patch(patch_current_service_area),
        )
        .route(
            "/app/v3/api/shops/current/policies/{policyId}",
            patch(patch_current_policy),
        )
        .route(
            "/app/v3/api/shops/current/applications",
            post(create_current_application),
        )
        .route(
            "/app/v3/api/shops/current/deposit_account",
            get(get_current_deposit_account),
        )
        .route(
            "/app/v3/api/shops/current/products",
            get(list_current_products).post(create_current_product),
        )
        .route(
            "/app/v3/api/shops/current/products/{productId}",
            patch(update_current_product),
        )
        .route(
            "/app/v3/api/shops/current/products/{productId}/publish",
            post(publish_current_product),
        )
        .route(
            "/app/v3/api/shops/current/products/{productId}/unpublish",
            post(unpublish_current_product),
        )
        .route("/app/v3/api/shops/current/orders", get(list_current_orders))
        .route(
            "/app/v3/api/shops/current/orders/{orderId}",
            get(retrieve_current_order),
        )
        .route(
            "/app/v3/api/shops/current/orders/{orderId}/fulfillments",
            post(create_current_order_fulfillment),
        )
        .route(
            "/app/v3/api/shops/current/settlements",
            get(list_current_settlements),
        )
        .with_state(AppShopState { shop, catalog })
}

async fn current_scope(
    runtime_context: Option<Extension<IamAppContext>>,
) -> Result<ShopScopeQuery, Response> {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return Err(unauthorized_response(message)),
    };
    ShopScopeQuery::new(&subject.tenant_id, subject.organization_id.as_deref())
        .map_err(|error| validation_response(error.message()))
}

async fn current_list_handler<F, Fut>(
    state: AppShopState,
    runtime_context: Option<Extension<IamAppContext>>,
    fetch: F,
) -> Response
where
    F: FnOnce(Arc<dyn CommerceShopStore>, ShopScopeQuery) -> Fut,
    Fut: Future<Output = Result<Vec<serde_json::Value>, CommerceServiceError>>,
{
    let scope = match current_scope(runtime_context).await {
        Ok(scope) => scope,
        Err(resp) => return resp,
    };
    match fetch(state.shop.clone(), scope).await {
        Ok(items) => success_list(items),
        Err(error) => shop_system_response("shop read model is unavailable", error),
    }
}

async fn current_single_handler<F, Fut>(
    state: AppShopState,
    runtime_context: Option<Extension<IamAppContext>>,
    fetch: F,
) -> Response
where
    F: FnOnce(Arc<dyn CommerceShopStore>, ShopScopeQuery) -> Fut,
    Fut: Future<Output = Result<Option<serde_json::Value>, CommerceServiceError>>,
{
    let scope = match current_scope(runtime_context).await {
        Ok(scope) => scope,
        Err(resp) => return resp,
    };
    match fetch(state.shop.clone(), scope).await {
        Ok(Some(item)) => success_resource(item),
        Ok(None) => not_found_response("shop resource was not found"),
        Err(error) => shop_system_response("shop read model is unavailable", error),
    }
}

async fn current_write_handler<F, Fut>(
    state: AppShopState,
    runtime_context: Option<Extension<IamAppContext>>,
    body: serde_json::Value,
    write: F,
) -> Response
where
    F: FnOnce(Arc<dyn CommerceShopStore>, ShopScopeQuery, serde_json::Value) -> Fut,
    Fut: Future<Output = Result<serde_json::Value, CommerceServiceError>>,
{
    let scope = match current_scope(runtime_context).await {
        Ok(scope) => scope,
        Err(resp) => return resp,
    };
    match write(state.shop.clone(), scope, body).await {
        Ok(item) => success_resource(item),
        Err(error) => shop_system_response("shop write model is unavailable", error),
    }
}

async fn current_create_handler<F, Fut>(
    state: AppShopState,
    runtime_context: Option<Extension<IamAppContext>>,
    body: serde_json::Value,
    create: F,
) -> Response
where
    F: FnOnce(Arc<dyn CommerceShopStore>, ShopScopeQuery, serde_json::Value) -> Fut,
    Fut: Future<Output = Result<serde_json::Value, CommerceServiceError>>,
{
    let scope = match current_scope(runtime_context).await {
        Ok(scope) => scope,
        Err(resp) => return resp,
    };
    match create(state.shop.clone(), scope, body).await {
        Ok(item) => success_created_resource(item),
        Err(error) => shop_system_response("shop create model is unavailable", error),
    }
}

async fn list_current_category_bindings(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_category_bindings(scope).await
    })
    .await
}

async fn upsert_current_category_bindings(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_category_bindings(scope, body).await },
    )
    .await
}

async fn list_current_brand_authorizations(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_brand_authorizations(scope).await
    })
    .await
}

async fn upsert_current_brand_authorizations(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_brand_authorizations(scope, body).await },
    )
    .await
}

async fn list_current_qualifications(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_qualifications(scope).await
    })
    .await
}

async fn upsert_current_qualifications(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_qualifications(scope, body).await },
    )
    .await
}

async fn list_current_customer_services(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_customer_services(scope).await
    })
    .await
}

async fn upsert_current_customer_services(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_customer_services(scope, body).await },
    )
    .await
}

async fn list_current_return_addresses(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_return_addresses(scope).await
    })
    .await
}

async fn upsert_current_return_addresses(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_return_addresses(scope, body).await },
    )
    .await
}

async fn list_current_shipping_templates(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_shipping_templates(scope).await
    })
    .await
}

async fn upsert_current_shipping_templates(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_shipping_templates(scope, body).await },
    )
    .await
}

async fn get_current_fulfillment_profile(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_single_handler(state, runtime_context, |store, scope| async move {
        store.find_fulfillment_profile(scope).await
    })
    .await
}

async fn patch_current_fulfillment_profile(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_fulfillment_profile(scope, body).await },
    )
    .await
}

async fn get_current_settlement_profile(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_single_handler(state, runtime_context, |store, scope| async move {
        store.find_settlement_profile(scope).await
    })
    .await
}

async fn patch_current_settlement_profile(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_settlement_profile(scope, body).await },
    )
    .await
}

async fn get_current_business_hours(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_single_handler(state, runtime_context, |store, scope| async move {
        store.find_business_hours(scope).await
    })
    .await
}

async fn patch_current_business_hours(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_business_hours(scope, body).await },
    )
    .await
}

async fn list_current_applications(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_applications(scope).await
    })
    .await
}

async fn list_current_verifications(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_verifications(scope).await
    })
    .await
}

async fn list_current_status_events(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_status_events(scope).await
    })
    .await
}

async fn list_current_channels(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_channels(scope).await
    })
    .await
}

async fn list_current_service_areas(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_service_areas(scope).await
    })
    .await
}

async fn list_current_policies(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_policies(scope).await
    })
    .await
}

async fn list_current_risk_signals(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_list_handler(state, runtime_context, |store, scope| async move {
        store.list_risk_signals(scope).await
    })
    .await
}

async fn upsert_current_channels(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_channels(scope, body).await },
    )
    .await
}

async fn upsert_current_service_areas(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_service_areas(scope, body).await },
    )
    .await
}

async fn upsert_current_policies(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_write_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_policies(scope, body).await },
    )
    .await
}

async fn list_shops(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<ShopListParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let query = match ShopListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        params.page.unwrap_or(1),
        params.page_size.unwrap_or(20),
    ) {
        Ok(v) => v,
        Err(e) => return validation_response(e.message()),
    };
    match state.shop.list_shops(query).await {
        Ok(page) => success_paged_list(
            page.items.into_iter().map(map_shop_summary).collect(),
            page.page,
            page.page_size,
            page.total,
        ),
        Err(error) => shop_system_response("shop list is unavailable", error),
    }
}

async fn retrieve_shop(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(shop_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let query = match ShopDetailQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        &shop_id,
    ) {
        Ok(v) => v,
        Err(e) => return validation_response(e.message()),
    };
    match state.shop.retrieve_shop(query).await {
        Ok(Some(shop)) => success_resource(map_shop_summary(shop)),
        Ok(None) => not_found_response("shop was not found"),
        Err(error) => shop_system_response("shop read model is unavailable", error),
    }
}

async fn retrieve_current_shop(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let scope = match current_scope(runtime_context).await {
        Ok(v) => v,
        Err(r) => return r,
    };
    match state.shop.retrieve_current_shop(scope).await {
        Ok(Some(shop)) => success_resource(map_shop_summary(shop)),
        Ok(None) => not_found_response("current shop was not found"),
        Err(error) => shop_system_response("shop read model is unavailable", error),
    }
}

async fn retrieve_current_dashboard(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let scope = match current_scope(runtime_context).await {
        Ok(v) => v,
        Err(r) => return r,
    };
    match state.shop.list_dashboard_snapshots(scope).await {
        Ok(items) => success_resource(serde_json::json!({"snapshots": items})),
        Err(error) => shop_system_response("shop dashboard is unavailable", error),
    }
}

async fn get_current_readiness(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_single_handler(state, runtime_context, |store, scope| async move {
        store.find_readiness(scope).await
    })
    .await
}

async fn get_current_deposit_account(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    current_single_handler(state, runtime_context, |store, scope| async move {
        store.find_deposit_account(scope).await
    })
    .await
}

async fn patch_current_channel(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(channel_id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let mut payload = body;
    if let Some(map) = payload.as_object_mut() {
        map.insert("id".into(), channel_id.into());
    }
    upsert_current_channels(State(state), runtime_context, Json(payload)).await
}

async fn create_current_service_area(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_create_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_service_areas(scope, body).await },
    )
    .await
}

async fn patch_current_service_area(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(service_area_id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let mut payload = body;
    if let Some(map) = payload.as_object_mut() {
        map.insert("id".into(), service_area_id.into());
    }
    upsert_current_service_areas(State(state), runtime_context, Json(payload)).await
}

async fn patch_current_policy(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(policy_id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let mut payload = body;
    if let Some(map) = payload.as_object_mut() {
        map.insert("id".into(), policy_id.into());
    }
    upsert_current_policies(State(state), runtime_context, Json(payload)).await
}

async fn create_current_application(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_create_handler(
        state,
        runtime_context,
        body,
        |store, scope, body| async move { store.upsert_applications(scope, body).await },
    )
    .await
}

async fn list_current_products(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let query = match ProductSpuListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        None,
        None,
        None,
        None,
        None,
        None,
    ) {
        Ok(v) => v,
        Err(e) => return validation_response(e.message()),
    };
    match state.catalog.list_spus(query).await {
        Ok(items) => success_paged_list(items.into_iter().map(map_spu).collect(), 1, 20, 0),
        Err(error) => shop_system_response("shop products are unavailable", error),
    }
}

async fn create_current_product(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<CreateSpuBody>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let organization_id = match subject.organization_id {
        Some(v) => v,
        None => return validation_response("organization_id is required"),
    };
    let command = CreateProductSpuCommand {
        tenant_id: subject.tenant_id,
        organization_id,
        spu_no: body.spu_no,
        title: body.title,
        subtitle: body.subtitle,
        description: body.description,
        product_type: body.product_type,
        category_id: body.category_id,
        visible_surfaces: body.visible_surfaces.unwrap_or_else(|| "all".into()),
    };
    if let Err(error) = command.validate() {
        return validation_response(error.message());
    }
    match state.catalog.create_spu(command).await {
        Ok(spu) => success_created_resource(map_spu(spu)),
        Err(error) => shop_system_response("shop product create is unavailable", error),
    }
}

async fn update_current_product(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(product_id): Path<String>,
    Json(body): Json<UpdateSpuBody>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let command = UpdateProductSpuCommand {
        tenant_id: subject.tenant_id,
        spu_id: product_id,
        title: body.title,
        subtitle: body.subtitle,
        description: body.description,
        category_id: body.category_id,
        visible_surfaces: body.visible_surfaces,
    };
    match state.catalog.update_spu(command).await {
        Ok(spu) => success_resource(map_spu(spu)),
        Err(error) => shop_system_response("shop product update is unavailable", error),
    }
}

async fn publish_current_product(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(product_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let command = PublishSpuCommand {
        tenant_id: subject.tenant_id,
        spu_id: product_id,
    };
    if let Err(error) = command.validate() {
        return validation_response(error.message());
    }
    match state.catalog.publish_spu(command).await {
        Ok(spu) => success_resource(map_spu(spu)),
        Err(error) => shop_system_response("shop product publish is unavailable", error),
    }
}

async fn unpublish_current_product(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(product_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(v) => v,
        Err(m) => return unauthorized_response(m),
    };
    let command = ArchiveSpuCommand {
        tenant_id: subject.tenant_id,
        spu_id: product_id,
    };
    if let Err(error) = command.validate() {
        return validation_response(error.message());
    }
    match state.catalog.archive_spu(command).await {
        Ok(spu) => success_resource(map_spu(spu)),
        Err(error) => shop_system_response("shop product unpublish is unavailable", error),
    }
}

async fn list_current_orders(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<ShopListParams>,
) -> Response {
    let scope = match current_scope(runtime_context).await {
        Ok(v) => v,
        Err(r) => return r,
    };
    match state
        .shop
        .list_shop_orders(
            scope,
            params.page.unwrap_or(1),
            params.page_size.unwrap_or(20),
        )
        .await
    {
        Ok(page) => success_paged_list(page.items, page.page, page.page_size, page.total),
        Err(error) => shop_system_response("shop orders are unavailable", error),
    }
}

async fn retrieve_current_order(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(order_id): Path<String>,
) -> Response {
    let scope = match current_scope(runtime_context).await {
        Ok(v) => v,
        Err(r) => return r,
    };
    match state.shop.retrieve_shop_order(scope, order_id).await {
        Ok(Some(item)) => success_resource(item),
        Ok(None) => not_found_response("shop order was not found"),
        Err(error) => shop_system_response("shop orders are unavailable", error),
    }
}

async fn create_current_order_fulfillment(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(order_id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    current_create_handler(
        state,
        runtime_context,
        body,
        move |store, scope, body| async move {
            store.create_shop_fulfillment(scope, order_id, body).await
        },
    )
    .await
}

async fn list_current_settlements(
    State(state): State<AppShopState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let scope = match current_scope(runtime_context).await {
        Ok(v) => v,
        Err(r) => return r,
    };
    match state.shop.list_settlements(scope).await {
        Ok(page) => success_paged_list(page.items, page.page, page.page_size, page.total),
        Err(error) => shop_system_response("shop settlements are unavailable", error),
    }
}

fn map_shop_summary(value: ShopSummaryView) -> ShopSummaryResponse {
    ShopSummaryResponse {
        id: value.shop_id,
        tenant_id: value.tenant_id,
        organization_id: value.organization_id,
        shop_no: value.shop_no,
        shop_name: value.shop_name,
        shop_type: value.shop_type,
        business_model: value.business_model,
        storefront_status: value.storefront_status,
        operation_status: value.operation_status,
        review_status: value.review_status,
        data_scope: value.data_scope,
        logo_media_resource_id: value.logo_media_resource_id,
        cover_media_resource_id: value.cover_media_resource_id,
        default_currency_code: value.default_currency_code,
        default_locale: value.default_locale,
        timezone: value.timezone,
        version: value.version,
        created_at: value.created_at,
        updated_at: value.updated_at,
    }
}
