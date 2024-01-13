use std::io::{stdin, stdout, Write};
use toyscheme::*;

fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("stdout flush failed");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("read_line() failed");

        match eval_expr(&input) {
            ExprResult::Num(num) => println!("{num}"),
            ExprResult::Nop => println!(""),
            ExprResult::Err(msg) => println!("error: {msg}"),
        };
    }
}
