use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_nbt::error::Error;
use crab_nbt::nbt::compound::NbtCompound;
use crab_nbt::nbt::utils::*;
use derive_more::From;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::io::Cursor;

use crate::nbt::list::NbtList;
use crate::nbt::nbt_trait::{NbtCompatible, PrivateNbtCompatible};

macro_rules! call_uniform {
    (($self:ident$(.$($expression:tt)+)?), $end_case:expr) => {
        {
            use self::NbtTag::*;
            match $self {
                End => $end_case,
                Byte(x) => x$(.$($expression)*)?,
                Short(x) => x$(.$($expression)*)?,
                Int(x) => x$(.$($expression)*)?,
                Long(x) => x$(.$($expression)*)?,
                Float(x) => x$(.$($expression)*)?,
                Double(x) => x$(.$($expression)*)?,
                ByteArray(x) => x$(.$($expression)*)?,
                String(x) => x$(.$($expression)*)?,
                List(x) => x$(.$($expression)*)?,
                Compound(x) => x$(.$($expression)*)?,
                IntArray(x) => x$(.$($expression)*)?,
                LongArray(x) => x$(.$($expression)*)?,
            }
        }
    };
}

/// Enum representing the different types of NBT tags.
/// Each variant corresponds to a different type of data that can be stored in an NBT tag.
#[repr(u8)]
#[derive(Clone, Debug, From)]
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
    List(NbtList) = LIST_ID,
    Compound(NbtCompound) = COMPOUND_ID,
    IntArray(Vec<i32>) = INT_ARRAY_ID,
    LongArray(Vec<i64>) = LONG_ARRAY_ID,
}

impl PartialEq for NbtTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Byte(a), Self::Byte(b)) => a == b,
            (Self::Short(a), Self::Short(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Long(a), Self::Long(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a.total_cmp(b) == Ordering::Equal,
            (Self::Double(a), Self::Double(b)) => a.total_cmp(b) == Ordering::Equal,
            (Self::ByteArray(a), Self::ByteArray(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            (Self::Compound(a), Self::Compound(b)) => a == b,
            (Self::IntArray(a), Self::IntArray(b)) => a == b,
            (Self::LongArray(a), Self::LongArray(b)) => a == b,
            _ => self.get_type_id() == other.get_type_id(),
        }
    }
}

impl Eq for NbtTag {}

impl PartialOrd for NbtTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NbtTag {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Byte(a), Self::Byte(b)) => a.cmp(b),
            (Self::Short(a), Self::Short(b)) => a.cmp(b),
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Long(a), Self::Long(b)) => a.cmp(b),
            (Self::Float(a), Self::Float(b)) => a.total_cmp(b),
            (Self::Double(a), Self::Double(b)) => a.total_cmp(b),
            (Self::ByteArray(a), Self::ByteArray(b)) => a.cmp(b),
            (Self::String(a), Self::String(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Compound(a), Self::Compound(b)) => a.cmp(b),
            (Self::IntArray(a), Self::IntArray(b)) => a.cmp(b),
            (Self::LongArray(a), Self::LongArray(b)) => a.cmp(b),
            _ => self.get_type_id().cmp(&other.get_type_id()),
        }
    }
}

impl NbtTag {
    /// Returns the numeric id associated with the data type.
    pub const fn get_type_id(&self) -> u8 {
        // See https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
        unsafe { *(self as *const Self as *const u8) }
    }

    pub fn serialize(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        self.serialize_into(&mut bytes);
        bytes.freeze()
    }

    pub fn serialize_into(&self, bytes: &mut BytesMut) {
        bytes.put_u8(self.get_type_id());
        self.serialize_data_into(bytes);
    }

    pub fn serialize_data(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        self.serialize_data_into(&mut bytes);
        bytes.freeze()
    }

    pub fn serialize_data_into(&self, bytes: &mut impl BufMut) {
        call_uniform!(
            (self.serialize_content_into(bytes)),
            () // End has no data, so serialization is noop
        );
    }

    pub fn deserialize(bytes: &mut impl Buf) -> Result<NbtTag, Error> {
        let tag_id = bytes.try_get_u8()?;
        Self::deserialize_data(bytes, tag_id)
    }

    pub fn deserialize_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<NbtTag, Error> {
        Self::deserialize(cursor)
    }

    pub fn deserialize_data(bytes: &mut impl Buf, tag_id: u8) -> Result<NbtTag, Error> {
        macro_rules! gen_match {
            (
                {
                    target = $target:expr,
                    bytes = $bytes:expr,
                    end = $end:expr,
                    else = $else:expr
                },
                $($tag_id:ident => $converter:expr),*
            ) => {
                {
                    fn deser_helper<T: PrivateNbtCompatible>(bytes: &mut impl Buf) -> Result<T, Error> {
                        T::deserialize_data(bytes)
                    }
                    match $target {
                        END_ID => $end,
                        $($tag_id => deser_helper($bytes).map($converter)),*,
                        _ => $else
                    }
                }
            };
        }
        gen_match!(
            {
                target = tag_id,
                bytes = bytes,
                end = Ok(NbtTag::End),
                else = Err(Error::UnknownTagId(tag_id))
            },
            BYTE_ID => NbtTag::Byte,
            SHORT_ID => NbtTag::Short,
            INT_ID => NbtTag::Int,
            LONG_ID => NbtTag::Long,
            FLOAT_ID => NbtTag::Float,
            DOUBLE_ID => NbtTag::Double,
            BYTE_ARRAY_ID => NbtTag::ByteArray,
            STRING_ID => NbtTag::String,
            LIST_ID => NbtTag::List,
            COMPOUND_ID => NbtTag::Compound,
            INT_ARRAY_ID => NbtTag::IntArray,
            LONG_ARRAY_ID => NbtTag::LongArray
        )
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

    pub fn extract_list(&self) -> Option<&NbtList> {
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

    /// Returns a [Some] with a reference to a dyn [NbtCompatible],
    /// if this tag is not [NbtTag::End], else [None].
    ///
    /// See also: [NbtTag::as_nbt_compatible_mut]
    pub fn as_nbt_compatible(&self) -> Option<&dyn NbtCompatible> {
        Some(call_uniform!((self), return None))
    }

    /// Returns a [Some] with a mutable reference to a dyn [NbtCompatible],
    /// if this tag is not [NbtTag::End], else [None].
    ///
    /// See also: [NbtTag::as_nbt_compatible]
    pub fn as_nbt_compatible_mut(&mut self) -> Option<&mut dyn NbtCompatible> {
        Some(call_uniform!((self), return None))
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        call_uniform!((self.write_snbt(f)), Ok(()))
    }
}
