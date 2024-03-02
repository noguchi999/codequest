use hyper::Client;
// https://qiita.com/ishishow/items/abacc371e62d2d2368b2

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    Ok(())
}