use std::collections::HashMap;

pub fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    if map.get("D") == None {
        println!("D is not found");
    } else {
        println!("D={}", map["D"]);
    }
}