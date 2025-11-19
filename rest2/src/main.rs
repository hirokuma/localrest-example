mod derive_from_request;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {

    // Build our application with some routes
    let app = Router::new()
        .route("/", post(derive_from_request::handler));

    // Run our application
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
