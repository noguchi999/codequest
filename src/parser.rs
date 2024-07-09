use peg;
use crate::node::Node;
peg::parser! (pub grammar tomat() for str {
    pub rule parse() -> Vec<Node> = v:sentences()
    rule sentences() -> Vec<Node> = sentence() ** end_of_line()
    rule sentence() -> Node = print() / if() / for() / let() / _ {Node::Nop}
    rule print() -> Node
        = "print" _ "\"" v:$([^ '"']*) "\""
        {Node::PrintStr(v.to_string())}
        / "print" _ v:calc()
        {Node::Print(Box::new(v))}
    rule if() -> Node = "if" _ v:if_cond() {v}
})