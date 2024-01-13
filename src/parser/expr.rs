use super::number::parse_number;

#[derive(Debug, PartialEq)]
pub enum ExprResult {
    /// Number
    Num(i64),
    /// No expression found
    Nop,
    /// Error
    Err(String),
}

/// Parses and evaluates an expression, including a definition.
pub fn parse_expr(input: &str) -> ExprResult {
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
                    Ok(num) => ExprResult::Num(num),
                    Err(msg) => ExprResult::Err(msg),
                }
            } else {
                ExprResult::Err("unknown input".to_string())
            }
        }
        None => ExprResult::Nop,
    }

    // TODO: Check if extra input.
}

#[cfg(test)]
mod parse_expr_tests {
    use super::*;

    #[test]
    fn return_empty_string_with_empty_input() {
        let input = "";
        let result = parse_expr(input);
        assert_eq!(result, ExprResult::Nop);
    }

    #[test]
    fn return_empty_string_with_only_newline() {
        let input = "\n";
        let result = parse_expr(input);
        assert_eq!(result, ExprResult::Nop);
    }

    #[test]
    fn handle_input_without_terminated_newline() {
        let input = "42";
        let result = parse_expr(input);
        assert_eq!(result, ExprResult::Num(42));
    }

    #[test]
    fn parse_number_and_return_as_is() {
        let input = "42\n";
        let result = parse_expr(input);
        assert_eq!(result, ExprResult::Num(42));
    }

    #[test]
    fn return_error_msg_with_non_number() {
        let input = "hello\n";
        let result = parse_expr(input);
        assert_eq!(result, ExprResult::Err("unknown input".to_owned()));
    }
}
