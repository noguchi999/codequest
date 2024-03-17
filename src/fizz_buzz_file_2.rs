use std::fs::File;
use std::io::Write;

pub fn main() {
    let filename = "fizz_buzz.txt";
    let data = get_fizzbuzz(100);
    let mut fp = File::create(filename).unwrap();
    let bytes = data.as_bytes();
    fp.write_all(bytes).unwrap();
}

fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();
    for i in 1..=max {
        if (i % 15 == 0) {
            result.push_str("FizzBuzz\n");
        } else if (i % 3 == 0) {
            result.push_str("Fizz\n");
        } else if (i % 5 == 0) {
            result.push_str("Buzz\n");
        } else {
            result.push_str(&format!("{}\n", i));
        }
    }
    result
}