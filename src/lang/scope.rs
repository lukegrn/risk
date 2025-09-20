use crate::lang::base;
use crate::lang::types::builtin::Builtin;
use crate::lang::types::primitive::Primitive;
use crate::lang::types::result::Result;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Scope {
    pub map: HashMap<String, Result>,
}

impl Scope {
    pub fn base() -> Scope {
        let mut base_scope: Scope = Scope {
            map: HashMap::new(),
        };

        // bools
        base_scope
            .map
            .insert(String::from("#t"), Result::Primitive(Primitive::B(true)));

        base_scope
            .map
            .insert(String::from("#f"), Result::Primitive(Primitive::B(false)));

        // definition
        base_scope.map.insert(
            String::from("define"),
            Result::Builtin(Builtin {
                id: "define".to_string(),
                f: base::definition::definitiondef,
            }),
        );

        // logic functions
        base_scope.map.insert(
            String::from("if"),
            Result::Builtin(Builtin {
                id: "if".to_string(),
                f: base::logic::ifdef,
            }),
        );

        base_scope.map.insert(
            String::from("eq?"),
            Result::Builtin(Builtin {
                id: "eq?".to_string(),
                f: base::logic::eqhuhdef,
            }),
        );

        base_scope.map.insert(
            String::from("not"),
            Result::Builtin(Builtin {
                id: "not".to_string(),
                f: base::logic::notdef,
            }),
        );

        base_scope
    }
}
