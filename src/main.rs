use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("stdout flush failed");

        let mut input = String::new();
        let nread = stdin().read_line(&mut input).expect("read_line() failed");

        // Do not print if only a newline ("\n") is read.
        if nread > 1 {
            print!("{input}");
        }
    }
}
