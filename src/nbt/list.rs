use crate::{NbtCompound, NbtTag};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NbtList {
    Homogeneous((u8, Vec<NbtTag>)),
    Heterogeneous(Vec<NbtCompound>),
}

impl Default for NbtList {
    fn default() -> Self {
        Self::new()
    }
}

impl NbtList {
    pub fn new() -> NbtList {
        Self::Homogeneous((NbtTag::End.get_type_id(), Vec::new()))
    }

    pub fn with_capacity(initial_capacity: usize) -> NbtList {
        Self::Homogeneous((
            NbtTag::End.get_type_id(),
            Vec::with_capacity(initial_capacity),
        ))
    }

    pub fn push(mut self, element: impl Into<NbtTag>) -> Self {
        let element: NbtTag = element.into();
        match self {
            NbtList::Homogeneous((mut ty, ref mut v)) => {
                let type_id = element.get_type_id();
                if ty == type_id || ty == NbtTag::End.get_type_id() {
                    ty = type_id;
                    v.push(element);
                    NbtList::Homogeneous((ty, std::mem::take(v)))
                } else {
                    let new = self.into_heterogeneous();
                    new.push(element)
                }
            }
            NbtList::Heterogeneous(mut v) => {
                if let NbtTag::Compound(compound) = element {
                    v.push(compound);
                } else {
                    v.push(NbtCompound {
                        child_tags: vec![("".to_string(), element)],
                    });
                }
                NbtList::Heterogeneous(v)
            }
        }
    }

    pub fn into_heterogeneous(self) -> Self {
        match self {
            NbtList::Homogeneous((_, a)) => {
                let mut b: Vec<NbtCompound> = Vec::with_capacity(a.capacity());
                for e in a {
                    b.push(NbtCompound {
                        child_tags: vec![("".to_string(), e)],
                    })
                }
                NbtList::Heterogeneous(b)
            }
            NbtList::Heterogeneous(_) => self,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            NbtList::Homogeneous((_, v)) => v.len(),
            NbtList::Heterogeneous(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_homogeneous(&self) -> Option<&Vec<NbtTag>> {
        match self {
            NbtList::Homogeneous((_, v)) => Some(v),
            NbtList::Heterogeneous(_) => None,
        }
    }

    pub fn as_heterogeneous(&self) -> Option<&Vec<NbtCompound>> {
        match self {
            NbtList::Homogeneous(_) => None,
            NbtList::Heterogeneous(v) => Some(v),
        }
    }

    pub fn element_type_id(&self) -> u8 {
        match self {
            NbtList::Homogeneous((ty, _)) => *ty,
            NbtList::Heterogeneous(_) => crate::nbt::utils::COMPOUND_ID,
        }
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

impl IntoIterator for NbtList {
    type Item = NbtTag;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            NbtList::Homogeneous((_, v)) => Box::new(v.into_iter()),
            NbtList::Heterogeneous(v) => Box::new(v.into_iter().map(NbtTag::Compound)),
        }
    }
}
