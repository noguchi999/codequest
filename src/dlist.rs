use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    data: isize,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>
}