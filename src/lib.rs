use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum ExprResult {
    /// Number
    Num(u64),
    /// No expression found
    Nop,
    /// Error
    Err(String),
}

pub fn read_expr(input: &str) -> ExprResult {
    // Strip terminating '\n'.
    let input = if input.ends_with('\n') {
        &input[..input.len() - 1]
    } else {
        // NOTE: It's not assumed that a terminating '\n' is missing, but we
        // handle the case just by ignoring.
        input
    };

    let mut chars = input.chars().peekable();
    match chars.peek() {
        Some(c) => {
            if c.is_digit(10) {
                match read_number(&mut chars) {
                    Ok(num) => ExprResult::Num(num),
                    Err(msg) => ExprResult::Err(msg),
                }
            } else {
                ExprResult::Err("unknown input".to_string())
            }
        }
        None => ExprResult::Nop,
    }
}

fn read_number(input: &mut Peekable<Chars>) -> Result<u64, String> {
    let mut num_str = String::new();
    while let Some(c) = input.peek() {
        if c.is_digit(10) {
            let c = input.next().unwrap();
            num_str.push(c);
        } else {
            break;
        }
    }
    num_str.parse::<u64>().map_err(|_| {
        format!(
            "cannot read a number from \"{}\"",
            input.clone().collect::<String>()
        )
    })
}

#[cfg(test)]
mod read_expr_tests {
    use super::*;

    #[test]
    fn return_empty_string_with_empty_input() {
        let input = "";
        let result = read_expr(input);
        assert_eq!(result, ExprResult::Nop);
    }

    #[test]
    fn return_empty_string_with_only_newline() {
        let input = "\n";
        let result = read_expr(input);
        assert_eq!(result, ExprResult::Nop);
    }

    #[test]
    fn handle_input_without_terminated_newline() {
        let input = "42";
        let result = read_expr(input);
        assert_eq!(result, ExprResult::Num(42));
    }

    #[test]
    fn parse_number_and_return_as_is() {
        let input = "42\n";
        let result = read_expr(input);
        assert_eq!(result, ExprResult::Num(42));
    }

    #[test]
    fn return_error_msg_with_non_number() {
        let input = "hello\n";
        let result = read_expr(input);
        assert_eq!(result, ExprResult::Err("unknown input".to_owned()));
    }
}

#[cfg(test)]
mod read_number_tests {
    use super::read_number;

    #[test]
    fn read_a_number_from_chars() {
        let s = "42 hello".to_string();
        let mut chars = s.chars().peekable();
        let result = read_number(&mut chars);
        assert_eq!(result, Ok(42));
        assert_eq!(chars.collect::<String>(), " hello".to_string());
    }

    #[test]
    fn cannot_read_negative_number() {
        let s = "-42 hello".to_string();
        let mut chars = s.chars().peekable();
        let result = read_number(&mut chars);
        assert_eq!(
            result,
            Err("cannot read a number from \"-42 hello\"".to_string())
        );
        assert_eq!(chars.collect::<String>(), "-42 hello".to_string());
    }

    #[test]
    fn cannot_read_non_number() {
        let s = "hello".to_string();
        let mut chars = s.chars().peekable();
        let result = read_number(&mut chars);
        assert_eq!(
            result,
            Err("cannot read a number from \"hello\"".to_string())
        );
        assert_eq!(chars.collect::<String>(), "hello".to_string());
    }
}
