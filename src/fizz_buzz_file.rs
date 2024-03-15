use std::fs::File;
use std::io::{Write, BufWriter};

pub fn main() {
    let filename = "fizz_buzz.txt";
    {
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);
        for i in 1..=100 {
            let mut line = format!("{}\n", i);
            if (i%3 == 0) && (i%5 == 0) {
                line = String::from("FizzBuzz\n");
            } else if i%3 == 0 {
                line = String::from("Fizz\n");
            } else if i%5 == 0 {
                line = String::from("Buzz\n");
            }

            let bytes = line.as_bytes();
            writer.write(bytes).unwrap();
        }
    }

    let s = fs::read_to_string(filename).unwrap();
    println!("{}", s);
}