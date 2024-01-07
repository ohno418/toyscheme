use std::io::{stdin, stdout, Write};
use toyscheme::read_expr;

fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("stdout flush failed");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("read_line() failed");

        match read_expr(&input) {
            Ok(output) => println!("{output}"),
            Err(msg) => println!("error: {msg}"),
        };
    }
}
