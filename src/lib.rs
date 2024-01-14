mod eval;
mod parser;
mod types;

pub use types::Obj;

/// Evaluates user input as an expression, including a definition.
pub fn eval(input: &str) -> Obj {
    let ast = parser::parse(input);
    eval::eval_ast(ast)
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
    fn eval_unknown_sym() {
        let input = "hi\n";
        let result = eval(input);
        assert_eq!(result.to_string(), "error: unbound variable: hi");
    }
}
