use axum::body;
use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use rust_launchpad_issue_12::router;
use rust_launchpad_issue_12::state::SharedState;
use tower_service::Service;

#[tokio::test]
async fn basic_db_test() {
    let state = SharedState::default();
    let mut app = router(&state);

    let response = app
        .call(
            Request::builder()
                .uri("/kv/test")
                .method("POST")
                .body(Body::from("Hello World"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .call(
            Request::builder()
                .uri("/kv/test")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(&body[..], b"Hello World");
}
