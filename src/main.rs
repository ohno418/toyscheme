use std::io::{stdin, stdout, Write};
use toyscheme::*;

fn main() {
    loop {
        let input = read();
        print(eval(&input));
    }
}

fn read() -> String {
    print!("> ");
    stdout().flush().expect("stdout flush failed");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("read_line() failed");
    input
}

fn print(ast: Ast) {
    match ast {
        Ast::Num(num) => println!("{num}"),
        Ast::None => println!(""),
        Ast::Err(msg) => println!("error: {msg}"),
    };
}
