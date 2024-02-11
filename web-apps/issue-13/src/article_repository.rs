use crate::article::Article;
use axum::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait ArticleRepository: Send + Sync + 'static {
    async fn get_article(&self, id: Uuid) -> Result<Article, ArticleRepositoryError>;
    async fn create_article(&self, article: Article) -> Result<Uuid, ArticleRepositoryError>;
}

pub enum ArticleRepositoryError {
    NotFound,
    Other(String),
}

#[async_trait]
impl ArticleRepository for SqlitePool {
    async fn get_article(&self, id: Uuid) -> Result<Article, ArticleRepositoryError> {
        let query = format!(
            r#"
            SELECT title, content, published_date
            FROM articles
            WHERE id = '{}'
            "#,
            id
        );

        let result = sqlx::query_as(&query);
        let article = result
            .fetch_one(self)
            .await
            .map_err(|_| ArticleRepositoryError::NotFound)?;

        Ok(article)
    }

    async fn create_article(&self, article: Article) -> Result<Uuid, ArticleRepositoryError> {
        let query = format!(
            r#"
            INSERT INTO articles (title, content, published_date)
            VALUES ('{}', '{}', '{}')
            RETURNING id
            "#,
            article.title, article.content, article.published_date
        );

        let result = sqlx::query_scalar(&query);
        let id: sqlx::types::Uuid = result
            .fetch_one(self)
            .await
            .map_err(|e| ArticleRepositoryError::Other(e.to_string()))?;

        Ok(id)
    }
}
