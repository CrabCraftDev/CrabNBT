use crate::{error::Error, slice_cursor::BinarySliceCursor, Nbt};
use crab_nbt::nbt::tag::NbtTag;
use crab_nbt::nbt::utils::{get_nbt_string, END_ID};
use derive_more::Into;
use std::io::{Cursor, Write};
use std::vec::IntoIter;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Into)]
pub struct NbtCompound {
    pub child_tags: Vec<(String, NbtTag)>,
}

impl NbtCompound {
    pub fn new() -> NbtCompound {
        NbtCompound {
            child_tags: Vec::new(),
        }
    }

    pub fn deserialize_content(bytes: &mut BinarySliceCursor) -> Result<NbtCompound, Error> {
        let mut compound = NbtCompound::new();

        while bytes.has_remaining() {
            let tag_id = bytes.read_u8()?;
            if tag_id == END_ID {
                break;
            }

            let name = get_nbt_string(bytes)?;

            if let Ok(tag) = NbtTag::deserialize_data(bytes, tag_id) {
                compound.put(name, tag);
            } else {
                break;
            }
        }

        Ok(compound)
    }

    pub fn deserialize_content_from_cursor(
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<NbtCompound, Error> {
        Self::deserialize_content(&mut BinarySliceCursor::new(cursor.get_ref()))
    }

    pub fn serialize_content(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (name, tag) in &self.child_tags {
            bytes.push(tag.get_type_id());
            bytes.extend(NbtTag::String(name.clone()).serialize_data());
            bytes.extend(tag.serialize_data());
        }
        bytes.push(END_ID);
        bytes
    }

    pub fn serialize_content_to_writer<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        writer.write_all(&self.serialize_content())?;
        Ok(())
    }

    pub fn put(&mut self, name: String, value: impl Into<NbtTag>) {
        if !self.child_tags.iter().any(|(key, _)| key == &name) {
            self.child_tags.push((name, value.into()));
        }
    }

    pub fn get_byte(&self, name: &str) -> Option<i8> {
        self.get(name).and_then(|tag| tag.extract_byte())
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<&NbtTag> {
        for (key, value) in &self.child_tags {
            if key.as_str() == name {
                return Some(value);
            }
        }
        None
    }

    pub fn get_short(&self, name: &str) -> Option<i16> {
        self.get(name).and_then(|tag| tag.extract_short())
    }

    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.get(name).and_then(|tag| tag.extract_int())
    }

    pub fn get_long(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(|tag| tag.extract_long())
    }

    pub fn get_float(&self, name: &str) -> Option<f32> {
        self.get(name).and_then(|tag| tag.extract_float())
    }

    pub fn get_double(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(|tag| tag.extract_double())
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.get(name).and_then(|tag| tag.extract_bool())
    }

    pub fn get_string(&self, name: &str) -> Option<&String> {
        self.get(name).and_then(|tag| tag.extract_string())
    }

    pub fn get_list(&self, name: &str) -> Option<&Vec<NbtTag>> {
        self.get(name).and_then(|tag| tag.extract_list())
    }

    pub fn get_compound(&self, name: &str) -> Option<&NbtCompound> {
        self.get(name).and_then(|tag| tag.extract_compound())
    }

    pub fn get_int_array(&self, name: &str) -> Option<&Vec<i32>> {
        self.get(name).and_then(|tag| tag.extract_int_array())
    }

    pub fn get_long_array(&self, name: &str) -> Option<&Vec<i64>> {
        self.get(name).and_then(|tag| tag.extract_long_array())
    }
}

impl From<Nbt> for NbtCompound {
    fn from(value: Nbt) -> Self {
        value.root_tag
    }
}

impl FromIterator<(String, NbtTag)> for NbtCompound {
    fn from_iter<T: IntoIterator<Item = (String, NbtTag)>>(iter: T) -> Self {
        let mut compound = NbtCompound::new();
        for (key, value) in iter {
            compound.put(key, value);
        }
        compound
    }
}

impl IntoIterator for NbtCompound {
    type Item = (String, NbtTag);
    type IntoIter = IntoIter<(String, NbtTag)>;

    fn into_iter(self) -> Self::IntoIter {
        self.child_tags.into_iter()
    }
}

impl Extend<(String, NbtTag)> for NbtCompound {
    fn extend<T: IntoIterator<Item = (String, NbtTag)>>(&mut self, iter: T) {
        self.child_tags.extend(iter)
    }
}

// Rust's AsRef is currently not reflexive so we need to implement it manually
impl AsRef<NbtCompound> for NbtCompound {
    fn as_ref(&self) -> &NbtCompound {
        self
    }
}
