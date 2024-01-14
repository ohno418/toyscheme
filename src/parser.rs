mod expr;
mod number;

use crate::types::Obj;

pub(crate) fn parse(input: &str) -> Obj {
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
