use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use is_executable::is_executable;
use walkdir::WalkDir;

use crate::utils::redirect_stdout;

pub fn echo(args: Vec<String>, operator: String, operator_args: Vec<String>) {
    let args: Vec<String> = args
        .iter()
        .filter(|a| *a != " ")
        .map(|a| format!("{}", a))
        .collect();

    redirect_stdout(args.join(" "), operator, operator_args);
}

pub fn type_output(arg: &str, operator: String, operator_args: Vec<String>) {
    match arg {
        "echo" => {
            redirect_stdout(format!("{arg} is a shell builtin"), operator, operator_args);
        }
        "exit" => {
            redirect_stdout(format!("{arg} is a shell builtin"), operator, operator_args);
        }
        "type" => {
            redirect_stdout(format!("{arg} is a shell builtin"), operator, operator_args);
        }
        "pwd" => {
            redirect_stdout(format!("{arg} is a shell builtin"), operator, operator_args);
        }
        other => type_executable(other, operator, operator_args),
    }
}

fn type_executable(arg: &str, operator: String, operator_args: Vec<String>) {
    let path_str = match env::var("PATH").ok() {
        Some(path) => path,
        None => {
            eprintln!("PATH variable not found in env");
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
                    let out = format!("{} is {}", arg, entry.path().display());
                    redirect_stdout(out, operator, operator_args);
                    return;
                }
            }
        }
    }

    eprintln!("{}: not found", arg);
}

pub fn execute_file(cmd: String, args: Vec<String>, operator: String, operator_args: Vec<String>) {
    let path_str = match env::var("PATH").ok() {
        Some(path) => path,
        None => {
            eprintln!("PATH variable not found in env");
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
                    if operator.is_empty() {
                        Command::new(entry.file_name())
                            .args(&args)
                            .status()
                            .expect("Failed to execute command");
                    } else {
                        let output = Command::new(entry.file_name())
                            .args(&args)
                            .output()
                            .expect("Failed to execute command");
                        let output = String::from_utf8_lossy(&output.stdout);
                        redirect_stdout(output.trim().to_string(), operator, operator_args);
                    }
                    return;
                }
            }
        }
    }

    eprintln!("{}: not found", cmd);
}

pub fn pwd(operator: String, operator_args: Vec<String>) {
    let current_dir = match env::current_dir().ok() {
        Some(dir) => dir,
        None => PathBuf::new(),
    };
    redirect_stdout(
        format!("{}", current_dir.display()),
        operator,
        operator_args,
    );
}

pub fn cd(arg: &str) {
    if arg.starts_with("/") {
        cd_absolute(arg);
        return;
    }

    if arg.starts_with("~") {
        cd_home_dir(arg);
        return;
    }

    cd_relative(arg);
}

fn cd_absolute(arg: &str) {
    let path = Path::new(arg);

    if !path.exists() {
        eprintln!("cd: {}: No such file or directory", arg);
        return;
    }

    match env::set_current_dir(path) {
        Ok(_) => return,
        Err(e) => eprintln!("{e}"),
    }
}

fn cd_relative(arg: &str) {
    let mut current_path = match env::current_dir().ok() {
        Some(path) => path,
        None => PathBuf::new(),
    };
    for dir in arg.split("/") {
        if dir == "." || dir == "~" {
            continue;
        }

        if dir == ".." {
            current_path.pop();
        } else {
            current_path.push(dir);
            if !current_path.exists() {
                current_path.pop();
                eprintln!("cd: {}: No such file or directory", dir);
                return;
            }
        }
    }

    match env::set_current_dir(current_path) {
        Ok(_) => return,
        Err(e) => eprintln!("{e}"),
    };
}

fn cd_home_dir(arg: &str) {
    let home = match std::env::home_dir() {
        Some(home) => home,
        None => PathBuf::new(),
    };

    match env::set_current_dir(home) {
        Ok(_) => cd_relative(arg),
        Err(e) => eprintln!("{e}"),
    }
}
