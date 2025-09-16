use crate::lang::{
    ast::AstNode,
    exec::{Program, Result, Scope},
};

pub fn ifdef(args: Vec<AstNode>, scope: &Scope) -> Option<Result> {
    if args.len() < 2 || args.len() > 3 {
        panic!(
            "Incorrect number of arguments to function <if>, expected 2 or 3, received {}",
            args.len()
        );
    }

    let gate = Program::new(vec![args[0].clone()], &scope).exec();

    // Everything except #f is true
    if gate != Some(Result::B(false)) {
        Program::new(vec![args[1].clone()], &scope).exec()
    } else {
        if args.len() == 3 {
            Program::new(vec![args[2].clone()], &scope).exec()
        } else {
            None
        }
    }
}

pub fn eqhuhdef(args: Vec<AstNode>, scope: &Scope) -> Option<Result> {
    if args.len() != 2 {
        panic!(
            "Incorrect number of arguments to function <eq?>, expected 2, received {}",
            args.len()
        )
    }

    let cmp = Program::new(vec![args[0].clone()], &scope).exec();
    let to = Program::new(vec![args[1].clone()], &scope).exec();

    match cmp {
        Some(cmpresult) => match to {
            Some(toresult) => Some(Result::B(cmpresult == toresult)),
            None => Some(Result::B(false)),
        },
        None => match to {
            Some(_) => Some(Result::B(false)),
            None => Some(Result::B(true)),
        },
    }
}

pub fn notdef(args: Vec<AstNode>, scope: &Scope) -> Option<Result> {
    if args.len() != 1 {
        panic!(
            "Incorrect number of arguments to function <not>, expected 1, received {}",
            args.len()
        )
    }

    let tonot = Program::new(vec![args[0].clone()], &scope).exec();

    match tonot {
        Some(result) => match result {
            Result::B(b) => Some(Result::B(!b)),
            // Anything but explicit #f nots to #f
            _ => Some(Result::B(false)),
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    mod ifdef {
        use super::super::*;

        #[test]
        fn if_happy_path() {
            assert_eq!(
                ifdef(
                    vec![
                        AstNode::Leaf("#t".to_string()),
                        AstNode::Leaf("1".to_string()),
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::I(1)
            )
        }

        #[test]
        fn if_with_else() {
            assert_eq!(
                ifdef(
                    vec![
                        AstNode::Leaf("#t".to_string()),
                        AstNode::Leaf("1".to_string()),
                        AstNode::Leaf("2".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::I(1)
            )
        }

        #[test]
        fn if_returns_else_when_strictly_false() {
            assert_eq!(
                ifdef(
                    vec![
                        AstNode::Leaf("#f".to_string()),
                        AstNode::Leaf("1".to_string()),
                        AstNode::Leaf("2".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::I(2)
            )
        }

        #[test]
        fn if_anything_but_false_is_true() {
            assert_eq!(
                ifdef(
                    vec![
                        AstNode::Leaf("88".to_string()),
                        AstNode::Leaf("1".to_string()),
                        AstNode::Leaf("2".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::I(1)
            );
        }
    }

    mod eqhuhdef {
        use super::super::*;

        #[test]
        fn eqhuh_bool_equality() {
            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("#t".to_string()),
                        AstNode::Leaf("#t".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::B(true)
            );

            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("#f".to_string()),
                        AstNode::Leaf("#f".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::B(true)
            );
        }

        #[test]
        fn eqhuh_bool_inequality() {
            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("#f".to_string()),
                        AstNode::Leaf("#t".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::B(false)
            );
        }

        #[test]
        fn eqhuh_int_equlaity() {
            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("100".to_string()),
                        AstNode::Leaf("100".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::B(true)
            );
        }

        #[test]
        fn eqhuh_int_inequlaity() {
            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("100".to_string()),
                        AstNode::Leaf("200".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::B(false)
            );
        }

        #[test]
        fn eqhuh_float_equality() {
            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("100.1".to_string()),
                        AstNode::Leaf("100.1".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::B(true)
            );
        }

        #[test]
        fn eqhuh_float_inequality() {
            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("100.1".to_string()),
                        AstNode::Leaf("200.2".to_string())
                    ],
                    &Scope::base()
                )
                .unwrap(),
                Result::B(false)
            );
        }
    }

    mod notdef {
        use super::super::*;

        #[test]
        fn notdef_true_for_false() {
            assert_eq!(
                notdef(vec![AstNode::Leaf("#f".to_string())], &Scope::base()).unwrap(),
                Result::B(true)
            )
        }

        #[test]
        fn notdef_false_for_everything_but_explicit_false() {
            assert_eq!(
                notdef(vec![AstNode::Leaf("#t".to_string())], &Scope::base()).unwrap(),
                Result::B(false)
            );
            assert_eq!(
                notdef(vec![AstNode::Leaf("123".to_string())], &Scope::base()).unwrap(),
                Result::B(false)
            );
        }
    }
}
