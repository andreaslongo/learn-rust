use crate::SharedState;
use axum::body::Bytes;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_macros::debug_handler;

#[derive(Debug)]
pub struct CustomError(StatusCode, String);

impl CustomError {
    fn new(status_code: StatusCode, msg: &str) -> Self {
        Self(status_code, msg.to_string())
    }
}

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError: {} {}", self.0, self.1)
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

#[debug_handler]
pub async fn kv_set(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    bytes: Bytes,
) -> Result<(), CustomError> {
    state.write().await.db.insert(key, bytes);
    Ok(())
}

#[debug_handler]
pub async fn kv_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Bytes, CustomError> {
    let db = &state.read().await.db;
    if let Some(val) = db.get(&key) {
        Ok(val.to_owned())
    } else {
        Err(CustomError::new(StatusCode::NOT_FOUND, "Key not found"))
    }
}
