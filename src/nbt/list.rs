use bytes::{BufMut, Bytes, BytesMut};

use crate::{NbtCompound, NbtTag};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NbtList {
    inner: Vec<NbtTag>,
    homogeneous: bool,
}

impl Default for NbtList {
    fn default() -> Self {
        Self::new()
    }
}

impl NbtList {
    pub fn new() -> NbtList {
        Self {
            inner: Vec::new(),
            homogeneous: true,
        }
    }

    pub fn with_capacity(initial_capacity: usize) -> NbtList {
        Self {
            inner: Vec::with_capacity(initial_capacity),
            homogeneous: true,
        }
    }

    pub fn is_homogeneous(&self) -> bool {
        self.homogeneous
    }

    pub fn is_heterogeneous(&self) -> bool {
        !self.homogeneous
    }

    pub fn get(&self, index: usize) -> Option<&NbtTag> {
        if self.homogeneous {
            return self.inner.get(index);
        }

        let NbtTag::Compound(compound) = &self.inner.get(index)? else {
            unreachable!()
        };

        if compound.child_tags.len() == 1 {
            let child = &compound.child_tags[0];
            if child.0.is_empty() {
                return Some(&child.1);
            }
        }

        Some(&self.inner[index])
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut NbtTag> {
        use polonius_the_crab::prelude::*;

        if self.homogeneous {
            return self.inner.get_mut(index);
        }

        let mut tag = self.inner.get_mut(index)?;
        polonius!(|tag| -> Option<&'polonius mut NbtTag> {
            let NbtTag::Compound(compound) = tag else {
                unreachable!()
            };

            if compound.child_tags.len() == 1 {
                let child = &mut compound.child_tags[0];
                if child.0.is_empty() {
                    polonius_return!(Some(&mut child.1))
                }
            }
        });

        Some(tag)
    }

    pub fn push(&mut self, element: impl Into<NbtTag>) {
        let element: NbtTag = element.into();
        let current_type_id = self.element_type_id();

        if self.homogeneous {
            let new_type_id = element.get_type_id();
            self.inner.push(element);
            if current_type_id != new_type_id && current_type_id != NbtTag::End.get_type_id() {
                self.make_heterogeneous();
            }
            return;
        }

        match element {
            NbtTag::Compound(_) => self.inner.push(element),
            _ => self.inner.push(NbtCompound::wrap(element).into()),
        }
    }

    pub fn make_heterogeneous(&mut self) {
        if self.is_heterogeneous() {
            return;
        }

        self.homogeneous = false;
        for e in &mut self.inner {
            if !matches!(e, NbtTag::Compound(_)) {
                *e = NbtCompound::wrap(std::mem::replace(e, NbtTag::End)).into();
            }
        }
    }

    pub fn into_native(self) -> Result<NbtTag, NbtList> {
        if self.is_heterogeneous() {
            return Err(self);
        }

        match self.element_type_id() {
            crate::nbt::utils::BYTE_ID => {
                let mut v = BytesMut::with_capacity(self.len());
                for e in self.into_iter() {
                    match e {
                        NbtTag::Byte(e) => v.put_i8(e),
                        _ => unreachable!(),
                    }
                }
                Ok(NbtTag::ByteArray(v.into()))
            }
            crate::nbt::utils::INT_ID => {
                let mut v = Vec::with_capacity(self.len());
                for e in self.into_iter() {
                    match e {
                        NbtTag::Int(e) => v.push(e),
                        _ => unreachable!(),
                    }
                }
                Ok(NbtTag::IntArray(v))
            }
            crate::nbt::utils::LONG_ID => {
                let mut v = Vec::with_capacity(self.len());
                for e in self.into_iter() {
                    match e {
                        NbtTag::Long(e) => v.push(e),
                        _ => unreachable!(),
                    }
                }
                Ok(NbtTag::LongArray(v))
            }
            _ => Err(self),
        }
    }

    pub fn into_inner(self) -> Vec<NbtTag> {
        self.inner
    }

    pub fn contains(&self, element: &NbtTag) -> bool {
        self.inner.contains(element)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn element_type_id(&self) -> u8 {
        if self.is_heterogeneous() {
            crate::nbt::utils::COMPOUND_ID
        } else if self.inner.is_empty() {
            crate::nbt::utils::END_ID
        } else {
            self.inner[0].get_type_id()
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        self.into_iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a> {
        self.into_iter()
    }
}

impl FromIterator<NbtTag> for NbtList {
    fn from_iter<T: IntoIterator<Item = NbtTag>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut list = NbtList::with_capacity(iter.size_hint().0);
        for element in iter {
            list.push(element);
        }

        list
    }
}

pub struct Iter<'a> {
    list: &'a NbtList,
    idx: usize,
}

pub struct IterMut<'a> {
    iter: std::slice::IterMut<'a, NbtTag>,
    homogeneous: bool,
}

pub struct IntoIter {
    list: NbtList,
    idx: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a NbtTag;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.list.get(self.idx)?;
        self.idx += 1;
        Some(element)
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut NbtTag;

    fn next(&mut self) -> Option<Self::Item> {
        use polonius_the_crab::prelude::*;

        let mut element = self.iter.next()?;

        if self.homogeneous {
            return Some(element);
        }

        polonius!(|element| -> Option<&'polonius mut NbtTag> {
            let NbtTag::Compound(compound) = element else {
                unreachable!()
            };

            if compound.child_tags.len() == 1 {
                let child = &mut compound.child_tags[0];
                if child.0.is_empty() {
                    polonius_return!(Some(&mut child.1))
                }
            }
        });

        Some(element)
    }
}

impl Iterator for IntoIter {
    type Item = NbtTag;

    fn next(&mut self) -> Option<Self::Item> {
        let tag = self.list.get_mut(self.idx)?;
        self.idx += 1;
        Some(std::mem::replace(tag, NbtTag::End))
    }
}

impl IntoIterator for NbtList {
    type Item = NbtTag;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self, idx: 0 }
    }
}

impl<'a> IntoIterator for &'a NbtList {
    type Item = &'a NbtTag;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { list: self, idx: 0 }
    }
}

impl<'a> IntoIterator for &'a mut NbtList {
    type Item = &'a mut NbtTag;

    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.inner.iter_mut(),
            homogeneous: self.homogeneous,
        }
    }
}

impl AsRef<[NbtTag]> for NbtList {
    fn as_ref(&self) -> &[NbtTag] {
        &self.inner
    }
}
