pub fn main() {
    let banana = ("banana", 300);
    let apple = ("apple", 200);
    let total = banana.1 + apple.1;

    print_tuple(&banana);
    print_tuple(&apple);
    println!("Total: {}", total);
}

fn print_tuple(t: &(&str, i64)) {
    println!("{}: {}", t.0, t.1);
}
