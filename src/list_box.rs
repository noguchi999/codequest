pub struct Node {
    data: i64,
    link: Option<Box<Node>>
}

fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { data: v, link: link }))
}