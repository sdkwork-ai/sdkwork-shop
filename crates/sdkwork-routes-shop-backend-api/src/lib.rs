pub mod backend_routes;
pub mod http_envelope;
pub mod http_route_manifest;
pub mod routes;
pub mod subject;
pub mod web_bootstrap;

pub use backend_routes::backend_shop_admin_router_with_postgres_pool;
pub use routes::build_shop_backend_router_with_framework;

use axum::Router;
use sdkwork_shop_service_host::ShopServiceHost;
use sdkwork_web_core::HttpRouteManifest;
use std::sync::Arc;

pub use http_route_manifest::backend_route_manifest;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    backend_route_manifest()
}

pub async fn gateway_mount(host: Arc<ShopServiceHost>) -> Router {
    build_shop_backend_router_with_framework(host).await
}

pub fn gateway_mount_business(host: Arc<ShopServiceHost>) -> Router {
    routes::build_shop_backend_router(host)
}
