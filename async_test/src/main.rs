#[tokio::main]
async fn main() {
    let f = say_later("Hello, world!");
    print!("I'm doing something else...");
    f.await;
}

async fn say_later(msg: &'static str) {
    println!("{}", msg);
}