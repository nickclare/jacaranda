#![allow(dead_code)]

extern crate jacaranda;
use jacaranda::tree::*;
use std::fmt::Write;

// AST tree example
enum AstNode {
    ExprNode, // eg. parenthesised expression, or the root expression
    OperationNode(Operator),
    LiteralNode(i64),
}

enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

fn evaluate(tree: &Tree<AstNode>) -> i64 {
    eval_node(&tree, tree.root())
}

fn eval_node(tree: &Tree<AstNode>, node: NodeIndex) -> i64 {
    let node = tree.get(node).unwrap();
    match node.data() {
        AstNode::LiteralNode(val) => *val,
        AstNode::ExprNode => eval_node(tree, *node.children().first().unwrap()),
        AstNode::OperationNode(op) => {
            let left = node.children()[0];
            let right = node.children()[1];
            let left = eval_node(tree, left);
            let right = eval_node(tree, right);

            match op {
                Operator::Plus => left + right,
                Operator::Minus => left - right,
                Operator::Times => left * right,
                Operator::Divide => left / right,
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
        AstNode::LiteralNode(val) => {
            write!(buf, "{}", val).unwrap();
        }
        AstNode::ExprNode => {
            write!(buf, "(").unwrap();
            display_node(tree, *node.children().first().unwrap(), buf);
            write!(buf, ")").unwrap();
        }
        AstNode::OperationNode(op) => {
            let left = node.children()[0];
            let right = node.children()[1];
            display_node(tree, left, buf);
            write!(
                buf,
                "{}",
                match op {
                    Operator::Plus => "+",
                    Operator::Minus => "-",
                    Operator::Times => "*",
                    Operator::Divide => "/",
                }
            )
            .unwrap();
            display_node(tree, right, buf);
        }
    }
}

fn parse_expr(_text: &str) -> Tree<AstNode> {
    todo!("Implement basic parser")
}

fn build_sample() -> Option<Tree<AstNode>> {
    let mut tree = Tree::new(AstNode::ExprNode);
    let plus = tree.add(tree.root(), AstNode::OperationNode(Operator::Plus))?;
    let _l = tree.add(plus, AstNode::LiteralNode(5))?;
    let re = tree.add(plus, AstNode::ExprNode)?;
    let rd = tree.add(re, AstNode::OperationNode(Operator::Divide))?;
    let _rl = tree.add(rd, AstNode::LiteralNode(7))?;
    let _rr = tree.add(rd, AstNode::LiteralNode(2))?;

    Some(tree)
}

fn main() {
    let t = build_sample().unwrap();

    println!("{} = {}", display(&t), evaluate(&t));
}
