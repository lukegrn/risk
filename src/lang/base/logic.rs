use crate::lang::{
    ast::AstNode,
    exec::Program,
    scope::Scope,
    types::{primitive::Primitive, result::Result},
};

pub fn ifdef(args: Vec<AstNode>, scope: &mut Scope) -> Option<Result> {
    if args.len() < 2 || args.len() > 3 {
        panic!(
            "Incorrect number of arguments to function <if>, expected 2 or 3, received {}",
            args.len()
        );
    }

    let gate = Program::new(args[0].clone(), scope).exec();
    let on_true = |s: &mut Scope| Program::new(args[1].clone(), s).exec();
    let on_false = |s: &mut Scope| Program::new(args[2].clone(), s).exec();

    match gate {
        Some(s) => match s {
            Result::Primitive(p) => match p {
                Primitive::B(b) => {
                    // Everything expects #f explicitly is true
                    if !b { on_false(scope) } else { on_true(scope) }
                }
                _ => on_true(scope),
            },
            _ => on_true(scope),
        },
        None => on_true(scope),
    }
}

pub fn eqhuhdef(args: Vec<AstNode>, scope: &mut Scope) -> Option<Result> {
    if args.len() != 2 {
        panic!(
            "Incorrect number of arguments to function <eq?>, expected 2, received {}",
            args.len()
        )
    }

    let cmp = Program::new(args[0].clone(), scope).exec();
    let to = Program::new(args[1].clone(), scope).exec();

    match cmp {
        Some(from) => match to {
            Some(res) => Some(Result::Primitive(Primitive::B(from == res))),
            None => Some(Result::Primitive(Primitive::B(false))),
        },
        None => match to {
            Some(_) => Some(Result::Primitive(Primitive::B(false))),
            None => Some(Result::Primitive(Primitive::B(true))),
        },
    }
}

pub fn notdef(args: Vec<AstNode>, scope: &mut Scope) -> Option<Result> {
    if args.len() != 1 {
        panic!(
            "Incorrect number of arguments to function <not>, expected 1, received {}",
            args.len()
        )
    }

    let tonot = Program::new(args[0].clone(), scope).exec();

    match tonot {
        Some(s) => match s {
            Result::Primitive(p) => match p {
                Primitive::B(b) => {
                    if b {
                        Some(Result::Primitive(Primitive::B(false)))
                    } else {
                        Some(Result::Primitive(Primitive::B(true)))
                    }
                }
                // anything but explicit #f nots to #f
                _ => Some(Result::Primitive(Primitive::B(false))),
            },
            _ => Some(Result::Primitive(Primitive::B(false))),
        },
        None => Some(Result::Primitive(Primitive::B(false))),
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::I(1))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::I(1))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::I(2))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::I(1))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::B(true))
            );

            assert_eq!(
                eqhuhdef(
                    vec![
                        AstNode::Leaf("#f".to_string()),
                        AstNode::Leaf("#f".to_string())
                    ],
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::B(true))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::B(false))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::B(true))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::B(false))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::B(true))
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
                    &mut Scope::base()
                )
                .unwrap(),
                Result::Primitive(Primitive::B(false))
            );
        }
    }

    mod notdef {
        use super::super::*;

        #[test]
        fn notdef_true_for_false() {
            assert_eq!(
                notdef(vec![AstNode::Leaf("#f".to_string())], &mut Scope::base()).unwrap(),
                Result::Primitive(Primitive::B(true))
            )
        }

        #[test]
        fn notdef_false_for_everything_but_explicit_false() {
            assert_eq!(
                notdef(vec![AstNode::Leaf("#t".to_string())], &mut Scope::base()).unwrap(),
                Result::Primitive(Primitive::B(false))
            );
            assert_eq!(
                notdef(vec![AstNode::Leaf("123".to_string())], &mut Scope::base()).unwrap(),
                Result::Primitive(Primitive::B(false))
            );
        }
    }
}
