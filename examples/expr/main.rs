#![allow(dead_code)]

extern crate jacaranda;
use jacaranda::tree::*;
use std::fmt::Write;

mod parser;

// AST tree example
pub enum AstNode {
    Expr, // eg. parenthesised expression, or the root expression
    Operation(Operator),
    Literal(i64),
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn evaluate(tree: &Tree<AstNode>) -> i64 {
    eval_node(&tree, tree.root())
}

fn eval_node(tree: &Tree<AstNode>, node: NodeIndex) -> i64 {
    let node = tree.get(node).unwrap();
    match node.data() {
        AstNode::Literal(val) => *val,
        AstNode::Expr => eval_node(tree, *node.children().first().unwrap()),
        AstNode::Operation(op) => {
            let left = node.children()[0];
            let right = node.children()[1];
            let left = eval_node(tree, left);
            let right = eval_node(tree, right);

            match op {
                Operator::Add => left + right,
                Operator::Sub => left - right,
                Operator::Mul => left * right,
                Operator::Div => left / right,
            }
        }
    }
}

fn display(tree: &Tree<AstNode>) -> String {
    let mut buf = String::new();
    display_node(&tree, tree.root(), &mut buf);
    buf
}

fn display_node(tree: &Tree<AstNode>, node: NodeIndex, buf: &mut String) {
    let node = tree.get(node).unwrap();
    match node.data() {
        AstNode::Literal(val) => {
            write!(buf, "{}", val).unwrap();
        }
        AstNode::Expr => {
            write!(buf, "(").unwrap();
            display_node(tree, *node.children().first().unwrap(), buf);
            write!(buf, ")").unwrap();
        }
        AstNode::Operation(op) => {
            let left = node.children()[0];
            let right = node.children()[1];
            display_node(tree, left, buf);
            write!(
                buf,
                "{}",
                match op {
                    Operator::Add => "+",
                    Operator::Sub => "-",
                    Operator::Mul => "*",
                    Operator::Div => "/",
                }
            )
            .unwrap();
            display_node(tree, right, buf);
        }
    }
}

fn build_sample() -> Option<Tree<AstNode>> {
    let mut tree = Tree::new(AstNode::Expr);
    let plus = tree.add(tree.root(), AstNode::Operation(Operator::Add))?;
    let _l = tree.add(plus, AstNode::Literal(5))?;
    let re = tree.add(plus, AstNode::Expr)?;
    let rd = tree.add(re, AstNode::Operation(Operator::Div))?;
    let _rl = tree.add(rd, AstNode::Literal(7))?;
    let _rr = tree.add(rd, AstNode::Literal(2))?;

    Some(tree)
}

fn main() -> Result<(), parser::ParseError> {
    let t = build_sample().unwrap();

    println!("{} = {}", display(&t), evaluate(&t));
    // use parser::*;
    // let mut state = ParseState {
    //     input: "(1+2)".as_bytes(),
    //     index: 0,
    //     tree: Tree::new(AstNode::Expr),
    // };

    Ok(())
}
