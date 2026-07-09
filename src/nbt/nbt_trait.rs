use as_any::AsAny;
use bytes::{Buf, BufMut, Bytes};

use crate::{NbtCompound, NbtTag, TryAsMut, TryAsRef, nbt::{list::NbtList, utils::ids::*}};

/// Implements behaviour for nbt-datatypes that should not be exposed outside the library.
/// Usually this is serialisation behaviour and dynamic dispatches used throughout the library.
pub(crate) trait PrivateNbtCompatible: AsAny {
    // fn deserialize(bytes: &mut impl Buf) where Self: Sized;
    // // TODO: Fix dyn-incompatibility caused by this generic method.
    // fn serialize(&self, bytes: &mut impl BufMut) where Self: Sized;
}
impl<T: 'static> PrivateNbtCompatible for T { }

pub trait NbtCompatible: PrivateNbtCompatible {
    fn get_type_id(&self) -> u8;
    fn as_tag(self) -> NbtTag;
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
