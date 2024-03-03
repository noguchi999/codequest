use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

// https://qiita.com/ishishow/items/abacc371e62d2d2368b2

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse()?;
    let mut resp = client.get(uri).await?;

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    println!("Response: {}", resp.status());

    Ok(())
}