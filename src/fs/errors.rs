use std::io::Error as IOError;

#[derive(Debug)]
pub enum DeserializationError {
    WrongFileFormat(String),
}

#[derive(Debug)]
pub enum PrimesReadError {
    IO(IOError),
    Deserialization(DeserializationError),
}

impl From<DeserializationError> for PrimesReadError {
    fn from(de: DeserializationError) -> Self {
        PrimesReadError::Deserialization(de)
    }
}

impl From<IOError> for PrimesReadError {
    fn from(de: IOError) -> Self {
        PrimesReadError::IO(de)
    }
}
