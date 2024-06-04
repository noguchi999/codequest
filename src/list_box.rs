pub struct Node {
    data: i64,
    link: Option<Box<Node>>
}

fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { data: v, link: link }))
}

pub fn main() {
    let c = ndoe(10, node(20, node(30, None))).unwrap();
    let mut p = &c;
    loop {
        println!("{}", p.data);
        match p.link {
            Some(ref link) => p = &link,
            None => break
        }
    }
}