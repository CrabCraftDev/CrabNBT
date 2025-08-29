use crab_nbt::error::Error;
use crab_nbt::nbt::compound::NbtCompound;
use crab_nbt::nbt::utils::*;
use derive_more::From;
use std::io::Cursor;

use crate::nbt::slice_cursor::BinarySliceCursor;

/// Enum representing the different types of NBT tags.
/// Each variant corresponds to a different type of data that can be stored in an NBT tag.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, PartialOrd, From)]
pub enum NbtTag {
    End = END_ID,
    Byte(i8) = BYTE_ID,
    Short(i16) = SHORT_ID,
    Int(i32) = INT_ID,
    Long(i64) = LONG_ID,
    Float(f32) = FLOAT_ID,
    Double(f64) = DOUBLE_ID,
    ByteArray(Vec<u8>) = BYTE_ARRAY_ID,
    String(String) = STRING_ID,
    List(Vec<NbtTag>) = LIST_ID,
    Compound(NbtCompound) = COMPOUND_ID,
    IntArray(Vec<i32>) = INT_ARRAY_ID,
    LongArray(Vec<i64>) = LONG_ARRAY_ID,
}

impl NbtTag {
    /// Returns the numeric id associated with the data type.
    pub const fn get_type_id(&self) -> u8 {
        // See https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
        unsafe { *(self as *const Self as *const u8) }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.get_type_id());
        bytes.extend(self.serialize_data());
        bytes
    }

    pub fn serialize_data(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            NbtTag::End => {}
            NbtTag::Byte(byte) => bytes.extend(byte.to_be_bytes()),
            NbtTag::Short(short) => bytes.extend(short.to_be_bytes()),
            NbtTag::Int(int) => bytes.extend(int.to_be_bytes()),
            NbtTag::Long(long) => bytes.extend(long.to_be_bytes()),
            NbtTag::Float(float) => bytes.extend(float.to_be_bytes()),
            NbtTag::Double(double) => bytes.extend(double.to_be_bytes()),
            NbtTag::ByteArray(byte_array) => {
                bytes.extend((byte_array.len() as i32).to_be_bytes());
                bytes.extend(byte_array);
            }
            NbtTag::String(string) => {
                let java_string = simd_cesu8::encode(string);
                bytes.extend((java_string.len() as u16).to_be_bytes());
                bytes.extend_from_slice(&java_string[..]);
            }
            NbtTag::List(list) => {
                bytes.extend((list.first().unwrap_or(&NbtTag::End).get_type_id()).to_be_bytes());
                bytes.extend((list.len() as i32).to_be_bytes());
                for nbt_tag in list {
                    bytes.extend(nbt_tag.serialize_data())
                }
            }
            NbtTag::Compound(compound) => {
                bytes.extend(compound.serialize_content());
            }
            NbtTag::IntArray(int_array) => {
                bytes.extend((int_array.len() as i32).to_be_bytes());
                for int in int_array {
                    bytes.extend(int.to_be_bytes())
                }
            }
            NbtTag::LongArray(long_array) => {
                bytes.extend((long_array.len() as i32).to_be_bytes());
                for long in long_array {
                    bytes.extend(long.to_be_bytes())
                }
            }
        }
        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Result<NbtTag, Error> {
        Self::deserialize_internal(&mut BinarySliceCursor::new(bytes))
    }

    pub fn deserialize_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<NbtTag, Error> {
        BinarySliceCursor::wrap_io_cursor(cursor, Self::deserialize_internal).flatten()
    }

    pub(crate) fn deserialize_internal(bytes: &mut BinarySliceCursor) -> Result<NbtTag, Error> {
        let tag_id = bytes.read_u8()?;
        Self::deserialize_data_internal(bytes, tag_id)
    }

    pub fn deserialize_data(bytes: &[u8], tag_id: u8) -> Result<NbtTag, Error> {
        Self::deserialize_data_internal(&mut BinarySliceCursor::new(bytes), tag_id)
    }

    pub fn deserialize_data_from_cursor(
        bytes: &mut Cursor<&[u8]>,
        tag_id: u8,
    ) -> Result<NbtTag, Error> {
        BinarySliceCursor::wrap_io_cursor(bytes, |cursor| {
            Self::deserialize_data_internal(cursor, tag_id)
        })
        .flatten()
    }

    pub(crate) fn deserialize_data_internal(
        bytes: &mut BinarySliceCursor,
        tag_id: u8,
    ) -> Result<NbtTag, Error> {
        match tag_id {
            END_ID => Ok(NbtTag::End),
            BYTE_ID => {
                let byte = bytes.read_i8()?;
                Ok(NbtTag::Byte(byte))
            }
            SHORT_ID => {
                let short = bytes.read_i16_be()?;
                Ok(NbtTag::Short(short))
            }
            INT_ID => {
                let int = bytes.read_i32_be()?;
                Ok(NbtTag::Int(int))
            }
            LONG_ID => {
                let long = bytes.read_i64_be()?;
                Ok(NbtTag::Long(long))
            }
            FLOAT_ID => {
                let float = bytes.read_f32_be()?;
                Ok(NbtTag::Float(float))
            }
            DOUBLE_ID => {
                let double = bytes.read_f64_be()?;
                Ok(NbtTag::Double(double))
            }
            BYTE_ARRAY_ID => {
                let len = bytes.read_i32_be()? as usize;
                let byte_array = bytes.read(len)?.to_vec();
                Ok(NbtTag::ByteArray(byte_array))
            }
            STRING_ID => Ok(NbtTag::String(get_nbt_string(bytes).unwrap().to_string())),
            LIST_ID => {
                let tag_type_id = bytes.read_u8()?;
                let len = bytes.read_i32_be()?;
                let mut list = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    let tag = NbtTag::deserialize_data_internal(bytes, tag_type_id)?;
                    assert_eq!(tag.get_type_id(), tag_type_id);
                    list.push(tag);
                }
                Ok(NbtTag::List(list))
            }
            COMPOUND_ID => Ok(NbtTag::Compound(NbtCompound::deserialize_content_internal(
                bytes,
            )?)),
            INT_ARRAY_ID => {
                const BYTES: usize = size_of::<i32>();

                let len = bytes.read_i32_be()? as usize;
                let numbers = read_array::<i32, BYTES, _>(bytes, len, i32::from_be_bytes)?;
                Ok(NbtTag::IntArray(numbers))
            }
            LONG_ARRAY_ID => {
                const BYTES: usize = size_of::<i64>();

                let len = bytes.read_i32_be()? as usize;
                let numbers = read_array::<i64, BYTES, _>(bytes, len, i64::from_be_bytes)?;
                Ok(NbtTag::LongArray(numbers))
            }
            _ => Err(Error::UnknownTagId(tag_id)),
        }
    }

    pub fn extract_byte(&self) -> Option<i8> {
        match self {
            NbtTag::Byte(byte) => Some(*byte),
            _ => None,
        }
    }

    pub fn extract_short(&self) -> Option<i16> {
        match self {
            NbtTag::Short(short) => Some(*short),
            _ => None,
        }
    }

    pub fn extract_int(&self) -> Option<i32> {
        match self {
            NbtTag::Int(int) => Some(*int),
            _ => None,
        }
    }

    pub fn extract_long(&self) -> Option<i64> {
        match self {
            NbtTag::Long(long) => Some(*long),
            _ => None,
        }
    }

    pub fn extract_float(&self) -> Option<f32> {
        match self {
            NbtTag::Float(float) => Some(*float),
            _ => None,
        }
    }

    pub fn extract_double(&self) -> Option<f64> {
        match self {
            NbtTag::Double(double) => Some(*double),
            _ => None,
        }
    }

    pub fn extract_bool(&self) -> Option<bool> {
        match self {
            NbtTag::Byte(byte) => Some(*byte != 0),
            _ => None,
        }
    }

    pub fn extract_byte_array(&self) -> Option<&Vec<u8>> {
        match self {
            // Note: Bytes are free to clone, so we can hand out an owned type
            NbtTag::ByteArray(byte_array) => Some(byte_array),
            _ => None,
        }
    }

    pub fn extract_string(&self) -> Option<&String> {
        match self {
            NbtTag::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn extract_list(&self) -> Option<&Vec<NbtTag>> {
        match self {
            NbtTag::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn extract_compound(&self) -> Option<&NbtCompound> {
        match self {
            NbtTag::Compound(compound) => Some(compound),
            _ => None,
        }
    }

    pub fn extract_int_array(&self) -> Option<&Vec<i32>> {
        match self {
            NbtTag::IntArray(int_array) => Some(int_array),
            _ => None,
        }
    }

    pub fn extract_long_array(&self) -> Option<&Vec<i64>> {
        match self {
            NbtTag::LongArray(long_array) => Some(long_array),
            _ => None,
        }
    }
}

impl From<&str> for NbtTag {
    fn from(value: &str) -> Self {
        NbtTag::String(value.to_string())
    }
}

impl From<&[u8]> for NbtTag {
    fn from(value: &[u8]) -> Self {
        NbtTag::ByteArray(value.to_vec())
    }
}

impl From<bool> for NbtTag {
    fn from(value: bool) -> Self {
        NbtTag::Byte(value as i8)
    }
}
