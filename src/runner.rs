use std::collections::HashMap;
use crate::parser::tomato;
use crate::node::Node;

struct Context {
    vars: HashMap<String, i64>,
}

fn run_node(ctx: &mut Context, node: Node) -> i64 {
    match node {
        Node::Number(v) => v,
        Node::Calc(op, l, r) => {
            calc_op(op, run_node(ctx, *l), run_node(ctx, *r))
        },
        Node::GetVar(name) => {
            match ctx.vars.get(&name) {
                Some(v) => *v,
                None => 0,
            }
        },
        Node::If(cond, true_n, false_n) => {
            let cond_v = run_node(ctx, *cond);
            if cond_v > 0 {
                run_node(ctx, *true_n)
            } else {
                run_node(ctx, *false_n)
            }
        },
    }
}