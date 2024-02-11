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
    db: HashMap<String, StoredType>,
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

use image::ImageError;

impl From<ImageError> for KVError {
    fn from(value: ImageError) -> Self {
        match value {
            ImageError::IoError(err) => Self::new(StatusCode::BAD_REQUEST, &err.to_string()),
            ImageError::Unsupported(err) => Self::new(StatusCode::BAD_REQUEST, &err.to_string()),
            _ => Self::new(StatusCode::BAD_REQUEST, ":-("),
        }
    }
}

pub async fn post_kv(
    Path(key): Path<String>,
    TypedHeader(content_type): TypedHeader<ContentType>,
    State(state): State<SharedState>,
    data: Bytes,
) -> Result<String, KVError> {
    let stored_type = get_stored_type(content_type, data)?;
    state.write().await.db.insert(key, stored_type);
    Ok("OK".to_string())
}

pub async fn get_kv(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    match state.read().await.db.get(&key) {
        Some(StoredType::Image(image)) => Ok(ImageResponse::try_from(image)?.into_response()),
        Some(StoredType::Other(bytes)) => Ok(bytes.clone().into_response()),
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
) -> KVResult<impl IntoResponse> {
    match state.read().await.db.get(&key) {
        Some(StoredType::Image(image)) => ImageResponse::try_from(image.grayscale()),

        _ => Err(KVError::new(StatusCode::NOT_FOUND, "Key not found")),
    }
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

use image::DynamicImage;

impl TryFrom<DynamicImage> for ImageResponse {
    type Error = KVError;

    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        ImageResponse::try_from(&value)
    }
}

impl TryFrom<&DynamicImage> for ImageResponse {
    type Error = KVError;

    fn try_from(value: &DynamicImage) -> Result<Self, Self::Error> {
        let mut vec: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut vec);
        value.write_to(&mut cursor, ImageOutputFormat::Png)?;
        Ok(ImageResponse::new(vec))
    }
}

pub enum StoredType {
    Image(DynamicImage),
    Other(Bytes),
}

fn get_stored_type(content_type: impl ToString, data: Bytes) -> KVResult<StoredType> {
    if content_type.to_string().starts_with("image") {
        let image = image::load_from_memory(&data)?;
        Ok(StoredType::Image(image))
    } else {
        Ok(StoredType::Other(data))
    }
}

type KVResult<T> = Result<T, KVError>;
