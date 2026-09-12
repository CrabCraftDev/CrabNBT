use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Index, IndexMut},
};

use bytes::{Buf, BufMut, Bytes};
use derive_more::{From, TryInto};

use crate::{
    error::Error,
    nbt::{
        nbt_trait::{NbtCompatible, PrivateNbtCompatible},
        utils::{
            compare_by,
            ids::{self, *},
            write_listlike,
        },
    },
    NbtCompound,
};

/// Helper macro to generate match cases to run code on the contents of an [NbtList].
///
/// Has two formats: Expression mode and Method mode.
/// - Expression mode is the most powerful, and can be used to execute
///   any expression on the content of the list, potentially returning a value.
/// - Method mode is specifically for executing [Vec] methods
///   (and methods of any type [Vec] [std::ops::Deref]s to).
///
/// ```
/// use crab_nbt::NbtList;
/// use crab_nbt::nbt_list_call_uniform;
/// let mut nbt_list = NbtList::Short(vec![1,2,3]);
/// // Expression mode
/// nbt_list_call_uniform!(
///     // may be any expression resulting in an NbtList or a borrowed form of it.
///     &mut nbt_list,
///     content,
///     content.push(Default::default()),
///     panic!("Cannot push default to an empty list")
/// );
/// assert_eq!(nbt_list, NbtList::Short(vec![1,2,3,0]));
///
/// // Method mode
/// // (note that this particular use case is already supported via NbtList::len)
/// assert_eq!(
///     nbt_list_call_uniform!((nbt_list.len()), 0),
///     4
/// );
/// ```
#[macro_export]
macro_rules! nbt_list_call_uniform {
    ($self:expr,$content:ident, $expression:expr, $end_case:expr) => {
        {
            match $self {
                // Manual reference for each value to avoid namespace pollution when using the macro
                // If we used a "use" statement for this, $expression and $end_case would inherit that context,
                // potentially causing undesirable type collisions
                $crate::NbtList::End => $end_case,
                $crate::NbtList::Byte($content) => $expression,
                $crate::NbtList::Short($content) => $expression,
                $crate::NbtList::Int($content) => $expression,
                $crate::NbtList::Long($content) => $expression,
                $crate::NbtList::Float($content) => $expression,
                $crate::NbtList::Double($content) => $expression,
                $crate::NbtList::ByteArray($content) => $expression,
                $crate::NbtList::String($content) => $expression,
                $crate::NbtList::List($content) => $expression,
                $crate::NbtList::Compound($content) => $expression,
                $crate::NbtList::IntArray($content) => $expression,
                $crate::NbtList::LongArray($content) => $expression,
            }
        }
    };
    (($self:ident.$($method:tt)+), $end_case:expr) => {
        nbt_list_call_uniform!($self,x, x.$($method)*, $end_case)
    };
}

#[derive(Default, Clone, Debug, PartialEq, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[repr(u8)]
pub enum NbtList {
    #[default] #[try_into(ignore)]
    End = END_ID,
    Byte(Vec<i8>) = BYTE_ID,
    Short(Vec<i16>) = SHORT_ID,
    Int(Vec<i32>) = INT_ID,
    Long(Vec<i64>) = LONG_ID,
    Float(Vec<f32>) = FLOAT_ID,
    Double(Vec<f64>) = DOUBLE_ID,
    ByteArray(Vec<Bytes>) = BYTE_ARRAY_ID,
    String(Vec<String>) = STRING_ID,
    List(Vec<NbtList>) = LIST_ID,
    Compound(Vec<NbtCompound>) = COMPOUND_ID,
    IntArray(Vec<Vec<i32>>) = INT_ARRAY_ID,
    LongArray(Vec<Vec<i64>>) = LONG_ARRAY_ID,
}
impl NbtList {
    pub fn get_content_type_id(&self) -> u8 {
        use self::NbtList::*;
        match self {
            End => END_ID,
            Byte(_) => BYTE_ID,
            Short(_) => SHORT_ID,
            Int(_) => INT_ID,
            Long(_) => LONG_ID,
            Float(_) => FLOAT_ID,
            Double(_) => DOUBLE_ID,
            ByteArray(_) => BYTE_ARRAY_ID,
            String(_) => STRING_ID,
            List(_) => LIST_ID,
            Compound(_) => COMPOUND_ID,
            IntArray(_) => INT_ARRAY_ID,
            LongArray(_) => LONG_ARRAY_ID,
        }
    }

    pub fn get(&self, index: usize) -> Option<&dyn NbtCompatible> {
        nbt_list_call_uniform!((self.get(index).map(|x| x as &dyn NbtCompatible)), None)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut dyn NbtCompatible> {
        nbt_list_call_uniform!(
            (self.get_mut(index).map(|x| x as &mut dyn NbtCompatible)),
            None
        )
    }

    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        self.into_iter()
    }

    /// Unpacks this list and converts it into a "heterogeneous" list of items
    /// wrapped in NbtCompounds like so: `{"": <element}`.
    ///
    /// This operation will always wrap the list __even if it is already a wrapped list__!
    /// If you only want to ensure your list is "heterogeneous", use [NbtList::ensure_wrapped].
    pub fn into_wrapped(self) -> Vec<NbtCompound> {
        nbt_list_call_uniform!(
            self,
            content,
            content
                .into_iter()
                .map(|element| NbtCompound {
                    child_tags: vec![(String::new(), element.into_tag())]
                })
                .collect(),
            vec![]
        )
    }

    /// Ensures that this list is a wrapped list, if it wasn't already.
    ///
    /// More formally, this method checks whether the list is currently wrapped.
    /// If it is not, it calls [NbtList::into_wrapped] and replaces this list with the result.
    ///
    /// Returns a mutable reference to the contents of `self`, after potentially having wrapped its prior contents.
    pub fn ensure_wrapped(&mut self) -> &mut Vec<NbtCompound> {
        if !self.is_wrapped() {
            *self = NbtList::Compound(std::mem::take(self).into_wrapped());
        }
        match self {
            NbtList::Compound(x) => x,
            _ => unreachable!("list must be a compound list"),
        }
    }

    /// Checks whether this list is a wrapped list.
    ///
    /// A list is considered wrapped, if its children are [NbtCompound]s
    /// with only one element, whose key is the empty string.
    /// This is the serialised result of heterogeneous lists in SNBT.
    pub fn is_wrapped(&self) -> bool {
        match self {
            Self::Compound(contents) => {
                // TODO: This check is probably too expensive for real-world use.
                contents.iter().all(|compound| {
                    let tags = &compound.child_tags;
                    tags.len() == 1 && tags[0].0.is_empty()
                })
            }
            _ => false,
        }
    }

    fn deser_list_helper<T: PrivateNbtCompatible>(
        bytes: &mut impl Buf,
        len: usize,
        wrapper: impl FnOnce(Vec<T>) -> NbtList,
    ) -> Result<NbtList, Error> {
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            list.push(T::deserialize_data(bytes)?);
        }
        Ok(wrapper(list))
    }

    fn ser_list_helper<T: PrivateNbtCompatible>(inner: &[T], bytes: &mut impl BufMut) {
        if inner.is_empty() {
            bytes.put_u8(ids::END_ID);
            bytes.put_i32(0);
        } else {
            bytes.put_u8(T::get_id());
            bytes.put_i32(inner.len() as i32);
            for element in inner.iter() {
                element.serialize_content_into(bytes);
            }
        }
    }
}
// Slices are impossible on erased vecs, because there is no uniform output type
impl Index<usize> for NbtList {
    type Output = dyn NbtCompatible;

    fn index(&self, index: usize) -> &Self::Output {
        nbt_list_call_uniform!(
            (self.index(index)),
            panic!("Index out of bounds for empty list. Index {index}")
        )
    }
}
impl IndexMut<usize> for NbtList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        nbt_list_call_uniform!(
            (self.index_mut(index)),
            panic!("Index out of bounds for empty list. Index {index}")
        )
    }
}
impl<'a> IntoIterator for &'a NbtList {
    type Item = &'a dyn NbtCompatible;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}
impl<'a> IntoIterator for &'a mut NbtList {
    type Item = &'a mut dyn NbtCompatible;

    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}
impl Eq for NbtList {}
impl Ord for NbtList {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Byte(a), Self::Byte(b)) => a.cmp(b),
            (Self::Short(a), Self::Short(b)) => a.cmp(b),
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Long(a), Self::Long(b)) => a.cmp(b),
            // This can be done using cmp_by when it is stabilised
            (Self::Float(a), Self::Float(b)) => compare_by(a, b, f32::total_cmp),
            (Self::Double(a), Self::Double(b)) => compare_by(a, b, f64::total_cmp),
            (Self::ByteArray(a), Self::ByteArray(b)) => a.cmp(b),
            (Self::String(a), Self::String(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Compound(a), Self::Compound(b)) => a.cmp(b),
            (Self::IntArray(a), Self::IntArray(b)) => a.cmp(b),
            (Self::LongArray(a), Self::LongArray(b)) => a.cmp(b),
            _ => self.get_content_type_id().cmp(&other.get_content_type_id()),
        }
    }
}
impl PartialOrd for NbtList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PrivateNbtCompatible for NbtList {
    fn deserialize_data(bytes: &mut impl Buf) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let tag_type_id = bytes.try_get_u8()?;
        let len = bytes.try_get_i32()?;
        match tag_type_id {
            ids::END_ID => Ok(NbtList::End),
            ids::BYTE_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::Byte),
            ids::SHORT_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::Short),
            ids::INT_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::Int),
            ids::LONG_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::Long),
            ids::FLOAT_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::Float),
            ids::DOUBLE_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::Double),
            ids::BYTE_ARRAY_ID => {
                NbtList::deser_list_helper(bytes, len as usize, NbtList::ByteArray)
            }
            ids::STRING_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::String),
            ids::LIST_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::List),
            ids::COMPOUND_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::Compound),
            ids::INT_ARRAY_ID => NbtList::deser_list_helper(bytes, len as usize, NbtList::IntArray),
            ids::LONG_ARRAY_ID => {
                NbtList::deser_list_helper(bytes, len as usize, NbtList::LongArray)
            }
            _ => Err(Error::UnknownTagId(tag_type_id)),
        }
    }

    fn serialize_content_into(&self, bytes: &mut impl bytes::BufMut)
    where
        Self: Sized,
    {
        nbt_list_call_uniform!(self, content, NbtList::ser_list_helper(content, bytes), {
            bytes.put_u8(ids::END_ID);
            bytes.put_i32(0);
        })
    }

    fn write_snbt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_listlike(f, "", "", self.iter().map(|el| el.snbt_dyn()))
    }

    fn get_id() -> u8
    where
        Self: Sized,
    {
        ids::LIST_ID
    }
}
impl Display for NbtList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // call_uniform!((self, |x: _| write_listlike(f, "", "", x.iter())), Ok(()))
        write_listlike(f, "", "", self.iter().map(|e| e.snbt_dyn()))
    }
}

/// Implements [`Vec`] methods on the NbtList by delegating the method call.
/// Can only be used on methods whose signature is independent of the list type.
///
/// The `on_end` expression (the expression after the `|`) **must** be identical
/// to the return value on every other variant of [`NbtList`] when the contained [`Vec`] is empty.
macro_rules! implUniformMethods {
    ($($method:ident($($modifiers:tt),*;$($param_name:ident: $param_type:ty),*) $(-> $return:ty)? | $on_end:expr),+) => {
        impl NbtList {
            $(
                pub fn $method($($modifiers)*self, $($param_name: $param_type),*) $(-> $return)? {
                    use self::NbtList::*;
                    match self {
                        End => $on_end,
                        Byte(x) => x.$method($($param_name),*),
                        Short(x) => x.$method($($param_name),*),
                        Int(x) => x.$method($($param_name),*),
                        Long(x) => x.$method($($param_name),*),
                        Float(x) => x.$method($($param_name),*),
                        Double(x) => x.$method($($param_name),*),
                        ByteArray(x) => x.$method($($param_name),*),
                        String(x) => x.$method($($param_name),*),
                        List(x) => x.$method($($param_name),*),
                        Compound(x) => x.$method($($param_name),*),
                        IntArray(x) => x.$method($($param_name),*),
                        LongArray(x) => x.$method($($param_name),*),
                    }
                }
            )+
        }
    };
}

implUniformMethods! {
    len(&;) -> usize | 0,
    capacity(&;) -> usize | 0,
    shrink_to_fit(&,mut;) | (),
    shrink_to(&,mut; min_capacity: usize) | (),
    truncate(&,mut; len: usize) | (),
    clear(&,mut;) | (),
    is_empty(&;) -> bool | true,
    dedup(&,mut;) | (),
    swap(&,mut; a: usize, b: usize) | panic!("Tried to swap on an empty list with {a}<->{b}"),
    reverse(&,mut;) | (),
    rotate_left(&,mut; mid: usize) | if mid > 0 { panic!("Tried to rotate an empty list") },
    rotate_right(&,mut; k: usize) | if k > 0 { panic!("Tried to rotate an empty list") }
}

pub struct Iter<'a> {
    list: &'a NbtList,
    index: Option<usize>,
}
impl<'a> Iter<'a> {
    fn new(list: &'a NbtList) -> Self {
        Self {
            list,
            index: Some(0),
        }
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a dyn NbtCompatible;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let val = self.list.get(index);
        if val.is_some() {
            self.index = Some(index.strict_add(1));
        } else {
            self.index = None;
        }
        val
    }
}

pub struct IterMut<'a> {
    list: &'a mut NbtList,
    index: Option<usize>,
}
impl<'a> IterMut<'a> {
    fn new(list: &'a mut NbtList) -> Self {
        Self {
            list,
            index: Some(0),
        }
    }
}
impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut dyn NbtCompatible;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let val: Option<&'a mut dyn NbtCompatible> = self
            .list
            .get_mut(index)
            // The borrow checker is being too eager with get_mut and degrades lifetime 'a to the lifetime of self.
            // For reasons unknown to me this only happens with mutable pointers.
            // Other methods of lifetime expansion (such as pointer casting) have proven unsuccessful, so transmute is used as a last resort.
            // SAFETY:
            //  Expanding the lifetime is safe in this instance, because the borrow checker already guarantees
            //  that self.list will live for 'a, because we are holding a borrow of it for that lifetime.
            //  get_mut ensures that its return value has lifetime 'a as well, so our reference must live for at least that long.
            .map(|r| unsafe { std::mem::transmute(r) });
        if val.is_some() {
            self.index = Some(index.strict_add(1));
        } else {
            self.index = None;
        }
        val
    }
}
