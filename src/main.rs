#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;
mod utils;

fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        let input = match utils::parse_input(&input.trim()) {
            Some(input) => input,
            None => continue,
        };
        let cmd = input.cmd;
        let args = input.args;
        let operator = input.operator;
        let operator_args = input.operator_args;

        match cmd.as_str() {
            "exit" => break,
            "echo" => commands::echo(args),
            "type" => {
                let arg = match args.get(0) {
                    Some(arg) => arg,
                    None => continue,
                };
                commands::type_output(arg);
            }
            "pwd" => commands::pwd(),
            "cd" => {
                let arg = match args.get(0) {
                    Some(arg) => arg,
                    None => continue,
                };

                commands::cd(arg);
            }
            _ => commands::execute_file(cmd, args),
        }
    }
}

fn main() {
    run();
}
