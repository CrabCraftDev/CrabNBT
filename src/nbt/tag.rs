use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_nbt::error::Error;
use crab_nbt::nbt::compound::NbtCompound;
use crab_nbt::nbt::utils::*;
use derive_more::From;
use std::fmt::{Display, Formatter};
use std::io::Cursor;

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
    ByteArray(Bytes) = BYTE_ARRAY_ID,
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

    pub fn serialize(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(self.get_type_id());
        bytes.put(self.serialize_data());
        bytes.freeze()
    }

    pub fn serialize_data(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        match self {
            NbtTag::End => {}
            NbtTag::Byte(byte) => bytes.put_i8(*byte),
            NbtTag::Short(short) => bytes.put_i16(*short),
            NbtTag::Int(int) => bytes.put_i32(*int),
            NbtTag::Long(long) => bytes.put_i64(*long),
            NbtTag::Float(float) => bytes.put_f32(*float),
            NbtTag::Double(double) => bytes.put_f64(*double),
            NbtTag::ByteArray(byte_array) => {
                bytes.put_i32(byte_array.len() as i32);
                bytes.put_slice(byte_array);
            }
            NbtTag::String(string) => {
                let java_string = simd_cesu8::encode(string);
                bytes.put_u16(java_string.len() as u16);
                bytes.put_slice(&java_string);
            }
            NbtTag::List(list) => {
                bytes.put_u8(list.first().unwrap_or(&NbtTag::End).get_type_id());
                bytes.put_i32(list.len() as i32);
                for nbt_tag in list {
                    bytes.put(nbt_tag.serialize_data())
                }
            }
            NbtTag::Compound(compound) => {
                bytes.put(compound.serialize_content());
            }
            NbtTag::IntArray(int_array) => {
                bytes.put_i32(int_array.len() as i32);
                for int in int_array {
                    bytes.put_i32(*int)
                }
            }
            NbtTag::LongArray(long_array) => {
                bytes.put_i32(long_array.len() as i32);
                for long in long_array {
                    bytes.put_i64(*long)
                }
            }
        }
        bytes.freeze()
    }

    pub fn deserialize(bytes: &mut impl Buf) -> Result<NbtTag, Error> {
        let tag_id = bytes.get_u8();
        Self::deserialize_data(bytes, tag_id)
    }

    pub fn deserialize_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<NbtTag, Error> {
        Self::deserialize(cursor)
    }

    pub fn deserialize_data(bytes: &mut impl Buf, tag_id: u8) -> Result<NbtTag, Error> {
        match tag_id {
            END_ID => Ok(NbtTag::End),
            BYTE_ID => {
                let byte = bytes.get_i8();
                Ok(NbtTag::Byte(byte))
            }
            SHORT_ID => {
                let short = bytes.get_i16();
                Ok(NbtTag::Short(short))
            }
            INT_ID => {
                let int = bytes.get_i32();
                Ok(NbtTag::Int(int))
            }
            LONG_ID => {
                let long = bytes.get_i64();
                Ok(NbtTag::Long(long))
            }
            FLOAT_ID => {
                let float = bytes.get_f32();
                Ok(NbtTag::Float(float))
            }
            DOUBLE_ID => {
                let double = bytes.get_f64();
                Ok(NbtTag::Double(double))
            }
            BYTE_ARRAY_ID => {
                let len = bytes.get_i32() as usize;
                let byte_array = bytes.copy_to_bytes(len);
                Ok(NbtTag::ByteArray(byte_array))
            }
            STRING_ID => Ok(NbtTag::String(get_nbt_string(bytes).unwrap())),
            LIST_ID => {
                let tag_type_id = bytes.get_u8();
                let len = bytes.get_i32();
                let mut list = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    let tag = NbtTag::deserialize_data(bytes, tag_type_id)?;
                    assert_eq!(tag.get_type_id(), tag_type_id);
                    list.push(tag);
                }
                Ok(NbtTag::List(list))
            }
            COMPOUND_ID => Ok(NbtTag::Compound(NbtCompound::deserialize_content(bytes)?)),
            INT_ARRAY_ID => {
                const BYTES: usize = size_of::<i32>();

                let len = bytes.get_i32() as usize;
                let numbers = read_array::<i32, BYTES, _>(bytes, len, i32::from_be_bytes);
                Ok(NbtTag::IntArray(numbers))
            }
            LONG_ARRAY_ID => {
                const BYTES: usize = size_of::<i64>();

                let len = bytes.get_i32() as usize;
                let numbers = read_array::<i64, BYTES, _>(bytes, len, i64::from_be_bytes);
                Ok(NbtTag::LongArray(numbers))
            }
            _ => Err(Error::UnknownTagId(tag_id)),
        }
    }

    pub fn deserialize_data_from_cursor(
        cursor: &mut Cursor<&[u8]>,
        tag_id: u8,
    ) -> Result<NbtTag, Error> {
        Self::deserialize_data(cursor, tag_id)
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

    pub fn extract_byte_array(&self) -> Option<Bytes> {
        match self {
            // Note: Bytes are free to clone, so we can hand out an owned type
            NbtTag::ByteArray(byte_array) => Some(byte_array.clone()),
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
        NbtTag::ByteArray(Bytes::copy_from_slice(value))
    }
}

impl From<bool> for NbtTag {
    fn from(value: bool) -> Self {
        NbtTag::Byte(value as i8)
    }
}

impl Display for NbtTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::End => Ok(()),
            Self::Byte(x) => write!(f, "{x}b"),
            Self::Short(x) => write!(f, "{x}s"),
            Self::Int(x) => write!(f, "{x}"),
            Self::Long(x) => write!(f, "{x}L"),
            // using debug here matches Minecraft on whole numbers (3.0 instead of 3)
            Self::Float(x) => write!(f, "{x:?}f"),
            Self::Double(x) => write!(f, "{x:?}d"),
            Self::ByteArray(arr) => write_listlike(
                f, "B; ", "B",
                arr.iter().map(|b| *b as i8)
            ),
            Self::String(s) => write!(f, "{}", escape_string_value(s)),
            Self::List(list) => write_listlike(f, "", "", list),
            Self::Compound(compound) => write!(f, "{compound}"),
            Self::IntArray(arr) => write_listlike(f, "I; ", "", arr),
            Self::LongArray(arr) => write_listlike(f, "L; ", "L", arr)
        }
    }
}

fn write_listlike<T: Display, I: IntoIterator<Item = T>>
    (f: &mut Formatter<'_>, prefix: &'static str, affix: &'static str, arr: I)
    -> std::fmt::Result {
    write!(f, "[{prefix}")?;
    join_formatted(
        f, ", ",
        arr.into_iter().map(|x| move |f: &mut Formatter<'_>| write!(f, "{x}{affix}"))
    )?;
    write!(f, "]")
}
