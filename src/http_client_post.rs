use hyper::{Client, Body, Method, Request};
// https://qiita.com/ishishow/items/abacc371e62d2d2368b2

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    let client = Client::new();
    let resp = client.request(req).await?;
    println!("Response {}", resp.status());

    Ok(())
}