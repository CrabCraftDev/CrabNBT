use std::ops::{Index, IndexMut, RangeBounds};

use bytes::{BufMut, BytesMut};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum MakeHomogeneousError {
    #[error("Not homogeneous")]
    NotHomogeneous,
    #[error("Non-Compound element in heterogeneous List")]
    NonCompoundElement,
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

    pub fn reserve(&mut self, additional_capacity: usize) {
        self.inner.reserve(additional_capacity)
    }

    pub fn reserve_exact(&mut self, additional_capacity: usize) {
        self.inner.reserve_exact(additional_capacity)
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

    pub fn insert(&mut self, index: usize, element: impl Into<NbtTag>) {
        let element: NbtTag = element.into();
        let current_type_id = self.element_type_id();

        if self.homogeneous {
            let new_type_id = element.get_type_id();
            self.inner.insert(index, element);
            if current_type_id != new_type_id && current_type_id != NbtTag::End.get_type_id() {
                self.make_heterogeneous();
            }
            return;
        }

        match element {
            NbtTag::Compound(_) => self.inner.insert(index, element),
            _ => self.inner.insert(index, NbtCompound::wrap(element).into()),
        }
    }

    pub fn remove(&mut self, index: usize) -> NbtTag {
        self.inner.remove(index)
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

    pub fn make_homogeneous(&mut self) -> Result<(), MakeHomogeneousError> {
        if self.homogeneous {
            return Ok(());
        }

        if self.is_empty() {
            self.homogeneous = true;
            return Ok(());
        }

        let first_element_id = self.get(0).unwrap().get_type_id();
        let mut new = Vec::with_capacity(self.len());
        for e in self.into_iter() {
            if e.get_type_id() != first_element_id {
                return Err(MakeHomogeneousError::NotHomogeneous);
            }

            let NbtTag::Compound(ref mut compound) = e else {
                return Err(MakeHomogeneousError::NonCompoundElement);
            };

            if compound.child_tags.len() == 1 && compound.child_tags[0].0.is_empty() {
                new.push(compound.child_tags.remove(0).1);
            } else {
                new.push(NbtTag::Compound(std::mem::replace(
                    compound,
                    NbtCompound::new(),
                )));
            }
        }
        self.homogeneous = true;

        Ok(())
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

    pub fn as_inner(&self) -> &Vec<NbtTag> {
        &self.inner
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

    pub fn extend<T, I>(&mut self, iter: T)
    where
        T: IntoIterator<Item = I>,
        I: Into<NbtTag>,
    {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        iter.into_iter().for_each(|e| self.push(e.into()));
    }

    pub fn retain(&mut self, condition: impl FnMut(&NbtTag) -> bool) {
        self.inner.retain(condition)
    }

    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> std::vec::Drain<'_, NbtTag> {
        self.inner.drain(range)
    }
}

impl From<Vec<NbtTag>> for NbtList {
    fn from(inner: Vec<NbtTag>) -> Self {
        let mut all_compounds = true;
        let mut any_singletons = false;
        for e in &inner {
            if let NbtTag::Compound(compound) = &e {
                if compound.child_tags.len() == 1 && compound.child_tags[0].0.is_empty() {
                    any_singletons = true;
                }
            } else {
                all_compounds = false;
            }
        }

        Self {
            inner,
            homogeneous: !(all_compounds && any_singletons),
        }
    }
}

impl FromIterator<NbtTag> for NbtList {
    fn from_iter<T: IntoIterator<Item = NbtTag>>(iter: T) -> Self {
        Vec::from_iter(iter).into()
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

impl Index<usize> for NbtList {
    type Output = NbtTag;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<usize> for NbtList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
