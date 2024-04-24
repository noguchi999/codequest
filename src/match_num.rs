pub fn main() {
    let i = 2u8;
    match i {
        0 => println!("Zero"),
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("..."),
    }
}