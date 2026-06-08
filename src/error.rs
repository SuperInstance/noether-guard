use std::fmt;

/// Errors produced by noether-guard.
#[derive(Debug, Clone)]
pub enum Error {
    /// No conservation laws registered.
    NoLaws,
    /// Value count mismatch on tick.
    ValueCountMismatch { expected: usize, got: usize },
    /// Invalid tolerance (must be >= 0).
    InvalidTolerance(f64),
    /// Invalid time step (must be >= 0).
    InvalidTime(f64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoLaws => write!(f, "no conservation laws registered"),
            Self::ValueCountMismatch { expected, got } => {
                write!(f, "expected {expected} values, got {got}")
            }
            Self::InvalidTolerance(v) => write!(f, "invalid tolerance: {v}"),
            Self::InvalidTime(v) => write!(f, "invalid time: {v}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
