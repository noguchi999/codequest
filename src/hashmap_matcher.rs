use std::collections::HashMap;

pub fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    match map.get("D") {
        Some(v) => println!("D={}", v),
        None => println!("D is not found"),
    }
}