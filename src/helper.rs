use rustyline::{
    Helper,
    completion::{Completer, FilenameCompleter, extract_word},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};

use crate::executables::Executables;

pub struct ShellHelper {
    filenames: FilenameCompleter,
}

impl ShellHelper {
    pub fn default() -> Self {
        ShellHelper {
            filenames: FilenameCompleter::new(),
        }
    }

    // helper function, we get the executables of the path, we filter the executables by the ones
    // that start with the prefix we are typing and we return the filtered and sorted executables
    fn get_canditates(&self, prefix: &str) -> rustyline::Result<Vec<String>> {
        let mut candidates: Vec<String> = Executables::new()?
            .candidates
            .iter()
            .filter_map(|bin| {
                bin.file_name().and_then(|name| {
                    let name = name.to_string_lossy();
                    if name.starts_with(prefix) {
                        Some(name.to_string())
                    } else {
                        None
                    }
                })
            })
            .collect();

        candidates.sort();
        Ok(candidates)
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let (word_start, prefix) = extract_word(line, pos, None, |c: char| c == ' ');
        let is_first_word = line[..word_start].trim().is_empty();

        if is_first_word {
            if prefix == "ech" && pos == line.len() {
                return Ok((0, vec!["echo".to_string()]));
            } else if prefix == "exi" && pos == line.len() {
                return Ok((0, vec!["exit".to_string()]));
            }

            let candidates = self.get_canditates(prefix)?;
            Ok((0, candidates))
        } else {
            let (start, mut pairs) = self.filenames.complete(line, pos, ctx)?;
            let mut candidates = Vec::new();
            for pair in &mut pairs {
                if !pair.replacement.ends_with('/') && !pair.replacement.ends_with(' ') {
                    candidates.push(pair.replacement.clone());
                }
            }
            Ok((start, candidates))
        }
    }

    fn update(
        &self,
        line: &mut rustyline::line_buffer::LineBuffer,
        start: usize,
        elected: &str,
        cl: &mut rustyline::Changeset,
    ) {
        //check if there are other candidates
        let has_more_candidates = self
            .get_canditates(elected)
            .map(|candidate| candidate.iter().any(|c| c != elected))
            .unwrap_or(false);

        let replacement = if has_more_candidates {
            elected.to_string()
        } else {
            format!("{} ", elected)
        };

        let end = line.pos();
        line.replace(start..end, &replacement, cl);
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
