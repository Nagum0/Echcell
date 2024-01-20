use std::fmt;
use std::error;

#[derive(Debug)]
pub enum CsvError {
    FileParseError,
    FileOutputError,
    ArgError,
    NullError,
    RangeError(String),
    CellPError(String),
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileParseError  => write!(f, "#[FILE ERROR] Could not parse csv file..."),
            Self::FileOutputError => write!(f, "#[FILE ERROR] Could not write to output file..."),
            Self::ArgError        => write!(f, "#[ARG ERROR] Incorrect argument amount..."),
            Self::NullError       => write!(f, "#[NULL]"),
            Self::RangeError(msg) => write!(f, "#[RANGE ERROR] {}", msg),
            Self::CellPError(msg) => write!(f, "#[CELL POINTER ERROR] {}", msg),
        }
    }
}

impl error::Error for CsvError {}