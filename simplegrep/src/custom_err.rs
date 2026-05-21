use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum GrepError {
    MissingSearchString,
    MissingFilePath,
}

impl fmt::Display for GrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrepError::MissingSearchString => write!(f, "Search string is missing from args"),
            GrepError::MissingFilePath => write!(f, "Filepath is missing from args"),
        }
    }
}

impl Error for GrepError {}
