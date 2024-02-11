use async_trait::async_trait;
use axum::extract::FromRequest;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Json;
use axum::Router;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use validator::Validate;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/user", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn create_user(ValidatedJson(user): ValidatedJson<User>) -> String {
    format!("User created: {:?}. Status: {:?}", user, user.status)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct User {
    name: String,
    #[validate(email)]
    email: String,
    #[validate(range(min = 18, max = 100))]
    age: u8,
    #[serde(skip)]
    status: UserStatus,
}

#[derive(Debug, Default, Serialize, Deserialize)]
enum UserStatus {
    Active,
    #[default]
    Inactive,
}

#[derive(Debug)]
struct ValidationError;

// TODO
// This Error is very basic and doesn't contain any detailed info. Maybe you want to change it? Think about which information we need to store, and which situations can go wrong. Maybe change the struct to an enum and add some variants. It's up to you!
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Server error".fmt(f)
    }
}

impl Error for ValidationError {}

impl IntoResponse for ValidationError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, "Error with format").into_response()
    }
}

struct ValidatedJson<T>(T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // TODO:
        // We need to convert those errors to our own error type. There's a more elegant way of doing that by implementing the From trait for conversions. Try that yourself! Maybe this is also the part where you introduce more information on what goes wrong.
        let Json(json) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| ValidationError)?;
        json.validate().map_err(|_| ValidationError)?;
        Ok(ValidatedJson(json))
    }
}
