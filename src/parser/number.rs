use super::Ast;

pub(super) fn parse_number(input: &mut &str) -> Ast {
    let mut num_str = String::new();
    let mut chars = input.chars();
    let mut is_first = true;
    while let Some(c) = chars.next() {
        if is_first {
            is_first = false;

            if c == '-' {
                num_str.push(c);
                continue;
            }
        }

        if c.is_digit(10) {
            num_str.push(c);
        } else {
            break;
        }
    }

    match num_str.parse::<i64>() {
        Ok(num) => {
            // Consume the input only when parsing has succeeded.
            *input = &input[num_str.len()..];
            Ast::Num(num)
        }
        Err(_) => Ast::Err(format!(
            "cannot parse a number from \"{}\"",
            input.chars().collect::<String>()
        )),
    }
}

#[cfg(test)]
mod parse_number_tests {
    use super::*;

    #[test]
    fn parse_a_number() {
        let mut s = "42";
        let result = parse_number(&mut s);
        assert_eq!(result, Ast::Num(42));
        assert_eq!(s, "");
    }

    #[test]
    fn parse_a_number_with_extra_input() {
        let mut s = "42 hello";
        let result = parse_number(&mut s);
        assert_eq!(result, Ast::Num(42));
        assert_eq!(s, " hello");
    }

    #[test]
    fn parse_negative_number() {
        let mut s = "-42 hello";
        let result = parse_number(&mut s);
        assert_eq!(result, Ast::Num(-42));
        assert_eq!(s, " hello");
    }

    #[test]
    fn cannot_parse_non_number() {
        let mut s = "hello";
        let result = parse_number(&mut s);
        assert_eq!(
            result,
            Ast::Err("cannot parse a number from \"hello\"".to_string())
        );
        assert_eq!(s, "hello");
    }

    #[test]
    fn cannot_parse_too_big_number() {
        let mut s = "9999999999999999999999999";
        let result = parse_number(&mut s);
        assert_eq!(
            result,
            Ast::Err("cannot parse a number from \"9999999999999999999999999\"".to_string())
        );
        assert_eq!(s, "9999999999999999999999999");
    }
}
