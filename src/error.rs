#[cfg(feature = "serde")]
use serde::{de, ser};
#[cfg(feature = "serde")]
use std::fmt::Display;
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The root tag of the NBT file is not a compound tag. Received tag id: {0}")]
    NoRootCompound(u8),
    #[error("The provided string is not a valid Java string.")]
    InvalidJavaString,
    #[error("Encountered an unknown NBT tag id {0}.")]
    UnknownTagId(u8),
    #[error("Serde error: {0}")]
    SerdeError(String),
    #[error("NBT doesn't support this type {0}")]
    UnsupportedType(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Not enough bytes remaining in buffer to read value (requested {requested} but only {available} available)")]
    NotEnoughBytes { requested: usize, available: usize },
    #[error("Cannot skip {amount} bytes, only {available} bytes are remaining in the buffer")]
    InvalidSkip { amount: usize, available: usize },
}

#[cfg(feature = "serde")]
impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerdeError(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerdeError(msg.to_string())
    }
}
