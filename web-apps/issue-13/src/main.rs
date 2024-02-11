use rust_launchpad_issue_13::router;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Executor;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://data.db?mode=rwc")
        .await?;
    pool.execute(include_str!("../schema.sql")).await?;

    let router = router(Arc::new(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
