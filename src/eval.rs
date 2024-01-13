use super::parser::Ast;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    /// Number
    Num(i64),
    /// No result
    None,
    /// Error
    Err(String),
}

pub(super) fn eval(ast: Ast) -> EvalResult {
    match ast {
        Ast::Num(num) => EvalResult::Num(num),
        Ast::Quote(inner) => eval(*inner),
        Ast::None => EvalResult::None,
        Ast::Err(msg) => EvalResult::Err(msg),
    }
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            EvalResult::Num(num) => write!(f, "{}", num),
            EvalResult::None => write!(f, ""),
            EvalResult::Err(msg) => write!(f, "error: {}", msg),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use super::*;

    #[test]
    fn eval_num() {
        let ast = Ast::Num(42);
        let result = eval(ast);
        assert_eq!(result, EvalResult::Num(42));
    }

    #[test]
    fn eval_quoted_num() {
        let ast = Ast::Quote(Box::new(Ast::Num(42)));
        let result = eval(ast);
        assert_eq!(result, EvalResult::Num(42));
    }

    #[test]
    fn eval_none() {
        let ast = Ast::None;
        let result = eval(ast);
        assert_eq!(result, EvalResult::None);
    }

    #[test]
    fn eval_err() {
        let ast = Ast::Err("some error!".to_string());
        let result = eval(ast);
        assert_eq!(result, EvalResult::Err("some error!".to_string()));
    }
}

#[cfg(test)]
mod display_result_tests {
    use super::*;

    #[test]
    fn display_num() {
        let eval_result = EvalResult::Num(42);
        assert_eq!(eval_result.to_string(), "42".to_string());
    }

    #[test]
    fn display_none() {
        let eval_result = EvalResult::None;
        assert_eq!(eval_result.to_string(), "".to_string());
    }

    #[test]
    fn display_err() {
        let eval_result = EvalResult::Err("some error".to_string());
        assert_eq!(eval_result.to_string(), "error: some error".to_string());
    }
}
