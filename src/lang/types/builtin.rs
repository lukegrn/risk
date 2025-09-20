use crate::lang::{ast::AstNode, scope::Scope, types::result::Result};

#[derive(Debug, Clone)]
pub struct Builtin {
    pub id: String,
    pub f: fn(Vec<AstNode>, &mut Scope) -> Option<Result>,
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_on_same_id() {
        let l = Builtin {
            id: "id".to_string(),
            f: |_v: Vec<AstNode>, _s: &mut Scope| None,
        };
        let r = Builtin {
            id: "id".to_string(),
            f: |_v: Vec<AstNode>, _s: &mut Scope| None,
        };
        assert!(l == r);
    }

    #[test]
    fn not_eq_based_on_id() {
        let l = Builtin {
            id: "id".to_string(),
            f: |_v: Vec<AstNode>, _s: &mut Scope| None,
        };
        let r = Builtin {
            id: "id2".to_string(),
            f: |_v: Vec<AstNode>, _s: &mut Scope| None,
        };
        assert!(l != r);
    }
}
