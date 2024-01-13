mod eval;
mod parser;

pub use eval::EvalResult;

/// Evaluates user input as an expression, including a definition.
pub fn eval(input: &str) -> EvalResult {
    let ast = parser::parse_expr(input);
    eval::eval(ast)
}
