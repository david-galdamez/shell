#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("$ ");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line");
    println!("{}: command not found", input);
    io::stdout().flush().unwrap();
}
