#[derive(Debug)]
pub struct Input<'a> {
    pub cmd: &'a str,
    pub args: Vec<String>,
}

impl<'a> Input<'a> {
    fn new(cmd: &'a str, args: Vec<String>) -> Self {
        Input { cmd, args }
    }
}

pub fn parse_input(input: &str) -> Option<Input<'_>> {
    let input: Vec<&str> = input.splitn(2, " ").collect();
    if input.len() == 0 {
        return None;
    }

    if input.len() == 1 {
        return Some(Input::new(input[0], Vec::new()));
    }

    let args = tokenize_args(input[1]);

    Some(Input::new(input[0], args))
}

fn tokenize_args(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut arg = String::new();
    let mut single_quote_counter = 0;
    let mut double_quote_counter = 0;

    for c in input.chars() {

        if c == '"' && double_quote_counter == 0 {
            double_quote_counter += 1;
            continue;
        }

        if c == '"' && double_quote_counter != 0 {
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
