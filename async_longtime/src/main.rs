use tokio::time;

#[tokio::main]
async fn main() {
    for i in 1..=3 {
        println!("#{}を開始", i);
    }
}

async fn read_longtime() -> String {
    time::sleep(time::Duration::from_secs(1)).await;
    String::from("読み込み完了(fn)")
}