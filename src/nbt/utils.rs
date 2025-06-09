use crate::{error::Error, slice_cursor::BinarySliceCursor};
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

pub fn get_nbt_string(bytes: &mut BinarySliceCursor) -> Result<String, Error> {
    let len = bytes.read_u16_be()? as usize;
    let string_bytes = bytes.read(len)?;
    from_java_cesu8(string_bytes)
        .as_deref()
        .map(ToOwned::to_owned)
        .map_err(|_| Error::InvalidJavaString)
}

// pub trait CursorSliceGetExt<T = Self> {
//     fn get_slice(&self, len: usize) -> Result<&[T], Error>;
// }

// impl<T> CursorSliceGetExt<T> for Cursor<&[T]> {
//     fn get_slice(&self, len: usize) -> Result<&[T], Error> {
//         let pos = self.position() as usize;
//         let end_bound = pos + len;
//         if end_bound > self.get_ref().len() - 1 {
//             Err(Error::NotEnoughBytes {
//                 requested: len,
//                 available: self.get_ref().len() - pos,
//             })
//         } else {
//             Ok(&self.get_ref()[pos..end_bound])
//         }
//     }
// }
