use anyhow::Result;

use rest::server;

#[tokio::main]
async fn main() -> Result<()> {
    server::rest_main(8000).await
}
