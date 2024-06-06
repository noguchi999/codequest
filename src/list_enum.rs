enum Node {
    Empty,
    Cons(i64, Box<Node>)
}

use Node::{Empty, Cons};
fn node(v: i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}