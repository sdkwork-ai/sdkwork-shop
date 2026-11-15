use axum::Extension;
use sdkwork_iam_context_service::IamAppContext;

#[derive(Debug, Clone)]
pub(crate) struct AppRuntimeSubject {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    #[allow(dead_code)]
    pub user_id: String,
}

impl AppRuntimeSubject {
    /// Subject for optional-credential storefront reads: the platform public
    /// tenant with no signed-in user.
    fn public_tenant(tenant_id: String) -> Self {
        Self {
            tenant_id,
            organization_id: None,
            user_id: String::new(),
        }
    }
}

pub(crate) fn app_runtime_subject_from_extension(
    context: Option<Extension<IamAppContext>>,
) -> Result<AppRuntimeSubject, String> {
    let Some(Extension(context)) = context else {
        return Err("authenticated runtime context is required".to_owned());
    };
    app_runtime_subject_from_iam(&context)
}

pub(crate) fn app_runtime_subject_from_iam(
    context: &IamAppContext,
) -> Result<AppRuntimeSubject, String> {
    let tenant_id = required_context_text(&context.tenant_id, "tenant_id")?;
    let user_id = required_context_text(&context.user_id, "user_id")?;
    let organization_id = context
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    Ok(AppRuntimeSubject {
        tenant_id,
        organization_id,
        user_id,
    })
}

fn required_context_text(value: &str, field_name: &'static str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!(
            "authenticated runtime context {field_name} is required"
        ));
    }
    Ok(value.to_owned())
}

/// Tenant id used by storefront browse reads before a user signs in (override via env).
fn public_tenant_id() -> String {
    std::env::var("SHOP_PUBLIC_TENANT_ID")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "100001".to_owned())
}

/// Resolves the subject for the optional-credential storefront browse reads.
///
/// `RouteAuth::DualTokenOrAnonymous` routes resolve no principal for an
/// anonymous visitor, so these reads fall back to the platform public tenant
/// instead of failing closed. A signed-in dual-token pair still resolves its own
/// tenant, and every other handler keeps
/// [`app_runtime_subject_from_extension`], which fails closed.
pub(crate) fn storefront_subject_from_extension(
    context: Option<Extension<IamAppContext>>,
) -> Result<AppRuntimeSubject, String> {
    match context {
        Some(Extension(context)) => app_runtime_subject_from_iam(&context),
        None => Ok(AppRuntimeSubject::public_tenant(public_tenant_id())),
    }
}
