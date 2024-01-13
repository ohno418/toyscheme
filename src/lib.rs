mod parser;

pub use parser::ExprResult;

/// Reads and evaluates an expression, including a definition.
pub fn read_eval_expr(input: &str) -> ExprResult {
    parser::read_expr(input)
}
