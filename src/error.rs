use std::fmt;
use std::io;

pub type PygoResult<T> = Result<T, PygoError>;

#[derive(Debug, Clone)]
pub struct PygoError {
    pub kind: ErrorKind,
    pub message: String,
}

impl From<io::Error> for PygoError {
    fn from(error: io::Error) -> Self {
        Self {
            kind: ErrorKind::Io,
            message: error.to_string(),
        }
    }
}

impl fmt::Display for PygoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PygoError from `{}` \"{}\"", self.kind, self.message)
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Io,
    NoArgument,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io => write!(f, "IO"),
            Self::NoArgument => write!(f, "No Argument's"),
        }
    }
}
