use std::iter::Peekable;
use std::str::Chars;

pub(super) fn read_number(input: &mut Peekable<Chars>) -> Result<u64, String> {
    let mut num_str = String::new();
    while let Some(c) = input.peek() {
        if c.is_digit(10) {
            let c = input.next().unwrap();
            num_str.push(c);
        } else {
            break;
        }
    }
    num_str.parse::<u64>().map_err(|_| {
        format!(
            "cannot read a number from \"{}\"",
            input.clone().collect::<String>()
        )
    })
}

#[cfg(test)]
mod read_number_tests {
    use super::read_number;

    #[test]
    fn read_a_number_from_chars() {
        let s = "42 hello".to_string();
        let mut chars = s.chars().peekable();
        let result = read_number(&mut chars);
        assert_eq!(result, Ok(42));
        assert_eq!(chars.collect::<String>(), " hello".to_string());
    }

    #[test]
    fn cannot_read_negative_number() {
        let s = "-42 hello".to_string();
        let mut chars = s.chars().peekable();
        let result = read_number(&mut chars);
        assert_eq!(
            result,
            Err("cannot read a number from \"-42 hello\"".to_string())
        );
        assert_eq!(chars.collect::<String>(), "-42 hello".to_string());
    }

    #[test]
    fn cannot_read_non_number() {
        let s = "hello".to_string();
        let mut chars = s.chars().peekable();
        let result = read_number(&mut chars);
        assert_eq!(
            result,
            Err("cannot read a number from \"hello\"".to_string())
        );
        assert_eq!(chars.collect::<String>(), "hello".to_string());
    }
}
