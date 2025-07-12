use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TransientError {
    IncretmentFailure,
    ParsingToByteFailure,
    ParsingToUTF8Error
}

impl Display for TransientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransientError::IncretmentFailure => writeln!(f, "Incretment has failed"),
            TransientError::ParsingToByteFailure => writeln!(f, "Parsing to byte failed"),
            TransientError::ParsingToUTF8Error => writeln!(f, "Parsing to utf8 failed")
        }
    }
}

impl Error for TransientError {}
