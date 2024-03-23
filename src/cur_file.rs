use std::fs;

pub fn main() {
    let files = fs::read_dir(".").expect("read_dir call failed");
    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();
        let fname = path.to_str().unwrap_or("unknown");
        println!("{}", fname);
    }
}
