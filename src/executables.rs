use std::{env, io, path::{Path, PathBuf}};

use is_executable::is_executable;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Executables {
    pub candidates: Vec<PathBuf>
}

impl Executables {
    // we iterate over the entries of the path like in the executable commando and we save the
    // executables in an String Vec, we save the PathBuf of the entry
    pub fn new() -> Result<Self, io::Error> {
        let path_str = env::var("PATH").expect("PATH variable not found in env");
        let mut executables = Vec::new();

        env::split_paths(&path_str).for_each(|path| {
            if Path::exists(&path) {
                WalkDir::new(&path).into_iter().filter_map(|e| e.ok()).for_each(|entry| {
                    if !is_executable(entry.path()) {
                        return;
                    }

                    executables.push(entry.into_path());
                });
            }
        });

        Ok(Executables {
            candidates: executables
        })
    }
}
