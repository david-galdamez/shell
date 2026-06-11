#[allow(unused_imports)]
use std::io::{self, Write};

fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        let command = parse_input(&input);
        if command[0].trim() == "exit" {
            break;
        } else if command[0].trim() == "echo" {
            println!("{}", command[1..].join(" "))
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
