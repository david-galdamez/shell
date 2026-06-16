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
    let input: Vec<&str> = input.splitn(1, " ").collect();
    if input.len() == 0 {
        return None;
    }

    println!("{input:?}");

    if input.len() == 1 {
        return Some(Input::new(input[0], Vec::new()));
    }

    let args = tokenize_args(input[2]);

    Some(Input::new(input[0], args))
}

fn tokenize_args(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut arg = String::new();
    let mut quote_counter = 0;

    for c in input.chars() {
        if c == '\'' {
            quote_counter += 1;
        }

        if c == '\'' && quote_counter != 0 {
            quote_counter -= 1;
        }

        if c == ' ' && quote_counter == 0 {
            args.push(arg.to_string());
            arg.clear();
        }

        arg.push(c);
    }

    args.push(arg);

    args
}
