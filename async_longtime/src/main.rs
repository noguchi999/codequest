use tokio::time;

#[tokio::main]
async fn main() {
    for i in 1..=3 {
        println!("#{}を開始", i);
        let s = read_longtime().await;
        println!("{}", s);
        let s = async {
            time::sleep(time::Duration::from_secs(1)).await;
            String::from("長い読み込み完了(block)")
        }.await;
        println!("{}", s);
    }
}

async fn read_longtime() -> String {
    time::sleep(time::Duration::from_secs(1)).await;
    String::from("読み込み完了(fn)")
}