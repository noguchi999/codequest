use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    data: isize,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>
}

pub struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>
}

impl List {
    pub fn new() -> List {
        Self { head: None, tail: None }
    }
    fn new_node(v:isize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data: v,
            next: None,
            prev: None
        }))
    }

    pub fn push(&mut self, v:isize) {
        let n = List::new_node(v);
        match self.tail.take() {
            None => {
                self.tail = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_tail) => {
                self.tail = Some(Rc::clone(&n));
                n.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(n);
            }
        }
    }

    pub fn unshift(&mut self, v:isize) {
        let n = List::new_node(v);
        match self.head.take() {
            None => {
                self.tail = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&old_head));
                n.borrow_mut().next = Some(old_head);
                self.head = Some(n);
            }
        }
    }

    pub fn iter(&mut self) -> ListIter {
        match & self.head {
            None => ListIter{cur:None},
            Some(head) => {
                let head = Rc::clone(&head);
                ListIter{cur: Some(head)}
            }
        }
    }
}

pub struct ListIter {
    pub cur: Option<Rc<RefCell<Node>>>
}

impl Iterator for ListIter {
    type Item = isize;
    fn next(&mut self) -> Option<isize> {
        match self.cur.take() {
            None => None,
            Some(cur) => {
                let cb = cur.borrow();
                let data = cb.data;
                match &cb.next {
                    None => self.cur = None,
                    Some(next) => self.cur = Some(Rc::clone(next))
                }
                Some(data)
            }
        }
    }
}

pub fn main() {
    let mut list = List::new();
    list.push(100);
    list.push(110);
    list.unshift(10);
    list.unshift(20);
    for v in list.iter() {
        println!("{}", v);
    }
}