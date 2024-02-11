use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use sqlx::{Executor, FromRow, QueryBuilder, Sqlite};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://data.db?mode=rwc")
        .await?;
    pool.execute(include_str!("../schema.sql")).await?;

    let router = Router::new()
        .route("/articles", post(create_article))
        .route("/articles/:id", get(get_article))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn create_article(
    State(pool): State<SqlitePool>,
    Json(new_article): Json<Article>,
) -> impl IntoResponse {
    sqlx::query(&new_article.insert())
        .execute(&pool)
        .await
        .map(|_| (StatusCode::OK, "Article created".to_string()))
        .map_err(internal_server_error)
}

async fn get_article(
    Path(article_id): Path<usize>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Article>, (StatusCode, String)> {
    sqlx::query_as(&Article::select(article_id))
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(not_found)
}

fn internal_server_error(e: sqlx::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Error creating article: {}", e),
    )
}

fn not_found(e: sqlx::Error) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("Article with id {} not found", e),
    )
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct Article {
    title: String,
    content: String,
    published_date: String,
}

impl SQLStatements<usize> for Article {
    fn insert(&self) -> String {
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("INSERT INTO articles (title, content, published_date)");

        query_builder.push_values([self], |mut b, article| {
            b.push_bind(article.title.clone())
                .push_bind(article.content.clone())
                .push_bind(article.published_date.clone());
        });
        query_builder.into_sql()
    }

    fn select(key: usize) -> String {
        format!(
            "SELECT title, content, published_date FROM articles WHERE id = {}",
            key
        )
    }
}

trait SQLStatements<T> {
    fn insert(&self) -> String;
    fn select(key: T) -> String;
}
