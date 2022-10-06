pub fn exec() {

    let mut word_1 = String::new();
    std::io::stdin().read_line(&mut word_1).ok();
    let answer_1 = word_1.trim().to_string();

    for c in answer_1.chars() {
        println!("{}", c);
    }
}