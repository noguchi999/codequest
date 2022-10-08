pub fn exec() {
    let mut word_1 = String::new();
    std::io::stdin().read_line(&mut word_1).ok();
    let input_vec_1: Vec<&str> = word_1.trim().split(" ").collect();

    let mut word_2 = String::new();
    std::io::stdin().read_line(&mut word_2).ok();
    let p: Vec<f32 > = word_2.trim().split(" ").map(|x| x.parse::<f32 >().unwrap()).collect();

    // p面ダイスの期待値は(p+1)/2
    // X番目～X+K番目の区間和の計算方法
    // ・X=0の場合
    // X+K番目までの累積和
    // ・1<=Xの場合
    // (X+K番目までの累積和) - (X-1番目までの累積和)
    // 累積和の計算量はO(N)
    // 累積和を計算した後の区間和の計算はO(1)

    let N: usize = input_vec_1[0].parse().unwrap();
    let K: usize = input_vec_1[1].parse().unwrap();

    let mut p_ex: Vec<f32> = Vec::new();
    for i in 0..N {
        p_ex.push((p[i] + 1.0) / 2.0);
    }

    let mut p_ex_cum:Vec<f32> = Vec::new();
    p_ex_cum.push(p_ex[0]);
    for i in 1..N {
        p_ex_cum.push(p_ex_cum[i-1] + p_ex[i]);
    }

    let mut ans_tmp = 0.0;
    for i in 0..(N-K+1) {
        if i == 0 {
            ans_tmp = p_ex_cum[i+K-1];
        } else {
            ans_tmp = p_ex_cum[i+K-1] - p_ex_cum[i-1];
        }
    }
    println!("{}", ans_tmp);

}