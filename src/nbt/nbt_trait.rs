use std::fmt::{Display, Formatter, Result as FmtResult};

use as_any::AsAny;
use bytes::{Buf, BufMut, Bytes};

use crate::{NbtCompound, NbtTag, TryAsMut, TryAsRef, nbt::{list::NbtList, utils::{escape_string_value, ids::*, write_listlike}}};

/// Implements behaviour for nbt-datatypes that should not be exposed outside the library.
/// Usually this is serialisation behaviour and dynamic dispatches used throughout the library.
pub(crate) trait PrivateNbtCompatible: AsAny {
    // fn deserialize(bytes: &mut impl Buf) where Self: Sized;
    // // TODO: Fix dyn-incompatibility caused by this generic method.
    // fn serialize(&self, bytes: &mut impl BufMut) where Self: Sized;

    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult;
}

pub trait NbtCompatible: PrivateNbtCompatible {
    fn get_type_id(&self) -> u8;
    fn snbt(&self) -> SnbtDisplay<Self> where Self: Sized {
        SnbtDisplay(self)
    }

    fn as_tag(self) -> NbtTag;
}
impl dyn NbtCompatible {
    // This method cannot be named "snbt" due to rustc falsely claiming
    // method ambiguity with NbtCompatible::snbt, which requires Self: Sized
    pub fn snbt_dyn(&self) -> SnbtDisplay<dyn NbtCompatible> {
        SnbtDisplay(self)
    }
}
impl<T: NbtCompatible> TryAsRef<T> for dyn NbtCompatible {
    fn try_as_ref(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}
impl<T: NbtCompatible> TryAsMut<T> for dyn NbtCompatible {
    fn try_as_mut(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

macro_rules! impl_NbtCompatible {
    ($($type:ty => $type_id:expr, $wrapper:expr),+) => {
        $(
            impl NbtCompatible for $type {
                fn get_type_id(&self) -> u8 {
                    $type_id
                }

                fn as_tag(self) -> NbtTag {
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
}
impl PrivateNbtCompatible for i16 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}s")
    }
}
impl PrivateNbtCompatible for i32 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}
impl PrivateNbtCompatible for i64 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}L")
    }
}
impl PrivateNbtCompatible for f32 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // using debug here matches Minecraft on whole numbers (3.0 instead of 3)
        write!(f, "{self:?}f")
    }
}
impl PrivateNbtCompatible for f64 {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // using debug here matches Minecraft on whole numbers (3.0 instead of 3)
        write!(f, "{self:?}d")
    }
}
impl PrivateNbtCompatible for Bytes {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write_listlike(f, "B; ", "B", self.iter().map(|b| *b as i8))
    }
}
impl PrivateNbtCompatible for String {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", escape_string_value(self))
    }
}
impl PrivateNbtCompatible for NbtList {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}
impl PrivateNbtCompatible for NbtCompound {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}
impl PrivateNbtCompatible for Vec<i32> {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write_listlike(f, "I; ", "", self)
    }
}
impl PrivateNbtCompatible for Vec<i64> {
    fn write_snbt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write_listlike(f, "L; ", "L", self)
    }
}

impl_NbtCompatible! {
    i8 => BYTE_ID, NbtTag::Byte,
    i16 => SHORT_ID, NbtTag::Short,
    i32 => INT_ID, NbtTag::Int,
    i64 => LONG_ID, NbtTag::Long,
    f32 => FLOAT_ID, NbtTag::Float,
    f64 => DOUBLE_ID, NbtTag::Double,
    Bytes => BYTE_ARRAY_ID, NbtTag::ByteArray,
    String => STRING_ID, NbtTag::String,
    NbtList => LIST_ID, NbtTag::List,
    NbtCompound => COMPOUND_ID, NbtTag::Compound,
    Vec<i32> => INT_ARRAY_ID, NbtTag::IntArray,
    Vec<i64> => LONG_ARRAY_ID, NbtTag::LongArray
}

#[derive(Debug, Clone, Copy)]
pub struct SnbtDisplay<'a, T: NbtCompatible + PrivateNbtCompatible + ?Sized>(pub &'a T);
impl<'a, T: NbtCompatible + ?Sized + PrivateNbtCompatible> Display for SnbtDisplay<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.write_snbt(f)
    }
}