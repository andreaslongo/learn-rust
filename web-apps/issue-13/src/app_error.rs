use crate::article_repository::ArticleRepositoryError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    ArticleRepositoryError(ArticleRepositoryError),
}

impl From<ArticleRepositoryError> for AppError {
    fn from(error: ArticleRepositoryError) -> Self {
        AppError::ArticleRepositoryError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ArticleRepositoryError(ArticleRepositoryError::NotFound) => {
                (StatusCode::NOT_FOUND, "Article not found").into_response()
            }
            AppError::ArticleRepositoryError(ArticleRepositoryError::Other(err)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
            }
        }
    }
}
