use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/:name", get(hello))
        .nest_service("/static", ServeDir::new("static/"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    HtmlTemplate(HelloTemplate { name })
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}

// "Newtype" pattern to work around Rust's orphan rule
struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
