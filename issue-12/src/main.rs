use rust_launchpad_issue_12::router;
use rust_launchpad_issue_12::state::SharedState;
use tokio::net::TcpListener;

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let state = SharedState::default();
    let app = router(&state);

    axum::serve(listener, app).await?;
    Ok(())
}
