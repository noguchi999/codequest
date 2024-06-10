pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
}