use sdkwork_api_shop_assembly::assemble_api_router_from_env;
use sdkwork_web_bootstrap::{infra_public_path_prefixes, ApiModuleRegistry, ComposedApiAssembly};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting SDKWork Shop API Server...");

    let assembly = assemble_api_router_from_env()
        .await
        .expect("assemble shop API authority");
    let framework = sdkwork_iam_web_adapter::build_web_framework_builder(
        sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env().await,
        assembly.route_manifest.clone(),
        infra_public_path_prefixes(),
    );
    let mut module_registry = ApiModuleRegistry::new();
    module_registry.add_modules(vec![assembly]);
    let app = module_registry
        .try_compose("SDKWork Shop API")
        .expect("compose shop API authority")
        .into_hosted(framework)
        .router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_SHOP_ENVIRONMENT"],
            &["SDKWORK_CORS_ALLOWED_ORIGINS"],
        ));

    let addr = std::env::var("SHOP_API_BIND").unwrap_or_else(|_| "0.0.0.0:18090".to_owned());
    tracing::info!("Shop API server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind shop server");
    axum::serve(listener, app).await.expect("serve shop server");
}
