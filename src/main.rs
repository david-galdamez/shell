use std::io::{self, Write};

mod commands;
mod utils;
mod redirect;

fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        }

        let input = match utils::parse_input(input.trim()) {
            Some(input) => input,
            None => continue,
        };
        let cmd = input.cmd;
        let args = input.args;
        let operator = input.operator;
        let operator_args = input.operator_args;

        match cmd.as_str() {
            "exit" => break,
            "echo" => commands::echo(args, operator, operator_args),
            "type" => {
                commands::type_output(cmd, args, operator, operator_args);
            }
            "pwd" => commands::pwd(operator, operator_args),
            "cd" => {
                let arg = match args.first() {
                    Some(arg) => arg,
                    None => continue,
                };

                commands::cd(arg);
            }
            _ => commands::executables(cmd, args, operator, operator_args),
        }
    }
}

fn main() {
    run();
}
