#[allow(unused_imports)]
use std::io::{self, Write};

mod utils;

fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        let input = match utils::parse_input(&input) {
            Some(input) => input,
            None => continue,
        };
        let cmd = input.cmd;
        let args = input.args;

        if cmd == "exit" {
            break;
        } else if cmd == "echo" {
            println!("{}", args.join(" "))
        } else if cmd == "type" {
            let arg = match args.get(0) {
                Some(arg) => *arg,
                None => continue,
            };
            match arg {
                "echo" => utils::builtin_output(arg),
                "exit" => utils::builtin_output(arg),
                "type" => utils::builtin_output(arg),
                other => println!("{other}: not found"),
            }
        } else {
            println!("{}: command not found", cmd);
        }
    }
}

fn main() {
    run();
}
