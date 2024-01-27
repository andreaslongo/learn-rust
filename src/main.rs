use axum::{response:: IntoResponse, routing::get, Router};

async fn hello_world() -> impl IntoResponse {
    "Hello world"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
