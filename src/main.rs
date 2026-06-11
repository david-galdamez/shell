#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line");
    print!("$ ");
    println!("{}: command not found", input.trim());
    io::stdout().flush().unwrap();
}
