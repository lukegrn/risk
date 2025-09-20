use crate::lang::types::result::Result;
use crate::lang::{scope::Scope, types::primitive::Primitive};

use crate::lang::ast::AstNode;

pub struct Program<'a> {
    scope: &'a mut Scope,
    ast: AstNode,
}

impl Program<'_> {
    pub fn exec(&mut self) -> Option<Result> {
        match &self.ast.clone() {
            // Reference
            // Try int -> float -> scope lookup (bools are defined in scope)
            AstNode::Leaf(l) => match l.parse::<i32>() {
                Ok(i) => Some(Result::Primitive(Primitive::I(i))),
                Err(_) => match l.parse::<f64>() {
                    Ok(f) => Some(Result::Primitive(Primitive::F(f))),
                    Err(_) => match self.scope.map.get(l) {
                        Some(s) => match s {
                            Result::Primitive(r) => Some(Result::Primitive(r.clone())),
                            Result::Builtin(f) => Some(Result::Builtin(f.clone())),
                            Result::FnDef(fn_def) => Some(Result::FnDef(fn_def.clone())),
                        },
                        None => panic!("undefined scope for {}", l),
                    },
                },
            },

            // Function call
            AstNode::AST(ast_nodes) => match &ast_nodes[..] {
                [] => None,
                [a, rest @ ..] => match Program::new(a.clone(), self.scope).exec() {
                    Some(result) => match result {
                        Result::Primitive(p) => {
                            panic!("Call to value {} as a function", Result::Primitive(p))
                        }
                        Result::Builtin(f) => (f.f)(rest.to_vec(), self.scope),
                        Result::FnDef(fn_def) => fn_def.exec(rest.to_vec(), self.scope),
                    },
                    None => None,
                },
            },
        }
    }

    pub fn new(ast: AstNode, scope: &mut Scope) -> Program<'_> {
        Program {
            scope: scope,
            ast: ast,
        }
    }
}

pub fn exec(exprs: Vec<AstNode>, scope: Option<&mut Scope>) -> Option<Result> {
    match scope {
        Some(s) => exprs
            .iter()
            .fold(None, |_, exp| Program::new(exp.clone(), s).exec()),
        None => {
            let mut s = Scope::base();
            exprs
                .iter()
                .fold(None, |_, exp| Program::new(exp.clone(), &mut s).exec())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execs_basic_sexp() {
        assert_eq!(
            exec(
                vec![AstNode::AST(vec![
                    AstNode::Leaf("if".to_string()),
                    AstNode::Leaf("#t".to_string()),
                    AstNode::Leaf("1".to_string())
                ])],
                None
            )
            .unwrap(),
            Result::Primitive(Primitive::I(1))
        )
    }

    #[test]
    fn execs_nested_sexp() {
        assert_eq!(
            exec(
                vec![AstNode::AST(vec![
                    AstNode::Leaf("if".to_string()),
                    AstNode::AST(vec![
                        AstNode::Leaf("if".to_string()),
                        AstNode::Leaf("1".to_string()),
                        AstNode::Leaf("#f".to_string())
                    ]),
                    AstNode::Leaf("1".to_string()),
                    AstNode::Leaf("2".to_string())
                ])],
                None
            )
            .unwrap(),
            Result::Primitive(Primitive::I(2))
        )
    }

    #[test]
    fn execs_multiple_sexp_and_returns_last() {
        assert_eq!(
            exec(
                vec![
                    AstNode::AST(vec![
                        AstNode::Leaf("define".to_string()),
                        AstNode::Leaf("x".to_string()),
                        AstNode::Leaf("1".to_string()),
                    ]),
                    AstNode::Leaf("x".to_string())
                ],
                None
            )
            .unwrap(),
            Result::Primitive(Primitive::I(1))
        )
    }
}
