use tokio::time;

#[tokio::main]
async fn main() {
    for i in 1..=3 {
        println!("#{}を開始", i);
    }
}