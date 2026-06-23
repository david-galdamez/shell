use std::{
    fs::File,
    io::Write,
    path::Path,
};

use crate::redirect::{Action, OutputTarget, Redirect};

#[derive(Debug)]
pub struct Input {
    pub cmd: String,
    pub args: Vec<String>,
    pub output_target: Option<OutputTarget>,
}

impl Input {
    fn new() -> Self {
        Input {
            cmd: String::new(),
            args: vec![],
            output_target: None,
        }
    }

    fn parse(input: Vec<String>) -> Option<Input> {
        if input.is_empty() {
            return None;
        }

        let mut parsed_input = Input::new();
        let mut output_target = OutputTarget::new();

        if input.len() == 1 {
            parsed_input.cmd = input[0].clone();
            return Some(parsed_input);
        }

        for (i, arg) in input.iter().enumerate() {
            if i == 0 {
                parsed_input.cmd = arg.clone();
                continue;
            }

            if arg == ">" || arg == "1>" || arg == "2>" {
                output_target.operator = arg.clone();
                output_target.action = Action::Redirect;
                continue;
            }

            if arg == ">>" || arg == "1>>" || arg == "2>>" {
                output_target.operator = arg.clone();
                output_target.action = Action::Append;
                continue;
            }

            if output_target.operator.is_empty() {
                parsed_input.args.push(arg.clone());
            } else {
                output_target.args.push(arg.clone());
            }
        }

        if !output_target.args.is_empty() && !output_target.operator.is_empty() {
            parsed_input.output_target = Some(output_target);
        }

        Some(parsed_input)
    }
}

pub fn parse_input(input: &str) -> Option<Input> {
    let tokenized_input = tokenize_args(input);
    Input::parse(tokenized_input)
}

fn tokenize_args(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut arg = String::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut backslash = false;

    for c in input.chars() {
        if c == '\\' && !backslash && !single_quote {
            backslash = true;
            continue;
        }

        if backslash {
            arg.push(c);
            backslash = false;
            continue;
        }

        if c == '"' && !double_quote && !single_quote {
            double_quote = true;
            continue;
        }

        if c == '"' && double_quote && !single_quote {
            double_quote = false;
            continue;
        }

        if c == '\'' && !single_quote && !double_quote {
            single_quote = true;
            continue;
        }

        if c == '\'' && single_quote && !double_quote {
            single_quote = false;
            continue;
        }

        if c == ' ' && !single_quote && !double_quote {
            if !arg.is_empty() {
                args.push(arg.to_string());
                arg.clear();
            }
            continue;
        }
        arg.push(c);
    }

    if !arg.is_empty() {
        args.push(arg);
    }

    args
}

pub fn handle_stdout(redirect: Redirect, output_target: Option<OutputTarget>) {
    let mut output = redirect.stdout.unwrap_or_default();
    let mut err = redirect.stderr.unwrap_or_default();

    if let Some(ot) = &output_target {
        if ot.operator == ">" || ot.operator == "1>" || ot.operator == ">>" || ot.operator == "1>>" {
            if !err.is_empty() {
                eprintln!("{}", err);
            }
            if !output.is_empty() {
                output.push('\n');
            }
            write_to_file(&output, ot);
        } else if ot.operator == "2>" || ot.operator == "2>>" {
            if !output.is_empty() {
                println!("{}", output);
            }
            if !err.is_empty() {
                err.push('\n');
            }
            write_to_file(&err, ot);
        }
    } else {
        if err.is_empty() {
            println!("{output}");
        } else {
            eprintln!("{err}");
        }
    }
}

fn write_to_file(output: &str, output_target: &OutputTarget) {
    let file_path = match output_target.args.first() {
        Some(name) => name,
        None => {
            eprintln!("File name not provided");
            return;
        }
    };

    let path = Path::new(file_path);
    let mut file = File::options();
    match output_target.action {
        Action::Append => {
            file.append(true);
        }
        Action::Redirect => {
            file.write(true);
        }
    }
    let mut file = match file.create(true).open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    match file.write_all(output.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
        }
    };
}
