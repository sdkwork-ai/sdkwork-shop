use std::sync::Arc;

use sdkwork_database_sqlx::DatabasePool;

use sdkwork_shop_service_host::ShopServiceHost;
use sdkwork_web_bootstrap::{
    ApiAssemblyContribution, CompositeReadinessCheck, DatabasePoolReadinessCheck, WebModule,
};
use sdkwork_web_core::HttpRouteManifest;

use crate::bootstrap::{
    assemble_api_router, assemble_app_api_contribution, assemble_backend_api_contribution,
    ApiAssembly, ApiAssemblyContext,
};

async fn context_from_env() -> Result<(ApiAssemblyContext, DatabasePool), String> {
    let host = Arc::new(ShopServiceHost::from_env().await?);
    let pool = host.database_pool().clone();
    let readiness_check = Arc::new(DatabasePoolReadinessCheck::new(
        host.database_pool().clone(),
    ));
    Ok((
        ApiAssemblyContext {
            host,
            domain_context_injectors: Vec::new(),
            readiness_check,
        },
        pool,
    ))
}

async fn context_from_pool(pool: DatabasePool) -> Result<ApiAssemblyContext, String> {
    let host = Arc::new(ShopServiceHost::from_pool(pool).await?);
    let readiness_check = Arc::new(DatabasePoolReadinessCheck::new(
        host.database_pool().clone(),
    ));
    Ok(ApiAssemblyContext {
        host,
        domain_context_injectors: Vec::new(),
        readiness_check,
    })
}

async fn merge_merchandise_contribution(
    shop: ApiAssembly,
    pool: DatabasePool,
) -> Result<ApiAssembly, String> {
    let merchandise = sdkwork_api_merchandise_assembly::assemble_api_router_with_pool(pool).await?;
    let mut routes = shop.route_manifest.routes().to_vec();
    routes.extend_from_slice(merchandise.route_manifest.routes());
    let mut domain_context_injectors = shop.domain_context_injectors;
    domain_context_injectors.extend(merchandise.domain_context_injectors);

    ApiAssemblyContribution::from_manifest(
        "sdkwork-shop",
        "SDKWork Shop API",
        shop.router.merge(merchandise.router),
        HttpRouteManifest::from_owned_routes(routes),
        domain_context_injectors,
        Arc::new(CompositeReadinessCheck::new(vec![
            shop.readiness_check,
            merchandise.readiness_check,
        ])),
    )
}

/// Assemble the full shop router against a caller-provided database pool so
/// the platform cloud gateway can share its process-wide PostgreSQL pool.
pub async fn assemble_api_router_with_pool(pool: DatabasePool) -> Result<ApiAssembly, String> {
    let shop = assemble_api_router(context_from_pool(pool.clone()).await?).await?;
    merge_merchandise_contribution(shop, pool).await
}

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let (context, pool) = context_from_env().await?;
    let shop = assemble_api_router(context).await?;
    merge_merchandise_contribution(shop, pool).await
}

pub async fn assemble_app_api_contribution_from_env() -> Result<ApiAssembly, String> {
    let (context, _) = context_from_env().await?;
    assemble_app_api_contribution(context).await
}

pub async fn assemble_backend_api_contribution_from_env() -> Result<ApiAssembly, String> {
    let (context, pool) = context_from_env().await?;
    let shop = assemble_backend_api_contribution(context).await?;
    merge_merchandise_contribution(shop, pool).await
}

/// Canonical Web Module definition for this application
/// (API_ASSEMBLY_SPEC §4.1.1): the complete HTTP surface — every route,
/// manifest, and OpenAPI document of this owner — as one installable module.
pub async fn web_module() -> Result<WebModule, String> {
    Ok(WebModule::from_contribution(
        assemble_api_router_from_env().await?,
    ))
}

/// Same as [`web_module`] but composed on a process-shared database pool
/// (platform gateways, API_ASSEMBLY_SPEC §4.1.1).
pub async fn web_module_with_pool(pool: DatabasePool) -> Result<WebModule, String> {
    Ok(WebModule::from_contribution(
        assemble_api_router_with_pool(pool).await?,
    ))
}
