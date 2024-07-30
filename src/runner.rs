use std::collections::HashMap;
use crate::parser::tomato;
use crate::node::Node;

struct Context {
    vars: HashMap<String, i64>,
}