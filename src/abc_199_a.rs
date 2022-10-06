pub fn exec() {

    let mut word_1 = String::new();
    std::io::stdin().read_line(&mut word_1).ok();
    let answer_1: i32 = word_1.trim().parse().unwrap();
    println!("数字が1つ: {}", answer_1);

    let mut word_2 = String::new();
    std::io::stdin().read_line(&mut word_2).ok();
    let answer_2: Vec<&str> = word_2.trim().split(" ").collect();
    println!("数字が複数: {}", answer_2.join(","));
}