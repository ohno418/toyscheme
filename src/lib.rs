mod parser;

pub use parser::Ast;

/// Evaluates an expression, including a definition.
pub fn eval(input: &str) -> Ast {
    parser::parse_expr(input)
}
