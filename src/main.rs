use crate::lang::{
    ast,
    exec::{self},
    scope::Scope,
    token,
    types::result::Result,
};
use std::io;

pub mod lang;

fn main() {
    let mut persistent_state = Scope::base();

    loop {
        println!("Enter lisp: ");
        let mut to_exec = String::new();
        io::stdin()
            .read_line(&mut to_exec)
            .expect("Failed to read line");

        let res: Option<Result>;

        if let Some(cleaned) = to_exec.strip_suffix("\n") {
            res = exec::exec(
                ast::new(token::tokenize(cleaned)),
                Some(&mut persistent_state),
            );
        } else {
            res = exec::exec(
                ast::new(token::tokenize(&to_exec)),
                Some(&mut persistent_state),
            );
        }

        match res {
            Some(r) => println!("{}", r),
            None => println!(""),
        }
    }
}
