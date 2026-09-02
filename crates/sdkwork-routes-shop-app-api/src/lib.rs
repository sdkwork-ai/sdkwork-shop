pub mod app_routes;
pub mod http_envelope;
pub mod http_route_manifest;
pub mod paths;
pub mod routes;
pub mod subject;
pub mod web_bootstrap;

pub use app_routes::{
    app_shop_router_with_postgres_pool, build_app_shop_router, CommerceShopFuture,
    CommerceShopStore,
};
pub use http_route_manifest::app_route_manifest;
pub use routes::{build_shop_app_router, build_shop_app_router_with_framework};
pub use web_bootstrap::{
    shop_app_api_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

use axum::Router;
use sdkwork_shop_service_host::ShopServiceHost;
use sdkwork_web_core::HttpRouteManifest;
use std::sync::Arc;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    app_route_manifest()
}

pub async fn gateway_mount(host: Arc<ShopServiceHost>) -> Router {
    build_shop_app_router_with_framework(host).await
}

pub fn gateway_mount_business(host: Arc<ShopServiceHost>) -> Router {
    build_shop_app_router(host)
}
