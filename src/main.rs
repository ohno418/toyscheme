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

fn print(parse: ExprResult) {
    match parse {
        ExprResult::Num(num) => println!("{num}"),
        ExprResult::Nop => println!(""),
        ExprResult::Err(msg) => println!("error: {msg}"),
    };
}
