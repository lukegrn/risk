use std::fmt;

#[derive(PartialEq, Clone)]
pub enum AstNode {
    Leaf(String),
    AST(Vec<AstNode>),
}

impl fmt::Debug for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Leaf(s) => s.fmt(f),
            AstNode::AST(nodes) => {
                write!(f, "[")?;
                for (i, node) in nodes.iter().enumerate() {
                    let _ = node.fmt(f);

                    if i + 1 != nodes.len() {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;

                Ok(())
            }
        }
    }
}

fn find_closing_paren(tokens: Vec<String>) -> Option<usize> {
    let (closing, _, found) = tokens.iter().fold((0, 0, false), |acc, token| {
        let (mut index, mut opening_count, mut found) = acc;
        if found {
            return acc;
        }

        if token == "(" {
            opening_count += 1;
        } else if token == ")" {
            opening_count -= 1;
        }

        if opening_count != 0 {
            index += 1
        } else {
            found = true
        }

        (index, opening_count, found)
    });

    if found { Some(closing) } else { None }
}

pub fn new(tokens: Vec<String>) -> Vec<AstNode> {
    match tokens[..] {
        [] => Vec::new(),
        _ => match tokens[0].as_str() {
            ")" => panic!("Syntax error, unexpected ')'"),
            "(" => {
                let closing =
                    find_closing_paren(tokens.clone()).expect("closing paren to be found");

                let mut tree: Vec<AstNode> = vec![AstNode::AST(new(tokens[1..closing].to_vec()))];
                tree.extend(new(tokens[closing + 1..].to_vec()));

                tree
            }
            _ => {
                let (first, rest) = tokens.split_first().expect("to have at least one value");
                let mut parsed = vec![AstNode::Leaf(first.to_string())];
                parsed.extend(new(rest.to_vec()));
                parsed
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_closing_paren_doesnt_on_mismatch_closing() {
        assert_eq!(
            find_closing_paren(["(", "(", ")"].iter().map(|s| s.to_string()).collect()),
            None
        )
    }

    #[test]
    fn find_closing_paren_does_on_excess_closing() {
        assert_eq!(
            find_closing_paren(["(", ")", ")"].iter().map(|s| s.to_string()).collect()),
            Some(1)
        )
    }

    #[test]
    fn find_closing_paren_does_on_well_formed() {
        assert_eq!(
            find_closing_paren(
                ["(", "sym", "other", "token", ")"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            Some(4)
        )
    }

    #[test]
    fn new_ast_works_on_well_formed() {
        assert_eq!(
            new(["(", "sym", ")", "(", "lambda", "(", "x", ")", "x", ")"]
                .iter()
                .map(|s| s.to_string())
                .collect()),
            vec![
                AstNode::AST(vec![AstNode::Leaf("sym".to_string())]),
                AstNode::AST(vec![
                    AstNode::Leaf("lambda".to_string()),
                    AstNode::AST(vec![AstNode::Leaf("x".to_string())]),
                    AstNode::Leaf("x".to_string())
                ])
            ]
        )
    }

    #[test]
    fn new_ast_works_for_single_sym() {
        assert_eq!(
            new(["sym"].iter().map(|s| s.to_string()).collect()),
            vec![AstNode::Leaf("sym".to_string())]
        )
    }

    #[test]
    #[should_panic]
    fn new_ast_fails_for_syntax_error_mismatched_parens() {
        new(["(", "(", "(", "(", ")", ")", ")"]
            .iter()
            .map(|s| s.to_string())
            .collect());
    }

    #[test]
    #[should_panic]
    fn new_ast_fails_for_syntax_error_excess_closing_parens() {
        new(["(", ")", ")", ")"].iter().map(|s| s.to_string()).collect());
    }
}
