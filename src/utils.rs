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
