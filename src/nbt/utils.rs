use crate::error::Error;
use bytes::{Buf, Bytes};
use cesu8::from_java_cesu8;

pub const END_ID: u8 = 0;
pub const BYTE_ID: u8 = 1;
pub const SHORT_ID: u8 = 2;
pub const INT_ID: u8 = 3;
pub const LONG_ID: u8 = 4;
pub const FLOAT_ID: u8 = 5;
pub const DOUBLE_ID: u8 = 6;
pub const BYTE_ARRAY_ID: u8 = 7;
pub const STRING_ID: u8 = 8;
pub const LIST_ID: u8 = 9;
pub const COMPOUND_ID: u8 = 10;
pub const INT_ARRAY_ID: u8 = 11;
pub const LONG_ARRAY_ID: u8 = 12;

pub fn get_nbt_string(bytes: &mut Bytes) -> Result<String, Error> {
    let len = bytes.get_u16() as usize;
    let string_bytes = bytes.copy_to_bytes(len);
    let string = from_java_cesu8(&string_bytes).map_err(|_| Error::InvalidJavaString)?;
    Ok(string.to_string())
}
