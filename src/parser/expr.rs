use super::number::parse_number;
use super::Obj;

/// Parses an expression, including a definition.
pub(super) fn parse_expr(input: &mut &str) -> Obj {
    match input.chars().next() {
        Some(c) => {
            if c.is_digit(10) || c == '-' {
                // number
                parse_number(input)
            } else if is_sym_char(c) {
                // symbol
                parse_symbol(input)
            } else {
                Obj::Err("unknown input".to_string())
            }
        }
        None => Obj::None,
    }

    // TODO: Check if extra input.
}

fn parse_symbol(input: &mut &str) -> Obj {
    let mut sym = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if is_sym_char(c) {
            sym.push(c);
        } else {
            break;
        }
    }
    *input = &input[sym.len()..];
    Obj::Sym(sym)
}

fn is_sym_char(c: char) -> bool {
    match c {
        '_' | '-' => true,
        c => c.is_ascii_alphabetic(),
    }
}

#[cfg(test)]
mod parse_expr_tests {
    use super::*;

    #[test]
    fn return_none_ast_with_empty_input() {
        let mut input = "";
        let result = parse_expr(&mut input);
        assert_eq!(result, Obj::None);
        assert_eq!(input, "");
    }

    #[test]
    fn parse_number() {
        let mut input = "42";
        let result = parse_expr(&mut input);
        assert_eq!(result, Obj::Num(42));
        assert_eq!(input, "");
    }

    #[test]
    fn parse_symbol() {
        let mut input = "this_is-sym";
        let result = parse_expr(&mut input);
        assert_eq!(result, Obj::Sym("this_is-sym".to_string()));
        assert_eq!(input, "");
    }
}
