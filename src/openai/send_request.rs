use hyper::{Client, Uri, Body, Method, Request};

// https://qiita.com/ishishow/items/abacc371e62d2d2368b2

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let ip_future = async {
        let mut resp = client.get(Uri::from_static("http://httpbin.org/ip")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let header_future = async {
        let mut resp = client.get(Uri::from_static("http://httpbin.org/headers")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let req = Request::builder().method(Method::POST).uri("http://httpbin.org/post").header("content-type", "application/json").body(Body::from(r#"{"library":"hyper"}"#))?;
    let post_future = async {
        let resp = client.request(req).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let (ip, headers, post_req) = futures::try_join!(ip_future, header_future, post_future)?;

    let ip_body = String::from_utf8(ip.to_vec()).expect("response was not valid utf-8");
    println!("ip: {}", ip_body);

    let headers_body = String::from_utf8(headers.to_vec()).expect("response was not valid utf-8");
    println!("headers: {}", headers_body);

    let post_body = String::from_utf8(post_req.to_vec()).expect("response was not valid utf-8");
    println!("post: {}", post_body);

    Ok(())
}