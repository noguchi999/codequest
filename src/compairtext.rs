use std::fs;

pub fn main() {
    let afile = "./inputs/atext.txt";
    let bfile = "./inputs/btext.txt";

    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();

    let astr = astr.trim();
    let bstr = bstr.trim();

    if astr == bstr {
        println!("ok");
    } else {
        println!("ng");
    }
}