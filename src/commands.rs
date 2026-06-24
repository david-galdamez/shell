use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use is_executable::is_executable;
use walkdir::WalkDir;

use crate::{
    redirect::{OutputTarget, Redirect},
    utils::handle_stdout,
};

const BUILTINS: [&str; 6] = ["echo", "type", "pwd", "exit", "cd", "complete"];

pub fn echo(args: Vec<String>, output_target: Option<OutputTarget>) {
    let args: Vec<String> = args
        .iter()
        .filter(|a| *a != " ")
        .map(|a| a.to_string())
        .collect();

    let mut output = Redirect::new();
    output.stdout = Some(args.join(" ").to_string());

    handle_stdout(output, output_target);
}

pub fn type_output(cmd: String, args: Vec<String>, output_target: Option<OutputTarget>) {
    let mut output = Redirect::new();
    let arg = match args.first() {
        Some(arg) => arg,
        None => {
            output.stderr = Some("You have to pass an argument".to_string());
            ""
        }
    };

    if BUILTINS.contains(&arg) {
        output.stdout = Some(format!("{} is a shell builtin", arg));
        handle_stdout(output, output_target);
    } else {
        executables(cmd, args, output_target);
    }
}

pub fn executables(cmd: String, args: Vec<String>, output_target: Option<OutputTarget>) {
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

                if cmd == "type" {
                    if let Some(arg) = args.first()
                        && arg
                            == entry
                                .file_name()
                                .to_str()
                                .expect("Expected valid file name")
                    {
                        let mut output = Redirect::new();
                        output.stdout = Some(format!("{} is {}", arg, entry.path().display()));
                        handle_stdout(output, output_target);
                        return;
                    }
                } else if cmd
                    == entry
                        .file_name()
                        .to_str()
                        .expect("Expected valid file name")
                {
                    if let Some(ot) = output_target {
                        match ot.operator.as_str() {
                            "|" => {
                                let mut command_1 = match Command::new(entry.file_name())
                                    .args(&args)
                                    .stdout(Stdio::piped())
                                    .spawn()
                                {
                                    Ok(cmd) => cmd,
                                    Err(e) => {
                                        eprintln!("{e}");
                                        return;
                                    }
                                };

                                let stdout_1 = match command_1.stdout.take() {
                                    Some(stdout) => stdout,
                                    None => {
                                        eprintln!("Failed to capture stdout of first command");
                                        return;
                                    }
                                };
                                let new_command = match ot.args.first() {
                                    Some(command) => command,
                                    None => {
                                        eprintln!("You have to pass a new command");
                                        return;
                                    }
                                };

                                let command_2 = match Command::new(new_command)
                                    .args(&ot.args[1..])
                                    .stdin(Stdio::from(stdout_1))
                                    .stdout(Stdio::piped())
                                    .spawn()
                                {
                                    Ok(cmd) => cmd,
                                    Err(e) => {
                                        eprintln!("{e}");
                                        return;
                                    }
                                };

                                let output_2 = command_2
                                    .wait_with_output()
                                    .expect("Failed to capture stdout of second command");

                                let _ = command_1.kill();
                                let _ = command_1.wait();
                                eprintln!(
                                    "DEBUG stdout: {:?}",
                                    String::from_utf8_lossy(&output_2.stdout)
                                );
                                eprintln!(
                                    "DEBUG stderr: {:?}",
                                    String::from_utf8_lossy(&output_2.stderr)
                                );
                                eprintln!("DEBUG status: {:?}", output_2.status);

                                let mut output = Redirect::new();
                                output.stdout = Some(
                                    String::from_utf8_lossy(&output_2.stdout)
                                        .trim_end_matches("\n")
                                        .to_string(),
                                );
                                if !output_2.status.success() {
                                    output.stderr = Some(
                                        String::from_utf8_lossy(&output_2.stderr)
                                            .trim_end_matches("\n")
                                            .to_string(),
                                    )
                                }
                                handle_stdout(output, None);
                            }
                            _ => {
                                let command_output = Command::new(entry.file_name())
                                    .args(&args)
                                    .output()
                                    .expect("Failed to execute process");

                                let mut output = Redirect::new();
                                output.stdout = Some(
                                    String::from_utf8_lossy(&command_output.stdout)
                                        .trim()
                                        .to_string(),
                                );

                                if !command_output.status.success() {
                                    output.stderr = Some(
                                        String::from_utf8_lossy(&command_output.stderr)
                                            .trim()
                                            .to_string(),
                                    );
                                }
                                handle_stdout(output, Some(ot));
                            }
                        }
                    } else {
                        Command::new(entry.file_name())
                            .args(&args)
                            .status()
                            .expect("Failed to execute process");
                    }
                    return;
                }
            }
        }
    }

    let mut output = Redirect::new();

    if cmd == "type" {
        output.stderr = Some(format!("{}: not found", args[0]));
    } else {
        output.stderr = Some(format!("{}: not found", cmd));
    }

    handle_stdout(output, output_target);
}

pub fn pwd(output_target: Option<OutputTarget>) {
    let current_dir = match env::current_dir().ok() {
        Some(dir) => dir,
        None => PathBuf::new(),
    };
    let mut output = Redirect::new();
    output.stdout = Some(format!("{}", current_dir.display()));
    handle_stdout(output, output_target);
}

pub fn cd(arg: &str) {
    let mut string_path = String::from(arg);
    if arg.starts_with("~") {
        let home = match std::env::home_dir() {
            Some(home) => home,
            None => PathBuf::new(),
        };

        if let Some(home_path) = home.to_str() {
            let splitted_path: Vec<&str> = arg.splitn(2, '/').collect();
            let relative_path = match splitted_path.get(1) {
                Some(path) => path,
                None => "",
            };
            string_path = format!("{}/{}", home_path, relative_path);
        }
    }

    let path = Path::new(&string_path);

    if !path.exists() {
        eprintln!("cd: {}: No such file or directory", arg);
        return;
    }

    match env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}
