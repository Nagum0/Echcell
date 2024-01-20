use std::fmt;
use std::error;

#[derive(Debug)]
pub enum CsvError {
    FileParseError,
    FileOutputError,
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileParseError  => write!(f, "#[FILE ERROR] Could not parse csv file..."),
            Self::FileOutputError => write!(f, "#[FILE ERROR] Could not write to output file..."),
        }
    }
}

impl error::Error for CsvError {}