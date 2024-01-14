use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Obj {
    /// Number.
    Num(i64),
    /// Symbol.
    Sym(String),
    /// No expression to evaluate found.
    None,
    /// Error.
    Err(String),
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Obj::Num(num) => write!(f, "{}", num),
            Obj::Sym(sym) => write!(f, "{}", sym),
            Obj::None => write!(f, ""),
            Obj::Err(msg) => write!(f, "error: {}", msg),
        }
    }
}

#[cfg(test)]
mod display_obj_tests {
    use super::*;

    #[test]
    fn display_num() {
        let obj = Obj::Num(42);
        assert_eq!(obj.to_string(), "42".to_string());
    }

    #[test]
    fn display_symbol() {
        let obj = Obj::Sym("sym".to_string());
        assert_eq!(obj.to_string(), "sym".to_string());
    }

    #[test]
    fn display_none() {
        let obj = Obj::None;
        assert_eq!(obj.to_string(), "".to_string());
    }

    #[test]
    fn display_err() {
        let obj = Obj::Err("some error".to_string());
        assert_eq!(obj.to_string(), "error: some error".to_string());
    }
}
