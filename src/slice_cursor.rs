use std::{
    fmt::Debug,
    io::{Cursor, Seek as _},
};

use crate::error::Error;

#[derive(Default, Eq, PartialEq)]
pub struct BinarySliceCursor<'a> {
    inner: &'a [u8],
    pos: usize,
}

impl<'a> BinarySliceCursor<'a> {
    /// Creates a new cursor from a binary slice
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            inner: slice,
            pos: 0,
        }
    }

    /// Gets current cursor position
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets cursor position
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Skips n bytes
    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn has_remaining(&self) -> bool {
        self.pos < self.inner.len()
    }

    /// Reads count bytes as a slice
    pub fn read(&mut self, count: usize) -> Result<&[u8], Error> {
        let len = self.inner.len();
        let end_index = self.pos + count;
        // end_index is not inclusive
        if end_index <= len {
            let slice = &self.inner[self.pos..end_index];
            self.pos += count;
            Ok(slice)
        } else {
            Err(Error::NotEnoughBytes {
                requested: count,
                available: self.inner.len() - self.pos,
            })
        }
    }

    /// Reads N bytes as an array
    pub fn read_array<const N: usize>(&mut self) -> Result<&[u8; N], Error> {
        let slice = self.read(N)?;
        debug_assert_eq!(slice.len(), N);
        Ok(slice
            .try_into()
            .expect("slice should be of the requested length"))
    }

    /// Reads one byte
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        Ok(u8::from_be_bytes(*self.read_array::<1>()?))
    }

    /// Reads big endian u16
    pub fn read_u16_be(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes(*self.read_array::<2>()?))
    }

    /// Reads big endian u32
    pub fn read_u32_be(&mut self) -> Result<u32, Error> {
        Ok(u32::from_be_bytes(*self.read_array::<4>()?))
    }

    /// Reads big endian i8
    pub fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(i8::from_be_bytes(*self.read_array::<1>()?))
    }

    /// Reads big endian i16
    pub fn read_i16_be(&mut self) -> Result<i16, Error> {
        Ok(i16::from_be_bytes(*self.read_array::<2>()?))
    }

    /// Reads big endian i32
    pub fn read_i32_be(&mut self) -> Result<i32, Error> {
        Ok(i32::from_be_bytes(*self.read_array::<4>()?))
    }

    /// Reads big endian i64
    pub fn read_i64_be(&mut self) -> Result<i64, Error> {
        Ok(i64::from_be_bytes(*self.read_array::<8>()?))
    }

    /// Reads big endian f32
    pub fn read_f32_be(&mut self) -> Result<f32, Error> {
        Ok(f32::from_be_bytes(*self.read_array::<4>()?))
    }

    /// Reads big endian f64
    pub fn read_f64_be(&mut self) -> Result<f64, Error> {
        Ok(f64::from_be_bytes(*self.read_array::<8>()?))
    }

    /// Wraps [std::io::Cursor] allowing it to be used as a BinarySliceCursor in the provided callback
    pub fn wrap_io_cursor<'x, T, F>(cursor: &mut Cursor<&'x [u8]>, f: F) -> Result<T, Error>
    where
        F: for<'b> FnOnce(&'b mut BinarySliceCursor<'x>) -> T,
    {
        let mut slice_cursor: BinarySliceCursor<'_> =
            BinarySliceCursor::new(&cursor.get_ref()[cursor.position() as usize..]);
        let result = f(&mut slice_cursor);
        cursor.seek_relative(slice_cursor.pos() as i64)?;
        Ok(result)
    }
}

impl<'a> AsRef<BinarySliceCursor<'a>> for BinarySliceCursor<'a> {
    fn as_ref(&self) -> &BinarySliceCursor<'a> {
        self
    }
}

impl<'a> AsMut<BinarySliceCursor<'a>> for BinarySliceCursor<'a> {
    fn as_mut(&mut self) -> &mut BinarySliceCursor<'a> {
        self
    }
}

impl<'a> Debug for BinarySliceCursor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinarySliceCursor")
            .field("pos", &self.pos)
            .field(
                "inner",
                &format_args!("slice of length {}", self.inner.len()),
            )
            .finish()
    }
}

#[cfg(test)]
mod test {
    use crate::{error::Error, slice_cursor::BinarySliceCursor};

    #[test]
    fn test_get_slice() {
        let mut cursor = BinarySliceCursor::new(&[0, 1, 2, 3, 4, 5, 6][..]);
        cursor.set_pos(2);
        let slice = cursor.read(3).unwrap();
        assert_eq!(slice, &[2, 3, 4][..]);
        let slice2 = cursor.read(10).unwrap_err();
        assert!(matches!(
            slice2,
            Error::NotEnoughBytes {
                available: 2,
                requested: 10
            }
        ));
    }
}
