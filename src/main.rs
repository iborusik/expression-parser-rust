use std::fmt::{Display};

use ascii_tree::Tree;
use ascii_tree::write_tree;

use ast::Expression;
use ast::AstParser;
use scanner::ExprScanner;

mod ast;
mod common;
mod scanner;

fn main() {
   
    calc_string("2+3*5");
    
    calc_string("((4+5))*1+7*3+2^3");         
    
    println!("done");
}

fn calc_string(s: &str) -> f64 {
    println!("-------------------------------------------------------------");
    println!("parsing string: {}", s);
    // at first
    // tokenize expressions
    let ep: ExprScanner = ExprScanner{};
    let res = ep.parse(s.to_string());
    
    let mut parser = AstParser::new();
    
    let r = res.and_then(|x| {
        parser.parse_fun(x)
    });
    
    let x = r.unwrap();
    
    let root_node = visit_tree(&x);
    let mut output = String::new();
    let _ = write_tree(&mut output, &root_node);
    println!("{}", output);
    
    println!("-------------------------------------------------------------");
    
    return 0.;  
}

fn visit_tree(exp: &Box<dyn Expression>) -> Tree {
    let left = exp.get_left();
    let right = exp.get_right();
    
    // leaf
    if left.is_none() && right.is_none() {
        return Tree::Leaf(vec![exp.get_desc()]);
    }
    
    if left.is_some() && right.is_some() {
        return Tree::Node(exp.get_desc(), vec![visit_tree(left.unwrap()), visit_tree(right.unwrap())]);
    } else if left.is_some() {
        return Tree::Node(exp.get_desc(), vec![visit_tree(left.unwrap())]);
    }
    
    Tree::Node(exp.get_desc(), vec![visit_tree(right.unwrap())])
}
