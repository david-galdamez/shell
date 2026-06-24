use rustyline::{Editor, Result};

use crate::helper::ShellHelper;

mod commands;
mod helper;
mod redirect;
mod utils;

fn run() -> Result<()> {
    let helper = ShellHelper::default();
    let mut rl: Editor<ShellHelper, _> = Editor::new()?;
    rl.set_helper(Some(helper));
    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(input) => {
                let input = match utils::parse_input(input.trim()) {
                    Some(input) => input,
                    None => continue,
                };
                let cmd = input.cmd;
                let args = input.args;
                let output_target = input.output_target;

                match cmd.as_str() {
                    "exit" => return Ok(()),
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
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                continue;
            }
        }
    }
}

fn main() {
    run().expect("Error initializing shell");
}
