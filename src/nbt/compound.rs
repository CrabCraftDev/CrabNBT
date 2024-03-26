use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_nbt::nbt::tag::NbtTag;
use crab_nbt::nbt::utils::{get_nbt_string, END_ID};
use derive_more::Into;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Default, Into)]
pub struct NbtCompound {
    pub child_tags: HashMap<String, NbtTag>,
}

impl NbtCompound {
    pub fn new(child_tags: HashMap<String, NbtTag>) -> NbtCompound {
        NbtCompound { child_tags }
    }

    pub(crate) fn deserialize(bytes: &mut Bytes) -> NbtCompound {
        let mut compound_tags = HashMap::new();

        while !bytes.is_empty() {
            let tag_id = bytes.get_u8();
            if tag_id == END_ID {
                break;
            }

            let name = get_nbt_string(bytes).unwrap();

            if let Ok(tag) = NbtTag::deserialize_raw(bytes, tag_id) {
                compound_tags.insert(name, tag);
            } else {
                break;
            }
        }

        NbtCompound {
            child_tags: compound_tags,
        }
    }

    pub(crate) fn serialize(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        for (name, tag) in &self.child_tags {
            bytes.put(tag.serialize_named(name));
        }
        bytes.put_u8(END_ID);
        bytes.freeze()
    }

    pub fn from_values(values: Vec<(&str, NbtTag)>) -> Self {
        let mut child_tags = HashMap::new();
        for (name, tag) in values {
            let name_string = name.to_string();
            child_tags.insert(name_string, tag);
        }
        Self { child_tags }
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

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.child_tags.get(name).and_then(|tag| tag.extract_bool())
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
