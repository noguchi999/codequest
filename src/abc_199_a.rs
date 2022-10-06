pub fn exec() {

    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    let answer: i32 = word.trim().parse().unwrap();
    println!("数字が1つ: {}", answer);
}