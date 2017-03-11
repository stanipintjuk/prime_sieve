use std::fmt::{Debug, Formatter, Error as FmtError};
use std::time::SystemTimeError;
use std::result::Result;
use std::sync::mpsc::RecvError;

pub enum MathError {
    Limit(String),
}

pub enum ThreadError {
    Recv(RecvError),
}
pub enum SieveError {
    Thread(ThreadError),
    Time(SystemTimeError),
}

impl From<ThreadError> for SieveError {
    fn from(err: ThreadError) -> Self {
        SieveError::Thread(err)
    }
}

impl From<RecvError> for SieveError {
    fn from(err: RecvError) -> Self {
        SieveError::Thread(ThreadError::Recv(err))
    }
}

impl From<SystemTimeError> for SieveError {
    fn from(err: SystemTimeError) -> Self {
        SieveError::Time(err)
    }
}

impl Debug for MathError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &MathError::Limit(ref msg) => write!(f, "MathError::Limit({})", msg),
        }
    }
}
