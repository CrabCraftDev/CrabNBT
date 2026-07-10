use std::{fmt::Display, ops::{Index, IndexMut}};

use bytes::Bytes;
use derive_more::{From, TryInto};

use crate::{NbtCompound, nbt::{nbt_trait::NbtCompatible, utils::{ids::*, write_listlike}}};

macro_rules! call_uniform {
    (($self:ident.$($expression:tt)+), $end_case:expr) => {
        {
            use self::NbtList::*;
            match $self {
                End => $end_case,
                Byte(x) => x.$($expression)*,
                Short(x) => x.$($expression)*,
                Int(x) => x.$($expression)*,
                Long(x) => x.$($expression)*,
                Float(x) => x.$($expression)*,
                Double(x) => x.$($expression)*,
                ByteArray(x) => x.$($expression)*,
                String(x) => x.$($expression)*,
                List(x) => x.$($expression)*,
                Compound(x) => x.$($expression)*,
                IntArray(x) => x.$($expression)*,
                LongArray(x) => x.$($expression)*,
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, PartialOrd, From, TryInto)]
#[repr(u8)]
pub enum NbtList {
    // TODO: Everything about End.
    //  This is needed because an empty list serializes to a list of type END_TAG
    //  Need to check how this can be treated in various applications
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
    pub fn get_type_id(&self) -> u8 {
        use self::NbtList::*;
        match self {
            End => END_ID,
            Byte(_) => BYTE_ID,
            Short(_) => SHORT_ID,
            Int(_) => INT_ID,
            Long(_) => LONG_ID,
            Float(_) => FLOAT_ID,
            Double(_) => DOUBLE_ID,
            ByteArray(_)  => BYTE_ARRAY_ID,
            String(_) => STRING_ID,
            List(_) => LIST_ID,
            Compound(_) => COMPOUND_ID,
            IntArray(_) => INT_ARRAY_ID,
            LongArray(_) => LONG_ARRAY_ID
        }
    }

    pub fn get(&self, index: usize) -> Option<&dyn NbtCompatible> {
        call_uniform!((self.get(index).map(|x| x as &dyn NbtCompatible)), None)
    }

    pub fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut dyn NbtCompatible > {
        call_uniform!((self.get_mut(index).map(|x| x as &mut dyn NbtCompatible)), None)
    }

    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        self.into_iter()
    }
}
// Slice indexes are impossible on erased vecs, because there is no uniform output type
impl Index<usize> for NbtList {
    type Output = dyn NbtCompatible;

    fn index(&self, index: usize) -> &Self::Output {
        call_uniform!((self.index(index)), panic!("Index out of bounds for empty list. Index {index}"))
    }
}
impl IndexMut<usize> for NbtList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        call_uniform!((self.index_mut(index)), panic!("Index out of bounds for empty list. Index {index}"))
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
impl Display for NbtList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // call_uniform!((self, |x: _| write_listlike(f, "", "", x.iter())), Ok(()))
        write_listlike(f, "", "", self.iter().map(|e| e.snbt_dyn()))
    }
}

macro_rules! impl_TryAsRefAndMut {
    // empty lists are always serialized as lists of type TAG_END in NBT
    // such a list could have any type, so all TryAsRef and TryAsMut calls must succeed on it.
    // to ensure that no behaviour changes occur between reloading of a list, any TryAsRef and TryAsMut on empty lists succeeds.
    ($(($variant:ident, $type:ty)),+) => {
        use crate::{TryAsRef, TryAsMut};
        $(
            impl TryAsRef<Vec<$type>> for NbtList {
                fn try_as_ref(&self) -> Option<&Vec<$type>> {
                    static EMPTY_VEC: Vec<$type> = Vec::new();
                    match self {
                        Self::$variant(x) => Some(x),
                        _ => {
                            if self.is_empty() {
                                Some(&EMPTY_VEC)
                            } else {
                                None
                            }
                        }
                    }
                }
            }
            impl TryAsMut<Vec<$type>> for NbtList {
                fn try_as_mut(&mut self) -> Option<&mut Vec<$type>> {
                    match self {
                        Self::$variant(x) => Some(x),
                        _ => {
                            if self.is_empty() {
                                *self = Self::$variant(Vec::new());
                                match self {
                                    Self::$variant(x) => Some(x),
                                    _ => unreachable!()
                                }
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        )+
    };
}
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
// TODO: Is it better to implement or not to implement these methods?
implUniformMethods! {
    len(&;) -> usize | 0,
    capacity(&;) -> usize | 0,
    shrink_to_fit(&,mut;) | (),
    shrink_to(&,mut; min_capacity: usize) | (),
    truncate(&,mut; len: usize) | (),
    clear(&,mut;) | (),
    is_empty(&;) -> bool | true,
    dedup(&,mut;) | (),
    swap(&,mut; a: usize, b: usize) | panic!("Tried to swap on an empty list with. {a}<->{b}"),
    reverse(&,mut;) | (),
    rotate_left(&,mut; mid: usize) | if mid > 0 { panic!("Tried to rotate an empty list") },
    rotate_right(&,mut; k: usize) | if k > 0 { panic!("Tried to rotate an empty list") }
    
}

impl_TryAsRefAndMut! {
    (Byte, i8),
    (Short, i16),
    (Int, i32),
    (Long, i64),
    (Float, f32),
    (Double, f64),
    (ByteArray, Bytes),
    (String, String),
    (List, NbtList),
    (Compound, NbtCompound),
    (IntArray, Vec<i32>),
    (LongArray, Vec<i64>)
}

pub struct Iter<'a> {
    list: &'a NbtList,
    index: Option<usize>
}
impl<'a> Iter<'a> {
    fn new(list: &'a NbtList) -> Self {
        Self { list, index: Some(0) }
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
    index: Option<usize>
}
impl<'a> IterMut<'a> {
    fn new(list: &'a mut NbtList) -> Self {
        Self { list, index: Some(0) }
    }
}
impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut dyn NbtCompatible;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let val: Option<&'a mut dyn NbtCompatible> = self.list.get_mut(index)
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