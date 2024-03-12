use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The root tag of the NBT file is not a compound tag. Received tag id: {0}")]
    NoRootCompound(u8),
    #[error("The provided string is not a valid Java string.")]
    InvalidJavaString,
    #[error("Encountered an unknown NBT tag id {0}.")]
    UnknownTagId(u8),
}
