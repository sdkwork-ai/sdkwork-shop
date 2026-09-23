use axum::Router;
use sdkwork_shop_service_host::ShopServiceHost;
use std::sync::Arc;

use crate::app_shop_router_with_postgres_pool;
use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

pub fn build_shop_app_router(host: Arc<ShopServiceHost>) -> Router {
    let pool = host
        .database_pool()
        .as_postgres()
        .expect("shop app-api requires an authoritative PostgreSQL pool");
    // The catalog store mints merchandise ids, so it takes the process Snowflake identity rather
    // than opening a second, divergent generator.
    app_shop_router_with_postgres_pool(pool.clone(), host.id_generator())
}

pub async fn build_shop_app_router_with_framework(host: Arc<ShopServiceHost>) -> Router {
    wrap_router_with_web_framework_from_env(build_shop_app_router(host)).await
}
