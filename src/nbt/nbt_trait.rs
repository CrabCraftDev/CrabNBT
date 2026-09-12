use std::fmt::{Display, Formatter, Result as FmtResult};

use as_any::AsAny;
use bytes::{Buf, BufMut, Bytes};

use crate::{
    error::Error,
    nbt::{
        list::NbtList,
        utils::{escape_string_value, get_nbt_string, ids::*, read_array, write_listlike},
    },
    NbtCompound, NbtTag,
};

/// Implements behaviour for nbt-datatypes that should not be exposed outside the library.
/// Usually this is serialisation behaviour and dynamic dispatches used throughout the library.
pub(crate) trait PrivateNbtCompatible: AsAny {
    fn get_id() -> u8
    where
        Self: Sized;

    /// Read the contents of the tag out of the buffer and constructs a new instance of self.
    /// The tag id has already been read at this point
    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized;
    /// Serialize self into bytes, excluding the tag id
    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized;

    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult;
}

#[allow(private_bounds)]
pub trait NbtCompatible: PrivateNbtCompatible {
    fn get_type_id(&self) -> u8;
    fn snbt(&self) -> SnbtDisplay<'_, Self>
    where
        Self: Sized,
    {
        SnbtDisplay(self)
    }

    fn into_tag(self) -> NbtTag;
}
impl dyn NbtCompatible {
    // This method cannot be named "snbt" due to rustc falsely claiming
    // method ambiguity with NbtCompatible::snbt, which requires Self: Sized
    // (we also can't remove that bound because the return value uses Self)
    pub fn snbt_dyn(&self) -> SnbtDisplay<'_, dyn NbtCompatible> {
        SnbtDisplay(self)
    }

    pub fn as_concrete<T: NbtCompatible>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    pub fn as_concrete_mut<T: NbtCompatible>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

macro_rules! impl_NbtCompatible {
    ($($type:ty => $wrapper:expr),+) => {
        $(
            impl NbtCompatible for $type {
                fn get_type_id(&self) -> u8 {
                    <$type>::get_id()
                }

                fn into_tag(self) -> NbtTag {
                    $wrapper(self)
                }
            }
        )+
    };
}

impl PrivateNbtCompatible for i8 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}b")
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(bytes.try_get_i8()?)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_i8(*self)
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        BYTE_ID
    }
}
impl PrivateNbtCompatible for i16 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}s")
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(bytes.try_get_i16()?)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_i16(*self)
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        SHORT_ID
    }
}
impl PrivateNbtCompatible for i32 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(bytes.try_get_i32()?)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_i32(*self)
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        INT_ID
    }
}
impl PrivateNbtCompatible for i64 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}L")
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(bytes.try_get_i64()?)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_i64(*self)
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        LONG_ID
    }
}
impl PrivateNbtCompatible for f32 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // using debug here matches Minecraft on whole numbers (3.0 instead of 3)
        write!(f, "{self:?}f")
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(bytes.try_get_f32()?)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_f32(*self)
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        FLOAT_ID
    }
}
impl PrivateNbtCompatible for f64 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // using debug here matches Minecraft on whole numbers (3.0 instead of 3)
        write!(f, "{self:?}d")
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(bytes.try_get_f64()?)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_f64(*self)
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        DOUBLE_ID
    }
}
impl PrivateNbtCompatible for Bytes {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write_listlike(f, "B; ", "B", self.iter().map(|b| *b as i8))
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = bytes.try_get_i32()? as usize;
        let byte_array = bytes.copy_to_bytes(len);
        Ok(byte_array)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_i32(self.len() as i32);
        bytes.put_slice(self);
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        BYTE_ARRAY_ID
    }
}
impl PrivateNbtCompatible for String {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", escape_string_value(self))
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(get_nbt_string(bytes).unwrap())
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        let java_string = simd_cesu8::encode(self);
        bytes.put_u16(java_string.len() as u16);
        bytes.put_slice(&java_string);
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        STRING_ID
    }
}
impl PrivateNbtCompatible for Vec<i32> {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write_listlike(f, "I; ", "", self)
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        const BYTES: usize = size_of::<i32>();

        let len = bytes.try_get_i32()? as usize;
        let numbers = read_array::<i32, BYTES, _>(bytes, len, i32::from_be_bytes);
        Ok(numbers)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_i32(self.len() as i32);
        for int in self {
            bytes.put_i32(*int)
        }
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        INT_ARRAY_ID
    }
}
impl PrivateNbtCompatible for Vec<i64> {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write_listlike(f, "L; ", "L", self)
    }

    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        const BYTES: usize = size_of::<i64>();

        let len = bytes.try_get_i32()? as usize;
        let numbers = read_array::<i64, BYTES, _>(bytes, len, i64::from_be_bytes);
        Ok(numbers)
    }

    fn serialize_content_into(&self, bytes: &mut impl BufMut)
    where
        Self: Sized,
    {
        bytes.put_i32(self.len() as i32);
        for long in self {
            bytes.put_i64(*long)
        }
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        LONG_ARRAY_ID
    }
}

impl_NbtCompatible! {
    i8 => NbtTag::Byte,
    i16 => NbtTag::Short,
    i32 => NbtTag::Int,
    i64 => NbtTag::Long,
    f32 => NbtTag::Float,
    f64 => NbtTag::Double,
    Bytes => NbtTag::ByteArray,
    String => NbtTag::String,
    NbtList => NbtTag::List,
    NbtCompound => NbtTag::Compound,
    Vec<i32> => NbtTag::IntArray,
    Vec<i64> => NbtTag::LongArray
}

#[derive(Debug, Clone, Copy)]
#[allow(private_bounds)]
pub struct SnbtDisplay<'a, T: NbtCompatible + PrivateNbtCompatible + ?Sized>(pub &'a T);
impl<'a, T: NbtCompatible + ?Sized + PrivateNbtCompatible> Display for SnbtDisplay<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.write_snbt(f)
    }
}
