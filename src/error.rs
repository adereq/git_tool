use std::io;
use std::fmt;

pub enum GitError {
    IoError(io::Error),
    NoDirectory,
    InvalidCommit,
    InvalidIndex
}

//to display error messages
impl fmt::Display for GitError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            GitError::IoError(ref e) => e.fmt(formatter),
            GitError::NoDirectory => formatter.write_str("No directory found"),
            GitError::InvalidCommit => formatter.write_str("Invalid command"),
            GitError::InvalidIndex => formatter.write_str("Invalid index"),
        }
    }
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> GitError {
        GitError::IoError(err)
    }
}