use tokio::time;

async fn say_later(sec: u64, msg: &str) {
    time::sleep(time::Duration::from_secs(sec)).await;
    println!("{}: {}", sec, msg);
}

#[tokio::main]
async fn main() {
    tokio::spawn(say_later(3, "毎日がエブリィデイ"));
}