use hyper_tls::HttpsConnector;
use hyper::Client;

// https://qiita.com/ishishow/items/abacc371e62d2d2368b2

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://hyper.rs".parse()?).await?;
    assert_eq!(res.status(), 200);

    Ok(())
}