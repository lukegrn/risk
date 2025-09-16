use crate::lang::{ast, exec, token};
use std::io;

pub mod lang;

fn main() {
    loop {
        println!("Enter lisp: ");
        let mut to_exec = String::new();
        io::stdin()
            .read_line(&mut to_exec)
            .expect("Failed to read line");

        let res: Option<exec::Result>;

        if let Some(cleaned) = to_exec.strip_suffix("\n") {
            res = exec::exec(ast::new(token::tokenize(cleaned)));
        } else {
            res = exec::exec(ast::new(token::tokenize(&to_exec)));
        }

        println!("{}", res.unwrap());
    }
}
