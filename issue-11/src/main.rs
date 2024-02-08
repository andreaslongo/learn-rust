use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::{headers::ContentType, TypedHeader};
use bytes::Bytes;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    db: HashMap<String, (String, Bytes)>,
}

#[derive(Debug)]
pub struct KVError(StatusCode, String);

impl KVError {
    fn new(status_code: StatusCode, msg: &str) -> Self {
        Self(status_code, msg.to_string())
    }
}

impl std::error::Error for KVError {}

impl Display for KVError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "KVError: {} {}", self.0, self.1)
    }
}

impl IntoResponse for KVError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

pub async fn post_kv(
    Path(key): Path<String>,
    TypedHeader(content_type): TypedHeader<ContentType>,
    State(state): State<SharedState>,
    data: Bytes,
) -> Result<String, KVError> {
    state
        .write()
        .await
        .db
        .insert(key, (content_type.to_string(), data));
    Ok("OK".to_string())
}

pub async fn get_kv(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, KVError> {
    match state.read().await.db.get(&key) {
        Some((content_type, data)) => Ok(([("content-type", content_type.clone())], data.clone())),
        None => Err(KVError::new(StatusCode::NOT_FOUND, "Key not found")),
    }
}

#[tokio::main]
async fn main() {
    let router = router(&SharedState::default());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

pub fn router(state: &SharedState) -> Router {
    Router::new()
        .route("/kv/:key", get(get_kv).post(post_kv))
        .route("/kv/:key/grayscale", get(grayscale))
        .with_state(Arc::clone(state))
}

use image::ImageOutputFormat;
use std::io::Cursor;

pub async fn grayscale(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, KVError> {
    let image = match state.read().await.db.get(&key) {
        Some((content_type, data)) => {
            if content_type == "image/png" {
                image::load_from_memory(data).unwrap()
            } else {
                return Err(KVError::new(
                    StatusCode::FORBIDDEN,
                    "Not possible to grayscale this type of image",
                ));
            }
        }
        None => return Err(KVError::new(StatusCode::NOT_FOUND, "Key not found")),
    };

    let mut vec: Vec<u8> = Vec::new();

    let mut cursor = Cursor::new(&mut vec);
    image
        .grayscale()
        .write_to(&mut cursor, ImageOutputFormat::Png)
        .unwrap();
    let bytes: Bytes = vec.into();

    Ok(ImageResponse::new(bytes))
}

pub struct ImageResponse(Bytes);

impl ImageResponse {
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Self(bytes.into())
    }
}

impl IntoResponse for ImageResponse {
    fn into_response(self) -> axum::response::Response {
        ([("content-type", "image/png")], self.0).into_response()
    }
}
