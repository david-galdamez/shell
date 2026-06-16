#[derive(Debug)]
pub struct Input {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Input {
    fn new(cmd: String, args: Vec<String>) -> Self {
        Input { cmd, args }
    }
}

pub fn parse_input(input: &str) -> Option<Input> {
    let input = tokenize_args(input);
    if input.len() == 0 {
        return None;
    }


    if input.len() == 1 {
        return Some(Input::new(input[0].clone(), Vec::new()));
    }

    Some(Input::new(input[0].clone(), input[1..].to_vec()))
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
