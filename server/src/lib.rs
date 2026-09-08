use axum::{Router, routing::get};
use dill::CatalogBuilder;

#[derive(Debug)]
pub struct IdentityService;

pub fn register(builder: &mut CatalogBuilder) {
    builder.add_value(IdentityService);
}

pub fn router(_catalog: &dill::Catalog) -> anyhow::Result<Router> {
    Ok(Router::new().route("/api/plugins/identity/health", get(|| async { "ok" })))
}
