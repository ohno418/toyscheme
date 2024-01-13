mod parser;

pub use parser::ExprResult;

/// Evaluates an expression, including a definition.
pub fn eval(input: &str) -> ExprResult {
    parser::parse_expr(input)
}
