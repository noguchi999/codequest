use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    data: isize,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>
}

pub struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>
}

impl List {
    pub fn new() -> List {
        Self { head: None, tail: None }
    }
}