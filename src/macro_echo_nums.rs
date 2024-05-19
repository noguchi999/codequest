#[macro_export]
macro_rules! echo_nums {
    ($($num:expr),*) => {
        $(
            println!("{}", $num);
        )*
        println!("");
    };
}

pub fn main() {
    echo_nums![1, 2, 3, 4, 5];
    echo_nums!(10, 20, 30, 40, 50);
}