#[derive(Debug)]
pub struct Input {
    pub cmd: String,
    pub args: Vec<String>,
    pub operator: String,
    pub operator_args: Vec<String>
}

impl Input {
    fn new() -> Self {
        Input { cmd: String::new(), args: vec![], operator: String::new(), operator_args: vec![] }
    }

    fn input(input: Vec<String>) -> Option<Input> {
        if input.len() == 0 {
            return None;
        }

        let mut parsed_input = Input::new();

        if input.len() == 1 {
            parsed_input.cmd = input[0].clone();
            return Some(
                parsed_input
            )
        }

        for (i, arg) in input.iter().enumerate() {
            if i == 0 {
                parsed_input.cmd = arg.clone();
                continue;
            }

            if arg == ">" || arg == "1>" {
                parsed_input.operator = arg.clone();
                continue;
            }

            if parsed_input.operator.is_empty() {
                parsed_input.args.push(arg.clone());
            } else {
                parsed_input.operator_args.push(arg.clone()); 
            }
        }

        Some(
            parsed_input
        )
    }
}

pub fn parse_input(input: &str) -> Option<Input> {
    let tokenized_input = tokenize_args(input);
    let input = Input::input(tokenized_input);

    input
}

fn tokenize_args(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut arg = String::new();
    let mut single_quote_counter = 0;
    let mut double_quote_counter = 0;
    let mut backlash = false;

    for c in input.chars() {

        if c == '\\' && !backlash && single_quote_counter == 0  {
            backlash = true;
            continue;
        }

        if backlash {
            arg.push(c);
            backlash = false;
            continue
        }

        if c == '"' && double_quote_counter == 0 && single_quote_counter == 0 {
            double_quote_counter += 1;
            continue;
        }

        if c == '"' && double_quote_counter != 0 && single_quote_counter == 0 {
            double_quote_counter -= 1;
            continue;
        }

        if c == '\'' && single_quote_counter == 0 && double_quote_counter == 0 {
            single_quote_counter += 1;
            continue;
        }

        if c == '\'' && single_quote_counter != 0 && double_quote_counter == 0 {
            single_quote_counter -= 1;
            continue;
        }

        if c == ' ' && single_quote_counter == 0  && double_quote_counter == 0 {
            if !arg.is_empty() {
                args.push(arg.to_string());
                arg.clear();
            }
            continue;
        }
        arg.push(c);
    }

    args.push(arg);

    args
}

fn redirect_stdout(stdout: String, operator: String, operator_args: Vec<String>) {
    if operator.is_empty() {
        println!("{stdout}")
    }

    if operator == ">" || operator == "1>"     {
        let file_name = match operator_args.get(0) {
            Some(name) => name,
            None => {
                println!("Error parsing");
                return;
            }
        };


    }
}