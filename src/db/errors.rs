use std::{error::Error, fmt::Display};
use sled::transaction::{ConflictableTransactionError, TransactionError};

#[derive(Debug)]
pub enum TransientError {
    IncretmentError,
    ParsingToByteError,
    ParsingToUTF8Error,
    SledError {error: sled::Error},
    SledTransactionError,
}

impl Display for TransientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransientError::IncretmentError => writeln!(f, "Incretment has failed"),
            TransientError::ParsingToByteError => writeln!(f, "Parsing to byte failed"),
            TransientError::ParsingToUTF8Error => writeln!(f, "Parsing to utf8 failed"),
            TransientError::SledError { error } => writeln!(f, "Sled failed {}", error),
            TransientError::SledTransactionError => writeln!(f, "Sled Transaction failed")
        }
    }
}


impl Error for TransientError {}
