use rustyline::{
    Helper, completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator,
};

pub struct ShellHelper;

impl ShellHelper {
    pub fn default() -> Self {
        ShellHelper
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if line == "ech" && pos == line.len() {
            Ok((0, vec!["echo ".to_string()]))
        } else if line == "exi" && pos == line.len() {
            Ok((0, vec!["exit ".to_string()]))
        } else {
            Ok((0, vec![]))
        }
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {}

impl Helper for ShellHelper {}
