pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn unshift(&mut self, v:size) {
        let new_node = Node{data: v, link: self.head.take()};
        self.head = Some(Box::new(new_node));
    }
}