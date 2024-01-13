use super::number::parse_number;

/// Represents an abstract syntax tree (AST), the result of the parsing.
#[derive(Debug, PartialEq)]
pub(crate) enum Ast {
    /// Number
    Num(i64),
    /// No expression to parse found
    None,
    /// Error
    Err(String),
}

/// Parses an expression, including a definition.
pub(crate) fn parse_expr(input: &str) -> Ast {
    // Strip terminating '\n'.
    let mut input = if input.ends_with('\n') {
        &input[..input.len() - 1]
    } else {
        // NOTE: It's not assumed that a terminating '\n' is missing, but we
        // handle the case just by ignoring.
        input
    };

    match input.chars().next() {
        Some(c) => {
            if c.is_digit(10) || c == '-' {
                match parse_number(&mut input) {
                    Ok(num) => Ast::Num(num),
                    Err(msg) => Ast::Err(msg),
                }
            } else {
                Ast::Err("unknown input".to_string())
            }
        }
        None => Ast::None,
    }

    // TODO: Check if extra input.
}

#[cfg(test)]
mod parse_expr_tests {
    use super::*;

    #[test]
    fn return_none_ast_with_empty_input() {
        let input = "";
        let result = parse_expr(input);
        assert_eq!(result, Ast::None);
    }

    #[test]
    fn return_none_ast_with_only_newline() {
        let input = "\n";
        let result = parse_expr(input);
        assert_eq!(result, Ast::None);
    }

    #[test]
    fn parse_number_without_terminated_newline() {
        let input = "42";
        let result = parse_expr(input);
        assert_eq!(result, Ast::Num(42));
    }

    #[test]
    fn parse_number_with_terminated_newline() {
        let input = "42\n";
        let result = parse_expr(input);
        assert_eq!(result, Ast::Num(42));
    }

    #[test]
    fn return_ast_error_msg_with_non_number() {
        let input = "hello\n";
        let result = parse_expr(input);
        assert_eq!(result, Ast::Err("unknown input".to_owned()));
    }
}
