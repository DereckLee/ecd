use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum EcdError {
    NoInput,
    NotFound(PathBuf),
    Io(std::io::Error),
}

impl fmt::Display for EcdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInput => {
                write!(
                    f,
                    "no input: provide at least one --file (-f) or --directory (-d)"
                )
            }
            Self::NotFound(path) => write!(f, "path not found: {}", path.display()),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for EcdError {}

impl From<std::io::Error> for EcdError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
