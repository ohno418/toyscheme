use super::number::parse_number;
use super::Ast;

/// Parses an expression, including a definition.
pub(super) fn parse_expr(input: &mut &str) -> Ast {
    match input.chars().next() {
        Some(c) => {
            if c.is_digit(10) || c == '-' {
                // number
                parse_number(input)
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
        let mut input = "";
        let result = parse_expr(&mut input);
        assert_eq!(result, Ast::None);
        assert_eq!(input, "");
    }

    #[test]
    fn parse_number() {
        let mut input = "42";
        let result = parse_expr(&mut input);
        assert_eq!(result, Ast::Num(42));
        assert_eq!(input, "");
    }

    #[test]
    fn return_ast_error_msg_with_non_number() {
        let mut input = "%@!*";
        let result = parse_expr(&mut input);
        assert_eq!(result, Ast::Err("unknown input".to_owned()));
        assert_eq!(input, "%@!*");
    }
}
