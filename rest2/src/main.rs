use rest2::server;

#[tokio::main]
async fn main() {
    server::start("127.0.0.1:8000".to_string()).await;
}
