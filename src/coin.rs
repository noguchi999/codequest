enum Coin {
    Yen1(isize),
    Yen5(isize),
    Yen10(isize),
    Yen50(isize),
    Yen100(isize),
    Yen500(isize),
}
impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Yen1(n) => n,
            Coin::Yen5(n) => n * 5,
            Coin::Yen10(n) => n * 10,
            Coin::Yen50(n) => n * 50,
            Coin::Yen100(n) => n * 100,
            Coin::Yen500(n) => n * 500,
        }
    }
}

pub fn main() {
    let wallet: Vec<Coin> = vec![
        Coin::Yen1(1),
        Coin::Yen5(2),
        Coin::Yen10(3),
        Coin::Yen50(4),
        Coin::Yen100(5),
        Coin::Yen500(6),
    ];

    let total = wallet.iter().fold(0, |sum, coin| sum + coin.calc_price());
    println!("Total: {}", total);
}