use std::rc::Rc;

pub fn main() {
    let a_rc = Rc::new(1000);
    {
        let b_rc = Rc::clone(&a_rc);
        println!("{}", b_rc);
        println!("参照数={}", Rc::strong_count(&a_rc));
    }
    println!("{}", a_rc);
    println!("参照数={}", Rc::strong_count(&a_rc));
}