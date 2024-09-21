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

}