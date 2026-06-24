use std::{env, path::Path};

use rustyline::{
    Helper,
    completion::{Completer, FilenameCompleter, extract_word},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};
use walkdir::WalkDir;

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

    fn get_dir_candidates(&self, prefix: &str) -> rustyline::Result<Vec<String>> {
        let path_str = env::var("PATH").expect("PATH variable not found in env");
        let mut directories = Vec::new();

        env::split_paths(&path_str).for_each(|path| {
            if Path::exists(&path) {
                WalkDir::new(&path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .for_each(|entry| {
                        if !Path::is_dir(entry.path()) {
                            return;
                        }

                        directories.push(entry.into_path());
                    });
            }
        });

        directories.sort();
        let candidates = directories.iter()
            .filter_map(|dir| {
                dir.file_name().and_then(|name| {
                    let name = name.to_string_lossy();
                    if name.starts_with(prefix) {
                        Some(name.to_string())
                    } else {
                        None
                    }
                })
            }).collect();
        Ok(candidates)
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
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
            let (start, mut pairs) = self.filenames.complete(line, pos, _ctx)?;
            let mut candidates = Vec::new();
            for pair in &mut pairs {
                if !pair.replacement.ends_with(' ') {
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
        let has_more_exec_candidates = self
            .get_canditates(elected)
            .map(|candidate| candidate.iter().any(|c| c != elected))
            .unwrap_or(false);

        let has_more_dir_candidates = self
            .get_dir_candidates(elected)
            .map(|candidate| candidate.iter().any(|c| c != elected))
            .unwrap_or(false);

        let matching_files: Vec<_> = WalkDir::new(".")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                .to_string_lossy()
                    .starts_with(elected.trim_start_matches("./"))
            }).collect();

        let has_more_file_candidates = matching_files.len() > 1;

        //now if we have more files candidates doesn't put the trealling space
        let replacement = if has_more_exec_candidates || has_more_dir_candidates || has_more_file_candidates {
            elected.to_string()
        } else {
            if !elected.ends_with('/') {
                format!("{} ", elected)
            } else {
                elected.to_string()
            }
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
