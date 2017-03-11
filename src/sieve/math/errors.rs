use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result;

pub enum MathError {
    Limit(String),
}

impl Debug for MathError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &MathError::Limit(ref msg) => write!(f, "MathError::Limit({})", msg),
        }
    }
}
