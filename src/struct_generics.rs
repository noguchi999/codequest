#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

pub fn main() {
    let pt_i = Point { x: 10, y: 20 };
    let pt_f = Point { x: 10.0, y: 20.0 };
    println!("{:?}", pt_i);
    println!("{:?}", pt_f);
}