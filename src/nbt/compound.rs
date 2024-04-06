use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_nbt::nbt::tag::NbtTag;
use crab_nbt::nbt::utils::{get_nbt_string, END_ID};
use derive_more::Into;
use std::collections::{hash_map::IntoIter, HashMap};

use crate::{error::Error, Nbt};

#[derive(Clone, PartialEq, Debug, Default, Into)]
pub struct NbtCompound {
    pub child_tags: HashMap<String, NbtTag>,
}

impl NbtCompound {
    pub fn new() -> NbtCompound {
        NbtCompound {
            child_tags: HashMap::new(),
        }
    }

    pub fn deserialize_content(bytes: &mut Bytes) -> Result<NbtCompound, Error> {
        let mut child_tags = HashMap::new();

        while !bytes.is_empty() {
            let tag_id = bytes.get_u8();
            if tag_id == END_ID {
                break;
            }

            let name = get_nbt_string(bytes)?;

            if let Ok(tag) = NbtTag::deserialize_data(bytes, tag_id) {
                child_tags.insert(name, tag);
            } else {
                break;
            }
        }

        Ok(NbtCompound { child_tags })
    }

    pub fn serialize_content(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        for (name, tag) in &self.child_tags {
            bytes.put_u8(tag.id());
            bytes.put(NbtTag::String(name.clone()).serialize_data());
            bytes.put(tag.serialize_data());
        }
        bytes.put_u8(END_ID);
        bytes.freeze()
    }

    pub fn get_byte(&self, name: &str) -> Option<i8> {
        self.child_tags.get(name).and_then(|tag| tag.extract_byte())
    }

    pub fn get_short(&self, name: &str) -> Option<i16> {
        self.child_tags
            .get(name)
            .and_then(|tag| tag.extract_short())
    }

    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.child_tags.get(name).and_then(|tag| tag.extract_int())
    }

    pub fn get_long(&self, name: &str) -> Option<i64> {
        self.child_tags.get(name).and_then(|tag| tag.extract_long())
    }

    pub fn get_float(&self, name: &str) -> Option<f32> {
        self.child_tags
            .get(name)
            .and_then(|tag| tag.extract_float())
    }

    pub fn get_double(&self, name: &str) -> Option<f64> {
        self.child_tags
            .get(name)
            .and_then(|tag| tag.extract_double())
    }

    pub fn get_string(&self, name: &str) -> Option<&String> {
        self.child_tags
            .get(name)
            .and_then(|tag| tag.extract_string())
    }

    pub fn get_list(&self, name: &str) -> Option<&Vec<NbtTag>> {
        self.child_tags.get(name).and_then(|tag| tag.extract_list())
    }

    pub fn get_compound(&self, name: &str) -> Option<&NbtCompound> {
        self.child_tags
            .get(name)
            .and_then(|tag| tag.extract_compound())
    }

    pub fn get_int_array(&self, name: &str) -> Option<&Vec<i32>> {
        self.child_tags
            .get(name)
            .and_then(|tag| tag.extract_int_array())
    }

    pub fn get_long_array(&self, name: &str) -> Option<&Vec<i64>> {
        self.child_tags
            .get(name)
            .and_then(|tag| tag.extract_long_array())
    }
}

impl From<Nbt> for NbtCompound {
    fn from(value: Nbt) -> Self {
        value.root_tag
    }
}

impl FromIterator<(String, NbtTag)> for NbtCompound {
    fn from_iter<T: IntoIterator<Item = (String, NbtTag)>>(iter: T) -> Self {
        Self {
            child_tags: HashMap::from_iter(iter),
        }
    }
}

impl IntoIterator for NbtCompound {
    type Item = (String, NbtTag);

    type IntoIter = IntoIter<String, NbtTag>;

    fn into_iter(self) -> Self::IntoIter {
        self.child_tags.into_iter()
    }
}

impl Extend<(String, NbtTag)> for NbtCompound {
    fn extend<T: IntoIterator<Item = (String, NbtTag)>>(&mut self, iter: T) {
        self.child_tags.extend(iter)
    }
}
