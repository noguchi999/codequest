enum Coin {
    Yen_1(isize),
    Yen_5(isize),
    Yen_10(isize),
    Yen_50(isize),
    Yen_100(isize),
    Yen_500(isize),
}
impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Yen_1(n) => n,
            Coin::Yen_5(n) => n * 5,
            Coin::Yen_10(n) => n * 10,
            Coin::Yen_50(n) => n * 50,
            Coin::Yen_100(n) => n * 100,
            Coin::Yen_500(n) => n * 500,
        }
    }
}

pub fn main() {
    let wallet: Vec<Coin> = vec![
        Coin::Yen_1(1),
        Coin::Yen_5(2),
        Coin::Yen_10(3),
        Coin::Yen_50(4),
        Coin::Yen_100(5),
        Coin::Yen_500(6),
    ];

    let total = wallet.iter().fold(0, |sum, coin| sum + coin.calc_price());
    println!("Total: {}", total);
}