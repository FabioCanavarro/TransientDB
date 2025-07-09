use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TransientError {
    IncretmentFailure,
}

impl Display for TransientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransientError::IncretmentFailure => writeln!(f, "Incretment has failed"),
        }
    }
}

impl Error for TransientError {}
