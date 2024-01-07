pub fn read_expr(input: &str) -> Result<String, String> {
    // Strip terminating '\n'.
    let input = if input.ends_with('\n') {
        &input[..input.len() - 1]
    } else {
        // NOTE: It's not assumed that a terminating '\n' is missing, but we
        // handle the case just by ignoring.
        input
    };

    // Do not print if only a newline ("\n") is read.
    if input.is_empty() {
        return Ok("".to_owned());
    }

    match input.parse::<u64>() {
        Ok(n) => Ok(n.to_string()),
        Err(_) => Err(format!("cannot parse \"{}\" as a number", input)),
    }
}

#[cfg(test)]
mod read_expr_tests {
    use super::read_expr;

    #[test]
    fn return_empty_string_with_empty_input() {
        let input = "";
        let result = read_expr(input);
        assert_eq!(result, Ok("".to_owned()));
    }

    #[test]
    fn return_empty_string_with_only_newline() {
        let input = "\n";
        let result = read_expr(input);
        assert_eq!(result, Ok("".to_owned()));
    }

    #[test]
    fn handle_input_without_terminated_newline() {
        let input = "42";
        let result = read_expr(input);
        assert_eq!(result, Ok("42".to_owned()));
    }

    #[test]
    fn parse_number_and_return_as_is() {
        let input = "42\n";
        let result = read_expr(input);
        assert_eq!(result, Ok("42".to_owned()));
    }

    #[test]
    fn return_error_msg_with_non_number() {
        let input = "hello\n";
        let result = read_expr(input);
        assert_eq!(result, Err("cannot parse \"hello\" as a number".to_owned()));
    }
}
