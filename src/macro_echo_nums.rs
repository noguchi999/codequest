#[macro_export]
macro_rules! echo_nums {
    ($($num:expr),*) => {
        $(
            println!("{}", $num);
        )*
        println!("");
    };
}