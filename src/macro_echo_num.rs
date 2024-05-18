macro_rules! echo_num {
    ($num:expr) => {
        println!("Number is: {}", $num);
    };
}

pub fn main() {
    echo_num!(5);
    echo_num!(15);
    echo_num!(30);
}