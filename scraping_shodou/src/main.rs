use scraper::Selector;
use std::{fs::File, io::Write};
use tokio::time;

#[tokio::main]
async fn main() {
    for title in ["温泉", "書道"] {
        download_images(title).await;
    }
}

async fn download_images(title: &str) {
    let shodou_url = "https://uta.pw/shodou";
    let url = format! (
        "{}/index.php?titles&show&title={}",
        shodou_url,
        urlencoding::encode(title)
    );
    println!("get: {}", url)
    let html = reqwest::get(url).await.unwrap().text().await.unwrap();
    let doc = scraper::Html::parse_document(&html);
    let sel = Selector::parse(".articles img").unwrap();
    for (i, node) in doc.select(&sel).enumerate() {

    }
}