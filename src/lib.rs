mod parser;

pub use parser::ExprResult;

/// Evaluates an expression, including a definition.
pub fn eval_expr(input: &str) -> ExprResult {
    parser::parse_expr(input)
}
