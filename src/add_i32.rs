fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

pub fn main() {
    println!("add_i32(1, 2) = {}", add_i32(10, 25));
    println!("add_f32(1, 2) = {}", add_f32(10.0, 25.0));
}