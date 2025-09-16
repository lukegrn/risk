use crate::lang::base;
use std::{collections::HashMap, fmt::Display};

use crate::lang::ast::AstNode;

#[derive(PartialEq, Debug, Clone)]
pub enum Result {
    S(String),
    I(i32),
    F(f64),
    B(bool),
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::S(s) => write!(f, "{}", s),
            Result::I(i) => write!(f, "{}", i),
            Result::F(fl) => write!(f, "{}", fl),
            Result::B(b) => match b {
                true => write!(f, "#t"),
                false => write!(f, "#f"),
            },
        }
    }
}

enum Scopable {
    Result(Result),
    Callback(fn(args: Vec<AstNode>, scope: &Scope) -> Option<Result>),
}

pub struct Program<'a> {
    scope: &'a Scope,
    ast: Vec<AstNode>,
}

pub struct Scope {
    map: HashMap<String, Scopable>,
}

impl Program<'_> {
    pub fn exec(&self) -> Option<Result> {
        match &self.ast.clone()[..] {
            [] => None,
            [a] => match a {
                AstNode::Leaf(l) => match l.parse::<i32>() {
                    Ok(i) => Some(Result::I(i)),
                    Err(_) => match l.parse::<f64>() {
                        Ok(f) => Some(Result::F(f)),
                        Err(_) => match self.scope.map.get(l) {
                            Some(v) => match v {
                                Scopable::Result(r) => Some(r.to_owned()),
                                Scopable::Callback(c) => c(vec![], &self.scope),
                            },
                            None => panic!("Reference to undefined variable {}", l),
                        },
                    },
                },
                AstNode::AST(ast_nodes) => Program::new(ast_nodes.to_vec(), &self.scope).exec(),
            },
            [f, rest @ ..] => match f {
                AstNode::Leaf(f) => match self.scope.map.get(f) {
                    Some(s) => match s {
                        Scopable::Result(r) => panic!("Call to primitive {} as function", r),
                        Scopable::Callback(c) => c(rest.to_vec(), self.scope),
                    },
                    None => panic!("Call to undefined function <{}>", f),
                },
                AstNode::AST(ast_nodes) => {
                    match Program::new(ast_nodes.to_vec(), self.scope).exec() {
                        Some(r) => match r {
                            Result::S(s) => match self.scope.map.get(&s) {
                                Some(v) => match v {
                                    Scopable::Result(r) => Some(r.to_owned()),
                                    Scopable::Callback(c) => c(vec![], &self.scope),
                                },
                                None => panic!("Reference to undefined variable {}", s),
                            },

                            p => panic!("Call to primitive {} as function", p),
                        },
                        None => panic!("Call to '()"),
                    }
                }
            },
        }
    }

    pub fn new(ast: Vec<AstNode>, scope: &Scope) -> Program<'_> {
        Program {
            scope: scope,
            ast: ast,
        }
    }
}

impl Scope {
    pub fn base() -> Scope {
        let mut base_scope: Scope = Scope {
            map: HashMap::new(),
        };

        // bools
        base_scope
            .map
            .insert(String::from("#t"), Scopable::Result(Result::B(true)));

        base_scope
            .map
            .insert(String::from("#f"), Scopable::Result(Result::B(false)));

        // logic functions
        base_scope
            .map
            .insert(String::from("if"), Scopable::Callback(base::logic::ifdef));

        base_scope.map.insert(
            String::from("eq?"),
            Scopable::Callback(base::logic::eqhuhdef),
        );

        base_scope.map.insert(
            String::from("neq?"),
            Scopable::Callback(base::logic::neqhuhdef),
        );

        base_scope
    }
}

pub fn exec(ast: Vec<AstNode>) -> Option<Result> {
    Program::new(ast, &Scope::base()).exec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execs_basic_sexp() {
        assert_eq!(
            exec(vec![
                AstNode::Leaf("if".to_string()),
                AstNode::Leaf("#t".to_string()),
                AstNode::Leaf("1".to_string())
            ])
            .unwrap(),
            Result::I(1)
        )
    }

    #[test]
    fn execs_nested_sexp() {
        assert_eq!(
            exec(vec![
                AstNode::Leaf("if".to_string()),
                AstNode::AST(vec![
                    AstNode::Leaf("if".to_string()),
                    AstNode::Leaf("1".to_string()),
                    AstNode::Leaf("#f".to_string())
                ]),
                AstNode::Leaf("1".to_string()),
                AstNode::Leaf("2".to_string())
            ])
            .unwrap(),
            Result::I(2)
        )
    }
}
