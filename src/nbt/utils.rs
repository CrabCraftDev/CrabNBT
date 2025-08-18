use crate::{error::Error, slice_cursor::BinarySliceCursor};
use simd_cesu8::decode;

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
    decode(string_bytes)
        .as_deref()
        .map(ToOwned::to_owned)
        .map_err(|_| Error::InvalidJavaString)
}

// This can be improved once rust-lang/rust#132980 is resolved:
// Instead of passing `BYTES` manually, we could use const generics, e.g. `size_of::<T>()`.
pub(crate) fn read_array<T, const N: usize, F>(
    bytes: &mut BinarySliceCursor,
    len: usize,
    from_be: F,
) -> Result<Vec<T>, Error>
where
    F: Fn([u8; N]) -> T,
{
    Ok(bytes
        .read(len * N)?
        .chunks_exact(N)
        .map(|chunk| {
            let arr: [u8; N] = chunk.try_into().expect("chunk size mismatch");
            from_be(arr)
        })
        .collect())
}
