use std::{
    fmt::Debug,
    io::{Cursor, Seek as _},
};

use crate::error::Error;

/// Our implementation of a cursor over a slice of bytes (`&[u8])
/// It has only what we need, and does not touch [std::io]
/// Thus it is simpler then a crate like `byteorder`
/// It also provides more human friendly errors in case something goes wrong
#[derive(Default, Eq, PartialEq)]
pub(crate) struct BinarySliceCursor<'a> {
    inner: &'a [u8],
    pos: usize,
}

impl<'a> BinarySliceCursor<'a> {
    pub(crate) fn new(slice: &'a [u8]) -> Self {
        Self {
            inner: slice,
            pos: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    // Currently used only by serde, so cfg avoids dead code warning
    // Remove cfg if you need this
    #[cfg(feature = "serde")]
    pub fn skip(&mut self, amount: usize) -> Result<(), Error> {
        let new_pos = self.pos + amount;
        if new_pos >= self.inner.len() {
            return Err(Error::InvalidSkip {
                amount,
                available: self.inner.len(),
            });
        }
        self.pos = new_pos;
        Ok(())
    }

    pub fn has_remaining(&self) -> bool {
        self.pos < self.inner.len()
    }

    /// Reads `count` bytes as a slice
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

    /// Reads `N` bytes as an array
    pub fn read_array<const N: usize>(&mut self) -> Result<&[u8; N], Error> {
        let slice = self.read(N)?;
        Ok(slice
            .try_into()
            .expect("slice should be of the requested length"))
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        Ok(u8::from_be_bytes(*self.read_array::<1>()?))
    }

    pub fn read_u16_be(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes(*self.read_array::<2>()?))
    }

    pub fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(i8::from_be_bytes(*self.read_array::<1>()?))
    }

    pub fn read_i16_be(&mut self) -> Result<i16, Error> {
        Ok(i16::from_be_bytes(*self.read_array::<2>()?))
    }

    pub fn read_i32_be(&mut self) -> Result<i32, Error> {
        Ok(i32::from_be_bytes(*self.read_array::<4>()?))
    }

    pub fn read_i64_be(&mut self) -> Result<i64, Error> {
        Ok(i64::from_be_bytes(*self.read_array::<8>()?))
    }

    pub fn read_f32_be(&mut self) -> Result<f32, Error> {
        Ok(f32::from_be_bytes(*self.read_array::<4>()?))
    }

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
        cursor.seek_relative(slice_cursor.position() as i64)?;
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
    use crate::{error::Error, nbt::slice_cursor::BinarySliceCursor};

    #[test]
    fn test_get_slice() {
        let mut cursor = BinarySliceCursor {
            inner: &[0, 1, 2, 3, 4, 5, 6][..],
            pos: 2,
        };
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
