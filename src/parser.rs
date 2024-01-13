mod expr;
mod number;

/// Represents an abstract syntax tree (AST), the result of the parsing.
#[derive(Debug, PartialEq)]
pub(crate) enum Ast {
    /// Number
    Num(i64),
    /// Quote
    Quote(Box<Ast>),
    /// No expression to parse found
    None,
    /// Error
    Err(String),
}

pub(crate) fn parse(input: &str) -> Ast {
    // Strip terminating '\n'.
    let mut input = if input.ends_with('\n') {
        &input[..input.len() - 1]
    } else {
        // NOTE: It's not assumed that a terminating '\n' is missing, but we
        // handle the case just by ignoring.
        input
    };

    expr::parse_expr(&mut input)
}
