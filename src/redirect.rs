#[derive(Debug)]
pub struct Redirect {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Redirect {
    pub fn new() -> Self {
        Redirect {
            stdout: None,
            stderr: None,
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Append,
    Redirect,
}

#[derive(Debug)]
pub struct OutputTarget {
    pub operator: String,
    pub action: Action,
    pub args: Vec<String>,
}

impl OutputTarget {
    pub fn new() -> Self {
        OutputTarget {
            operator: String::new(),
            action: Action::Redirect,
            args: vec![],
        }
    }
}
