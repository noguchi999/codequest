macro_rules! map_init {
    ($($key:expr => $value:expr), *) => {{
        let mut tmp = std::collections::HashMap::new();
        $(
            tmp.insert($key, $value);
        ) *
        tmp
    }};
}

pub fn main() {
    let week = map_init! [
        "Monday" => 1,
        "Tuesday" => 2,
        "Wednesday" => 3,
        "Thursday" => 4,
        "Friday" => 5,
        "Saturday" => 6,
        "Sunday" => 7
    ];
    println!("Monday: {}", week["Monday"]);
}