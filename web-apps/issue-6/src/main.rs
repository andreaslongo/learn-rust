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
    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO articles (title, content, published_date)");

    query_builder.push_values([new_article], |mut b, article| {
        b.push_bind(article.title)
            .push_bind(article.content)
            .push_bind(article.published_date);
    });

    let result = query_builder.build().execute(&pool).await;

    match result {
        Ok(_) => (StatusCode::OK, "Article created".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating article: {}", e),
        ),
    }
}

async fn get_article(
    State(pool): State<SqlitePool>,
    Path(article_id): Path<usize>,
) -> Result<Json<Article>, (StatusCode, String)> {
    // We don't need to fear SQL injection here since the article ID is an integer and not a string.
    let query = format!(
        "SELECT title, content, published_date FROM articles WHERE id = {}",
        article_id
    );
    let result = sqlx::query_as(&query);

    let article: Article = result.fetch_one(&pool).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            format!("Article with id {} not found", article_id),
        )
    })?;

    Ok(Json(article))
}

#[derive(Deserialize, Serialize, FromRow)]
struct Article {
    title: String,
    content: String,
    published_date: String,
}
