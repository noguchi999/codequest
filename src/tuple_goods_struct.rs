struct Item(String, i64);

pub fn main() {
    let banana = Item("banana".to_string(), 300);
    let apple = Item("apple".to_string(), 200);
    let items = vec![banana, apple];
    let total = sum(&items);

    println!("Total: {}", total);
}

fn sum(items: &Vec<Item>) -> i64 {
    items.iter().map(|item| item.1).sum()
}