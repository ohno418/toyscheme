use std::io::{stdin, stdout, Write};
use toyscheme::*;

fn main() {
    loop {
        let input = read();
        let eval_result = eval(&input);
        println!("{}", eval_result);
    }
}

fn read() -> String {
    print!("> ");
    stdout().flush().expect("stdout flush failed");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("read_line() failed");
    input
}
