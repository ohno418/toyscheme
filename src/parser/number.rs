pub(super) fn parse_number(input: &mut &str) -> Result<u64, String> {
    let mut num_str = String::new();
    while let Some(c) = input.chars().next() {
        if c.is_digit(10) {
            num_str.push(c);
            *input = &input[1..];
        } else {
            break;
        }
    }
    num_str.parse::<u64>().map_err(|_| {
        format!(
            "cannot parse a number from \"{}\"",
            input.chars().collect::<String>()
        )
    })
}

#[cfg(test)]
mod parse_number_tests {
    use super::parse_number;

    #[test]
    fn parse_a_number() {
        let mut s = "42";
        let result = parse_number(&mut s);
        assert_eq!(result, Ok(42));
        assert_eq!(s, "");
    }

    #[test]
    fn parse_a_number_with_extra_input() {
        let mut s = "42 hello";
        let result = parse_number(&mut s);
        assert_eq!(result, Ok(42));
        assert_eq!(s, " hello");
    }

    #[test]
    fn cannot_parse_negative_number() {
        let mut s = "-42 hello";
        let result = parse_number(&mut s);
        assert_eq!(
            result,
            Err("cannot parse a number from \"-42 hello\"".to_string())
        );
        assert_eq!(s, "-42 hello");
    }

    #[test]
    fn cannot_parse_non_number() {
        let mut s = "hello";
        let result = parse_number(&mut s);
        assert_eq!(
            result,
            Err("cannot parse a number from \"hello\"".to_string())
        );
        assert_eq!(s, "hello");
    }
}
