use sdkwork_routes_shop_app_api::app_route_manifest;
use sdkwork_web_core::RouteAuth;

fn route_auth(method: &str, path: &str) -> RouteAuth {
    app_route_manifest()
        .match_route(method, path)
        .unwrap_or_else(|| panic!("missing http route manifest for {method} {path}"))
        .auth
}

#[test]
fn storefront_shop_reads_admit_anonymous_visitors() {
    // Browsing the shop directory must not be gated behind a user session:
    // both storefront reads are optional-credential, so an anonymous visitor
    // falls back to the platform public tenant while a signed-in dual-token
    // pair still resolves its own tenant.
    for path in ["/app/v3/api/shops", "/app/v3/api/shops/{shopId}"] {
        assert_eq!(
            route_auth("GET", path),
            RouteAuth::DualTokenOrAnonymous,
            "{path}"
        );
    }
}

#[test]
fn merchant_console_shop_routes_keep_dual_token() {
    // `/app/v3/api/shops/current/**` is the merchant console surface and must
    // keep rejecting anonymous visitors.
    for path in [
        "/app/v3/api/shops/current",
        "/app/v3/api/shops/current/dashboard",
        "/app/v3/api/shops/current/products",
    ] {
        assert_eq!(route_auth("GET", path), RouteAuth::DualToken, "{path}");
    }
}
