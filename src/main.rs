#[allow(unused_imports)]
use std::io::{self, Write};

fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn builtin_output(arg: &str) {
    println!("{arg} is a shell builtin");
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        let input = parse_input(&input);
        let command = match input.get(0) {
            Some(cmd) => cmd.trim(),
            None => continue,
        };

        if command == "exit" {
            break;
        } else if command == "echo" {
            println!("{}", input[1..].join(" "))
        } else if command == "type" {
            let arg = match input.get(1) {
                Some(arg) => arg.trim(),
                None => continue,
            };

            match arg {
                "echo" => builtin_output(arg),
                "exit" => builtin_output(arg),
                "type" => builtin_output(arg),
                other => println!("{other}: not found"),
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
