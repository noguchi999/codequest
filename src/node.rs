#[derive(Debug, Clone)]
pub enum Node {
    Nop,
    Number(i64),
    Calc(char, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Vec<Node>>, Box<Vec<Node>>),
    For(String, i64, i64, Box<Vec<Node>>),
    Print(Box<Node>),
    PringStr(String),
    SetVar(String, Box<Node>),
    GetVar(String),
}