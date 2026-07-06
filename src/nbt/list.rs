use crate::{NbtCompound, NbtTag};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NbtList {
    Homogeneous(Vec<NbtTag>),
    Heterogeneous(Vec<NbtTag>),
}

impl Default for NbtList {
    fn default() -> Self {
        Self::new()
    }
}

impl NbtList {
    pub fn new() -> NbtList {
        Self::Homogeneous(Vec::new())
    }

    pub fn with_capacity(initial_capacity: usize) -> NbtList {
        Self::Homogeneous(Vec::with_capacity(initial_capacity))
    }

    pub fn get(&self, index: usize) -> Option<&NbtTag> {
        match self {
            NbtList::Homogeneous(v) => v.get(index),
            NbtList::Heterogeneous(v) => {
                let NbtTag::Compound(compound) = &v.get(index)? else {
                    unreachable!()
                };

                if compound.child_tags.len() == 1 {
                    let child = &compound.child_tags[0];
                    if child.0.is_empty() {
                        return Some(&child.1);
                    }
                }

                Some(&v[index])
            }
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut NbtTag> {
        match self {
            NbtList::Homogeneous(v) => v.get_mut(index),
            NbtList::Heterogeneous(ref mut v) => {
                use polonius_the_crab::prelude::*;

                let mut tag = v.get_mut(index)?;
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
        }
    }

    pub fn push(mut self, element: impl Into<NbtTag>) -> Self {
        let element: NbtTag = element.into();
        let current_type_id = self.element_type_id();

        match self {
            NbtList::Homogeneous(ref mut v) => {
                let new_type_id = element.get_type_id();
                if current_type_id == new_type_id || current_type_id == NbtTag::End.get_type_id() {
                    v.push(element);
                    NbtList::Homogeneous(std::mem::take(v))
                } else {
                    let new = self.into_heterogeneous();
                    new.push(element)
                }
            }
            NbtList::Heterogeneous(mut v) => {
                match element {
                    NbtTag::Compound(_) => {
                        v.push(element);
                    }
                    _ => v.push(NbtCompound::wrap(element).into()),
                }
                NbtList::Heterogeneous(v)
            }
        }
    }

    pub fn into_heterogeneous(self) -> Self {
        match self {
            NbtList::Homogeneous(mut v) => {
                for e in &mut v {
                    if !matches!(e, NbtTag::Compound(_)) {
                        *e = NbtCompound::wrap(std::mem::replace(e, NbtTag::End)).into();
                    }
                }
                NbtList::Heterogeneous(v)
            }
            NbtList::Heterogeneous(_) => self,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            NbtList::Homogeneous(v) => v.len(),
            NbtList::Heterogeneous(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn element_type_id(&self) -> u8 {
        match self {
            NbtList::Homogeneous(v) => {
                if v.is_empty() {
                    crate::nbt::utils::END_ID
                } else {
                    v[0].get_type_id()
                }
            }
            NbtList::Heterogeneous(_) => crate::nbt::utils::COMPOUND_ID,
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        self.into_iter()
    }
}

impl FromIterator<NbtTag> for NbtList {
    fn from_iter<T: IntoIterator<Item = NbtTag>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut list = NbtList::with_capacity(iter.size_hint().0);
        for element in iter {
            list = list.push(element);
        }

        list
    }
}

pub struct Iter<'a> {
    list: &'a NbtList,
    idx: usize,
}

// TODO: IterMut

pub struct IntoIter {
    list: NbtList,
    idx: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a NbtTag;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.list.get(self.idx);
        if element.is_some() {
            self.idx += 1;
        }
        element
    }
}

impl Iterator for IntoIter {
    type Item = NbtTag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.len() > self.idx {
            let element = match self.list {
                NbtList::Homogeneous(ref mut v) => std::mem::replace(&mut v[self.idx], NbtTag::End),
                NbtList::Heterogeneous(ref mut v) => {
                    let NbtTag::Compound(mut compound) =
                        std::mem::replace(&mut v[self.idx], NbtTag::End)
                    else {
                        unreachable!()
                    };

                    if compound.child_tags.len() == 1 {
                        let child = &mut compound.child_tags[0];
                        if child.0.is_empty() {
                            self.idx += 1;
                            return Some(std::mem::replace(&mut child.1, NbtTag::End));
                        }
                    }
                    compound.into()
                }
            };
            self.idx += 1;
            Some(element)
        } else {
            None
        }
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
