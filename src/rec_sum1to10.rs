fn sum(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n + sum(n - 1)
    }
}

pub fn main() {
    println!("{}", sum(10));
}