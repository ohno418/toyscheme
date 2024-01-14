use crate::types::Obj;

pub(super) fn eval_ast(ast: Obj) -> Obj {
    match ast {
        Obj::Sym(sym) => {
            // Evaluating a symbol is not supported for now.
            Obj::Err(format!("unbound variable: {sym}"))
        }
        _ => ast,
    }
}

#[cfg(test)]
mod eval_tests {
    use super::*;

    #[test]
    fn eval_num() {
        let ast = Obj::Num(42);
        let result = eval_ast(ast);
        assert_eq!(result, Obj::Num(42));
    }

    #[test]
    fn eval_symbol() {
        let ast = Obj::Sym("sym".to_string());
        let result = eval_ast(ast);
        assert_eq!(result, Obj::Err("unbound variable: sym".to_string()));
    }

    #[test]
    fn eval_none() {
        let ast = Obj::None;
        let result = eval_ast(ast);
        assert_eq!(result, Obj::None);
    }

    #[test]
    fn eval_err() {
        let ast = Obj::Err("some error!".to_string());
        let result = eval_ast(ast);
        assert_eq!(result, Obj::Err("some error!".to_string()));
    }
}
