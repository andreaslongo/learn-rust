mod kv_store;
pub mod state;
use axum::routing::get;
use axum::Router;
use kv_store::kv_get;
use kv_store::kv_set;
use state::SharedState;
use std::sync::Arc;

pub fn router(state: &SharedState) -> Router {
    Router::new()
        .route("/kv/:key", get(kv_get).post(kv_set))
        .with_state(Arc::clone(state))
}
