use std::fmt::Display;

use crate::lang::types::{builtin::Builtin, primitive::Primitive, userfunc::FnDef};

#[derive(PartialEq, Debug, Clone)]
pub enum Result {
    Primitive(Primitive),
    Builtin(Builtin),
    FnDef(FnDef),
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::Primitive(p) => match p {
                Primitive::I(i) => write!(f, "{}", i),
                Primitive::F(fl) => write!(f, "{}", fl),
                Primitive::B(b) => match b {
                    true => write!(f, "#t"),
                    false => write!(f, "#f"),
                },
            },
            Result::Builtin(func) => write!(f, "builtin#{}", func.id),
            Result::FnDef(fn_def) => {
                write!(f, "userfuncdef#{:?}{:?}", fn_def.params, fn_def.body)
            }
        }
    }
}
