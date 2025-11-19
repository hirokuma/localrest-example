use rest2::server;

use axum::{Router, routing::post};

#[tokio::main]
// #[axum::debug_handler]
async fn main() {

    // Build our application with some routes
    let app = Router::new()
        .route("/", post(server::AppState::handler))
        .with_state(server::AppState::new());

    // Run our application
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
