use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("stdout flush failed");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("read_line() failed");

        print!("{input}");
    }
}
