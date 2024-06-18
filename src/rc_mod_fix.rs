use std::rc::Rc;
use std::cell::RefCell;

pub fn main() {
    let a = Rc::new(RefCell::new(1000));
}