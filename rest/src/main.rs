use anyhow::Result;

use rest::server;

#[tokio::main]
async fn main() -> Result<()> {
    // 特に意味は無いが2つ立ち上げる
    tokio::spawn(server::rest_main(8000));
    server::rest_main(8001).await
}
