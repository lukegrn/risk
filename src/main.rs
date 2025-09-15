use crate::lang::{ast, token};
use std::io;

pub mod lang;

fn main() {
    println!("Enter lisp: ");
    let mut to_exec = String::new();
    io::stdin()
        .read_line(&mut to_exec)
        .expect("Failed to read line");

    let res: Vec<ast::AstNode>;

    if let Some(cleaned) = to_exec.strip_suffix("\n") {
        res = ast::new(token::tokenize(cleaned));
    } else {
        res = ast::new(token::tokenize(&to_exec));
    }

    dbg!(res);
}
