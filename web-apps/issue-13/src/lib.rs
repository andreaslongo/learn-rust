pub mod app_error;
pub mod article;
pub mod article_error;
pub mod article_repository;
pub mod article_routes;

use crate::app_error::AppError;
use crate::article::Article;
use crate::article_repository::ArticleRepository;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use sqlx::types::Uuid;
use std::sync::Arc;

pub fn router<T: ArticleRepository>(repository: Arc<T>) -> Router {
    Router::new()
        .route("/articles/:id", get(get_article))
        .route("/articles", post(create_article))
        .with_state(repository)
}

async fn get_article<T: ArticleRepository>(
    State(repository): State<Arc<T>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Article>, AppError> {
    let article = repository.get_article(id).await?;
    Ok(Json(article))
}

async fn create_article<T: ArticleRepository>(
    State(repository): State<Arc<T>>,
    Json(article): Json<Article>,
) -> Result<Json<Uuid>, AppError> {
    let id = repository.create_article(article).await?;
    Ok(Json(id))
}
