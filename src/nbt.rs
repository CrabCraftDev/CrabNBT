use crate::error::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_nbt::nbt::compound::NbtCompound;
use crab_nbt::nbt::tag::NbtTag;
use crab_nbt::nbt::utils::*;
use std::ops::Deref;

pub mod compound;
pub mod tag;
mod utils;

/// Representation of the root nbt tag.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Nbt {
    pub name: String,
    pub root_tag: NbtCompound,
}

impl Nbt {
    pub fn new(name: String, tag: NbtCompound) -> Self {
        Nbt {
            name,
            root_tag: tag,
        }
    }

    pub fn read(bytes: &mut Bytes) -> Result<Nbt, Error> {
        let tag_type_id = bytes.get_u8();

        if tag_type_id != COMPOUND_ID {
            return Err(Error::NoRootCompound(tag_type_id));
        }

        Ok(Nbt {
            name: get_nbt_string(bytes)?,
            root_tag: NbtCompound::deserialize_raw(bytes)?,
        })
    }

    /// Reads Nbt tag, that doesn't contain the name of root compound
    /// Used in [Network NBT](https://wiki.vg/NBT#Network_NBT_(Java_Edition))
    pub fn read_unnamed(bytes: &mut Bytes) -> Result<Nbt, Error> {
        let tag_type_id = bytes.get_u8();

        if tag_type_id != COMPOUND_ID {
            return Err(Error::NoRootCompound(tag_type_id));
        }

        Ok(Nbt {
            name: String::new(),
            root_tag: NbtCompound::deserialize_raw(bytes)?,
        })
    }

    pub fn write(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(COMPOUND_ID);
        bytes.put(NbtTag::String(self.name.to_string()).serialize_raw());

        bytes.put(self.root_tag.serialize_raw());
        bytes.freeze()
    }

    /// Writes Nbt tag, without name of root compound
    /// Used in [Network NBT](https://wiki.vg/NBT#Network_NBT_(Java_Edition))
    pub fn write_unnamed(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(COMPOUND_ID);
        bytes.put(self.root_tag.serialize_raw());
        bytes.freeze()
    }
}

impl Deref for Nbt {
    type Target = NbtCompound;

    fn deref(&self) -> &Self::Target {
        &self.root_tag
    }
}

impl From<NbtCompound> for Nbt {
    fn from(value: NbtCompound) -> Self {
        Nbt::new(String::new(), value)
    }
}

impl AsRef<NbtCompound> for Nbt {
    fn as_ref(&self) -> &NbtCompound {
        &self.root_tag
    }
}

impl AsMut<NbtCompound> for Nbt {
    fn as_mut(&mut self) -> &mut NbtCompound {
        &mut self.root_tag
    }
}
