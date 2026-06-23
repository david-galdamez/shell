use std::io::{self, Write};

mod commands;
mod redirect;
mod utils;

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
        let output_target = input.output_target;

        match cmd.as_str() {
            "exit" => break,
            "echo" => commands::echo(args, output_target),
            "type" => {
                commands::type_output(cmd, args, output_target);
            }
            "pwd" => commands::pwd(output_target),
            "cd" => {
                let arg = match args.first() {
                    Some(arg) => arg,
                    None => continue,
                };

                commands::cd(arg);
            }
            _ => commands::executables(cmd, args, output_target),
        }
    }
}

fn main() {
    run();
}
