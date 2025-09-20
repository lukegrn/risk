pub fn tokenize(line: &str) -> Vec<String> {
    line.replace("(", " ( ")
        .replace(")", " ) ")
        .replace("\n", " ")
        .replace("\t", " ")
        .split(" ")
        .map(|s| s.to_string())
        .filter(|s| !s.eq(""))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_single_token() {
        assert_eq!(tokenize("token"), ["token"]);
    }

    #[test]
    fn tokenize_list_of_tokens() {
        assert_eq!(
            tokenize("(1 2 3)) 1 token"),
            ["(", "1", "2", "3", ")", ")", "1", "token"]
        )
    }

    #[test]
    fn tokenize_strips_excess_whitespace() {
        assert_eq!(
            tokenize("( 1 2    3) ) () ) 1 token"),
            ["(", "1", "2", "3", ")", ")", "(", ")", ")", "1", "token"]
        )
    }

    #[test]
    fn tokenize_handles_newline_and_tabs() {
        assert_eq!(
            tokenize(
                "( 1 2 3
                )"
            ),
            ["(", "1", "2", "3", ")"]
        );
    }

    #[test]
    fn tokenize_handles_touching_sexps() {
        assert_eq!(
            tokenize("(1 2 3)(1)"),
            ["(", "1", "2", "3", ")", "(", "1", ")"]
        );
    }
}
