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

    pub fn push(&mut self, v:isize) {
        let n = List::new_node(v);
        match self.foot.take() {
            None => {
                self.foot = Some(Rc::downgrade(&n));
                self.head = Some(n);
            },
            Some(old_foot) => {
                self.foot = Some(Rc::clone(&n));
                n.borrow_mut().prev = Some(Rc::downgrade(&old_foot));
                old_foot.borrow_mut().next = Some(n);
            }
        }
    }

    pub fn unshift(&mut self, v:isize) {
        let n = List::new_node(v);
        match self.head_take() {
            None => {
                self.foot = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&old_head));
                n.borrow_mut().next = Some(old_head);
                self.head = Some(n);
            }
        }
    }
}