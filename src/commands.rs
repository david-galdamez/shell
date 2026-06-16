use std::{
    env,
    path::{self, Path, PathBuf},
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
    let current_dir = match env::current_dir().ok() {
        Some(dir) => dir,
        None => PathBuf::new(),
    };
    println!("{}", current_dir.display());
}

pub fn cd(arg: &str) {
    if arg.starts_with("/") {
        cd_absolute(arg);
        return;
    }

    cd_relative(arg);
}

fn cd_absolute(arg: &str) {
    let path = Path::new(arg);

    if !path.exists() {
        println!("cd: {}: No such file or directory", arg);
        return;
    }

    match env::set_current_dir(path) {
        Ok(_) => return,
        Err(e) => println!("{e}"),
    }
}

fn cd_relative(arg: &str) {
    let mut current_path = match env::current_dir().ok() {
        Some(path) => path,
        None => PathBuf::new(),
    };
    for dir in arg.split("/") {
        if dir == "." {
            continue;
        }

        if dir == ".." {
            current_path.pop();
        } else {
            current_path.push(dir);
            if !current_path.exists() {
                current_path.pop();
                println!("cd: {}: No such file or directory", dir);
                return;
            }
        }
    }

    match env::set_current_dir(current_path) {
        Ok(_) => return,
        Err(e) => println!("{e}"),
    };
}
