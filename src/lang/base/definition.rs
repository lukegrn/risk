use crate::lang::{
    ast::AstNode,
    exec::Program,
    scope::Scope,
    types::{result::Result, userfunc::FnDef},
};

/*
* Used to define functions and variables
*
* The following forms are supported
*
* (define (fn params...) expr) => None
* (define var expr) => None
*/
pub fn definitiondef(args: Vec<AstNode>, scope: &mut Scope) -> Option<Result> {
    match &args[..] {
        [] => panic!("Empty call to define"),
        [_] => panic!("Must provide right hand side to set the left hand side to"),
        [subject, expr] => match subject {
            AstNode::Leaf(varname) => {
                let val = Program::new(expr.to_owned(), scope).exec();

                match val {
                    Some(result) => {
                        scope.map.insert(varname.to_string(), result);
                        None
                    }
                    None => panic!("Right hand side evaluated to nothing"),
                }
            }
            AstNode::AST(function_signature) => match &function_signature[..] {
                [] => panic!("Must provide function name"),
                [fname, params @ ..] => match fname {
                    AstNode::Leaf(n) => {
                        let str_params = params.iter().map(|p| match p {
                            AstNode::Leaf(p) => p.to_string(),
                            AstNode::AST(_) => {
                                panic!("All function parameters must be simple strings")
                            }
                        });

                        scope.map.insert(
                            n.to_string(),
                            Result::FnDef(FnDef::new(str_params.collect(), expr.clone())),
                        );
                        None
                    }
                    AstNode::AST(_) => panic!("Cannot use an expression as function name"),
                },
            },
        },
        _ => panic!("define can only be called with two parameters"),
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::types::primitive::Primitive;

    use super::*;

    #[test]
    #[should_panic]
    fn definitiondef_panics_empty_call() {
        definitiondef(vec![], &mut Scope::base());
    }

    #[test]
    #[should_panic]
    fn definitiondef_panics_missing_rhs() {
        definitiondef(vec![AstNode::Leaf("x".to_string())], &mut Scope::base());
    }

    #[test]
    fn definitiondef_defines_variable() {
        let s = &mut Scope::base();
        definitiondef(
            vec![
                AstNode::Leaf("x".to_string()),
                AstNode::AST(vec![
                    AstNode::Leaf("if".to_string()),
                    AstNode::Leaf("#t".to_string()),
                    AstNode::Leaf("1".to_string()),
                ]),
            ],
            s,
        );

        assert_eq!(
            Program::new(AstNode::Leaf("x".to_string()), s)
                .exec()
                .unwrap(),
            Result::Primitive(Primitive::I(1))
        )
    }

    #[test]
    fn definitiondef_defines_userfunc_with_no_params() {
        let s = &mut Scope::base();
        definitiondef(
            vec![
                AstNode::AST(vec![AstNode::Leaf("func".to_string())]),
                AstNode::AST(vec![
                    AstNode::Leaf("if".to_string()),
                    AstNode::Leaf("#t".to_string()),
                    AstNode::Leaf("1".to_string()),
                ]),
            ],
            s,
        );

        assert_eq!(
            Program::new(AstNode::AST(vec![AstNode::Leaf("func".to_string())]), s)
                .exec()
                .unwrap(),
            Result::Primitive(Primitive::I(1))
        )
    }

    #[test]
    fn definitiondef_defines_userfunc_with_params() {
        let s = &mut Scope::base();
        definitiondef(
            vec![
                AstNode::AST(vec![
                    AstNode::Leaf("func".to_string()),
                    AstNode::Leaf("x".to_string()),
                    AstNode::Leaf("y".to_string()),
                ]),
                AstNode::AST(vec![
                    AstNode::Leaf("if".to_string()),
                    AstNode::AST(vec![
                        AstNode::Leaf("eq?".to_string()),
                        AstNode::Leaf("x".to_string()),
                        AstNode::Leaf("y".to_string()),
                    ]),
                    AstNode::Leaf("1".to_string()),
                    AstNode::Leaf("2".to_string()),
                ]),
            ],
            s,
        );

        assert_eq!(
            Program::new(
                AstNode::AST(vec![
                    AstNode::Leaf("func".to_string()),
                    AstNode::Leaf("1".to_string()),
                    AstNode::Leaf("2".to_string())
                ]),
                s
            )
            .exec()
            .unwrap(),
            Result::Primitive(Primitive::I(2))
        )
    }
}
