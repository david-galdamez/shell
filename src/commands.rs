use std::{
    env,
    path::{self, Path},
    process::Command,
};

use is_executable::is_executable;
use walkdir::WalkDir;

pub fn type_output(arg: &str) {
    match arg {
        "echo" => println!("{arg} is a shell builtin"),
        "exit" => println!("{arg} is a shell builtin"),
        "type" => println!("{arg} is a shell builtin"),
        "pwd" => println!("{arg} is a shell builtin"),
        other => type_executable(other),
    }
}

fn type_executable(arg: &str) {
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
                if !is_executable(entry.path()) {
                    continue;
                }

                if arg == entry.file_name().to_str().unwrap() {
                    println!("{} is {}", arg, entry.path().display());
                    return;
                }
            }
        }
    }

    println!("{}: not found", arg);
}

pub fn execute_file(cmd: &str, args: Vec<&str>) {
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
                if !is_executable(entry.path()) {
                    continue;
                }

                if cmd == entry.file_name().to_str().expect("Expected valid file") {
                    let mut status = Command::new(entry.file_name());
                    for arg in &args {
                        status.arg(arg);
                    }
                    status.status().expect("Failed to execute command");

                    return;
                }
            }
        }
    }

    println!("{}: not found", cmd);
}

pub fn pwd() {
    let path = Path::new(".");
    let abs = match path::absolute(path) {
        Ok(path) => path,
        Err(_) => panic!("Error getting current path"),
    };
    println!("{}", abs.display());
}
