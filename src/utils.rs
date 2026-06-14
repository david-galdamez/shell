use std::{env, fs, path::Path};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct Input<'a> {
    pub cmd: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Input<'a> {
    fn new(cmd: &'a str, args: Vec<&'a str>) -> Self {
        Input { cmd, args }
    }
}

pub fn parse_input(input: &str) -> Option<Input<'_>> {
    let args: Vec<&str> = Vec::new();
    let input: Vec<&str> = input.split_whitespace().collect();
    if input.len() == 0 {
        return None;
    }

    if input.len() == 1 {
        return Some(Input::new(input[0], args));
    }

    Some(Input::new(input[0], input[1..].to_vec()))
}

pub fn builtin_output(arg: &str) {
    println!("{arg} is a shell builtin");
}

pub fn type_executable(arg: &str) {
    let path_str = match env::var("PATH").ok() {
        Some(path) => path,
        None => {
            println!("PATH variable not found in env");
            return;
        }
    };

    for path in env::split_paths(&path_str) {
        if Path::exists(&path) {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if arg == entry.file_name().to_str().unwrap() {
                    println!("{} is {}", arg, entry.into_path().to_str().unwrap());
                    return;
                }
            }
        }
    }

    println!("{}: not found", arg);
}
