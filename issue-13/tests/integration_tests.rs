use axum::async_trait;
use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use rust_launchpad_issue_13::article::Article;
use rust_launchpad_issue_13::article_repository::ArticleRepository;
use rust_launchpad_issue_13::article_repository::ArticleRepositoryError;
use rust_launchpad_issue_13::router;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_service::Service;
use uuid::Uuid;

#[derive(Default)]
pub struct InMemoryArticleRepository(RwLock<HashMap<Uuid, Article>>);

#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn get_article(&self, id: Uuid) -> Result<Article, ArticleRepositoryError> {
        self.0
            .read()
            .await
            .get(&id)
            .map(ToOwned::to_owned)
            .ok_or(ArticleRepositoryError::NotFound)
    }

    async fn create_article(&self, article: Article) -> Result<Uuid, ArticleRepositoryError> {
        let id = Uuid::new_v4();
        self.0.write().await.insert(id, article);
        Ok(id)
    }
}

#[tokio::test]
async fn create_article() {
    let repository = InMemoryArticleRepository::default();
    let mut router = router(Arc::new(repository));

    let request = Request::builder()
        .method("POST")
        .uri("/articles")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "title": "Hello",
                "content": "World",
                "published_date": "2024-01-01"
            }"#,
        ))
        .unwrap();

    let response = router.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let id: Uuid = serde_json::from_slice(&body).unwrap();

    assert!(id != Uuid::nil());
}
