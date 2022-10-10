use std::cmp;

pub fn exec() {
    let mut word_1 = String::new();
    std::io::stdin().read_line(&mut word_1).ok();
    let v: Vec<i32> = word_1.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

    let N = v[0]; // 参考書
    let M = v[1]; // 学びたいアルゴリズム
    let X = v[2]; // アルゴリズムの理解度

    let mut C:Vec<i32> = Vec::new();
    let mut A:Vec<Vec<i32>> = Vec::new();
    let mut word_2 = String::new();
    for i in 0..N {
        std::io::stdin().read_line(&mut word_2).ok();
        let CA: Vec<i32> = word_2.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        C.push(CA[0]);
        A.push(CA[1..CA.len()].to_vec());
    }

    let mut ans = i32::MAX;
    for i in 0..(1<<N) {
        let mut cost = 0;
        let mut skill:Vec<i32> = Vec::with_capacity(M as usize);

        for shift in 0..(N as usize) {
            if i>>shift & 1 == 1 {
                cost += C[shift];
                for j in 0..(M as usize) {
                    skill[j] += A[shift][j];
                }
            }
        }

        let min_cost = *skill.iter().min().unwrap();
        if X <= min_cost {
            let v = vec![ans, cost];
            ans = *v.iter().min().unwrap();
        }

        if ans == i32::MAX {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}