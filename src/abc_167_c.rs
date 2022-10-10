pub fn exec() {
    let mut word_1 = String::new();
    std::io::stdin().read_line(&mut word_1).ok();
    let v: Vec<i32> = word_1.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

    let N = v[0]; // 参考書
    let M = v[1]; // 学びたいアルゴリズム
    let X = v[2]; // アルゴリズムの理解度

    let mut C:Vec<i32> = Vec::new();
    let mut A:Vec<i32> = Vec::new();
    let mut word_2 = String::new();
    for i in 0..N {
        std::io::stdin().read_line(&mut word_2).ok();
        let CA: Vec<i32> = word_2.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        C.push(CA[0]);
        A = CA[1..CA.len()].to_vec();
    }

    let mut ans = i32::MAX;
    for i in 0..(1<<N) {

    }
}