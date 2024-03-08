use std::collections::HashMap;

pub fn main() {
    let tuki = ["睦月", "如月", "弥生", "卯月", "皐月", "水無月", "文月", "葉月", "長月", "神無月", "霜月", "師走"];
    let mut tuki_map: HashMap<&str, usize> = HashMap::new();

    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, i+1);
    }

    println!("水無月 = {}月", tuki_map["水無月"]);
    println!("神無月 = {}月", tuki_map["神無月"]);
    println!("師走 = {}月", tuki_map["師走"]);
}