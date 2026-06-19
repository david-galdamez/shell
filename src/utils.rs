use std::{fs::File, io::Write, path::PathBuf};

use crate::redirect::Redirect;

#[derive(Debug)]
pub struct Input {
    pub cmd: String,
    pub args: Vec<String>,
    pub operator: Option<String>,
    pub operator_args: Vec<String>,
}

impl Input {
    fn new() -> Self {
        Input {
            cmd: String::new(),
            args: vec![],
            operator: None,
            operator_args: vec![],
        }
    }

    fn parse(input: Vec<String>) -> Option<Input> {
        if input.is_empty() {
            return None;
        }

        let mut parsed_input = Input::new();

        if input.len() == 1 {
            parsed_input.cmd = input[0].clone();
            return Some(parsed_input);
        }

        for (i, arg) in input.iter().enumerate() {
            if i == 0 {
                parsed_input.cmd = arg.clone();
                continue;
            }

            if arg == ">" || arg == "1>" {
                parsed_input.operator = Some(arg.clone());
                continue;
            }

            if arg == "2>" {
                parsed_input.operator = Some(arg.clone());
                continue;
            }

            match parsed_input.operator {
                Some(_) => parsed_input.operator_args.push(arg.clone()),
                None => parsed_input.args.push(arg.clone()),
            }
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
    let mut backlash = false;

    for c in input.chars() {
        if c == '\\' && !backlash && !single_quote {
            backlash = true;
            continue;
        }

        if backlash {
            arg.push(c);
            backlash = false;
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

pub fn handle_stdout(redirect: Redirect, operator: Option<String>, operator_args: Vec<String>) {
    let output = redirect.stdout.unwrap_or_default();
    let err = redirect.stderr.unwrap_or_default();

    if let Some(op) = operator {
        if op == ">" || op == "1>" {
            if !err.is_empty() {
                eprintln!("{}", err);
            }
            write_to_file(&output, operator_args);
        } else if op == "2>" {
            if !output.is_empty() {
                println!("{}", output);
            }
            write_to_file(&err, operator_args);
        }
    } else {
        if err.is_empty() {
            println!("{output}");
        } else {
            eprintln!("{err}");
        }
    }
}

fn write_to_file(output: &str, operator_args: Vec<String>) {
    let file_path = match operator_args.first() {
        Some(name) => name,
        None => {
            eprintln!("File not provided");
            return;
        }
    };

    let mut path = PathBuf::new();
    path.push(file_path);

    let mut file = match File::create(path) {
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
