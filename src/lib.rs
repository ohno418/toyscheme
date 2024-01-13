mod eval;
mod parser;

pub use eval::EvalResult;

/// Evaluates user input as an expression, including a definition.
pub fn eval(input: &str) -> EvalResult {
    let ast = parser::parse_expr(input);
    eval::eval(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_empty_input() {
        let input = "\n";
        let result = eval(input);
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn eval_number() {
        let input = "42\n";
        let result = eval(input);
        assert_eq!(result.to_string(), "42");
    }

    #[test]
    fn eval_unknown_ident() {
        let input = "hi\n";
        let result = eval(input);
        assert_eq!(result.to_string(), "error: unknown input");
    }
}
