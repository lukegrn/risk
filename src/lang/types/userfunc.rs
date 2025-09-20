use crate::lang::{ast::AstNode, exec::Program, scope::Scope, types::result::Result};

#[derive(PartialEq, Debug, Clone)]
pub struct FnDef {
    pub params: Vec<String>,
    pub body: AstNode,
}

impl FnDef {
    pub fn exec(&self, args: Vec<AstNode>, scope: &mut Scope) -> Option<Result> {
        // todo: explore something better than duplicating the entire scope
        let mut local_scope = scope.clone();
        if args.len() != self.params.len() {
            panic!(
                "Incorrect number of arguments provided. Expected {}, received {}",
                self.params.len(),
                args.len()
            )
        }

        for (i, param) in self.params.clone().iter().enumerate() {
            match Program::new(args[i].clone(), scope).exec() {
                Some(s) => local_scope.map.insert(param.to_string(), s),
                None => panic!("Cannot pass none to function"),
            };
        }

        Program::new(self.body.clone(), &mut local_scope).exec()
    }

    pub fn new(params: Vec<String>, body: AstNode) -> FnDef {
        FnDef {
            params: params,
            body: body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::types::primitive::Primitive;

    #[test]
    fn fn_def_exec_with_no_params_or_args() {
        let f = FnDef::new(vec![], AstNode::Leaf("2".to_string()));

        assert_eq!(
            f.exec(vec![], &mut Scope::base()).unwrap(),
            Result::Primitive(Primitive::I(2))
        )
    }

    #[test]
    #[should_panic]
    fn fn_def_fails_with_mismatched_params_and_args() {
        let f = FnDef::new(
            vec!["x".to_string(), "y".to_string()],
            AstNode::Leaf("2".to_string()),
        );

        assert_eq!(
            f.exec(vec![], &mut Scope::base()).unwrap(),
            Result::Primitive(Primitive::I(2))
        )
    }

    #[test]
    fn fn_def_applies_args_to_params() {
        let f = FnDef::new(
            vec!["x".to_string(), "y".to_string()],
            AstNode::AST(vec![
                AstNode::Leaf("eq?".to_string()),
                AstNode::Leaf("x".to_string()),
                AstNode::Leaf("y".to_string()),
            ]),
        );

        assert_eq!(
            f.exec(
                vec![
                    AstNode::Leaf("1".to_string()),
                    AstNode::Leaf("1".to_string())
                ],
                &mut Scope::base()
            )
            .unwrap(),
            Result::Primitive(Primitive::B(true))
        )
    }

    #[test]
    fn fn_def_args_can_evaluate_themselves() {
        let f = FnDef::new(
            vec!["x".to_string(), "y".to_string()],
            AstNode::AST(vec![
                AstNode::Leaf("eq?".to_string()),
                AstNode::Leaf("x".to_string()),
                AstNode::Leaf("y".to_string()),
            ]),
        );

        assert_eq!(
            f.exec(
                vec![
                    AstNode::AST(vec![
                        AstNode::Leaf("if".to_string()),
                        AstNode::Leaf("#t".to_string()),
                        AstNode::Leaf("1".to_string()),
                    ]),
                    AstNode::Leaf("1".to_string())
                ],
                &mut Scope::base()
            )
            .unwrap(),
            Result::Primitive(Primitive::B(true))
        )
    }
}
