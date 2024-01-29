use std::fmt;
use std::error;

#[derive(Debug)]
pub enum CsvError {
    FileError(String),
    ArgError,
    ExprError(String),
    RangeError(String),
    CellPError(String),
    RunError(String),
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileError(msg)  => write!(f, "#[FILE ERROR] {}", msg),
            Self::ArgError        => write!(f, "#[ARG ERROR] Incorrect argument amount..."),
            Self::ExprError(msg)  => write!(f, "#[EXPR ERROR] {}", msg),
            Self::RangeError(msg) => write!(f, "#[RANGE ERROR] {}", msg),
            Self::CellPError(msg) => write!(f, "#[CELL POINTER ERROR] {}", msg),
            Self::RunError(msg)   => write!(f, "Error while running application!\n{}", msg),
        }
    }
}

impl error::Error for CsvError {}
