use super::number::parse_number;
use super::Ast;

/// Parses an expression, including a definition.
pub(super) fn parse_expr(input: &mut &str) -> Ast {
    match input.chars().next() {
        Some(c) => {
            if c.is_digit(10) || c == '-' {
                // number
                parse_number(input)
            } else if is_sym_char(c) {
                // symbol
                parse_symbol(input)
            } else if c == '\'' {
                *input = &input[1..];
                Ast::Quote(Box::new(parse_expr(input)))
            } else {
                Ast::Err("unknown input".to_string())
            }
        }
        None => Ast::None,
    }

    // TODO: Check if extra input.
}

fn parse_symbol(input: &mut &str) -> Ast {
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
    Ast::Sym(sym)
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
        assert_eq!(result, Ast::None);
        assert_eq!(input, "");
    }

    #[test]
    fn parse_number() {
        let mut input = "42";
        let result = parse_expr(&mut input);
        assert_eq!(result, Ast::Num(42));
        assert_eq!(input, "");
    }

    #[test]
    fn parse_symbol() {
        let mut input = "this_is-sym";
        let result = parse_expr(&mut input);
        assert_eq!(result, Ast::Sym("this_is-sym".to_string()));
        assert_eq!(input, "");
    }

    #[test]
    fn parse_quoted_symbol() {
        let mut input = "'this_is-sym";
        let result = parse_expr(&mut input);
        assert_eq!(
            result,
            Ast::Quote(Box::new(Ast::Sym("this_is-sym".to_string())))
        );
        assert_eq!(input, "");
    }

    #[test]
    fn parse_quoted_number() {
        let mut input = "'123";
        let result = parse_expr(&mut input);
        assert_eq!(result, Ast::Quote(Box::new(Ast::Num(123))));
        assert_eq!(input, "");
    }
}
