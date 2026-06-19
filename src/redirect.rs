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
