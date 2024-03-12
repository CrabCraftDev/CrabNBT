use crate::error::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_nbt::nbt::compound::NbtCompound;
use crab_nbt::nbt::tag::NbtTag;
use crab_nbt::nbt::utils::*;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct Nbt {
    pub name: String,
    pub tag: NbtCompound,
}

impl Nbt {
    pub fn new(name: &str, tag: NbtCompound) -> Self {
        Nbt {
            name: name.to_string(),
            tag,
        }
    }

    pub fn read(bytes: &mut Bytes, is_network: bool) -> Result<Nbt, Error> {
        let tag_type_id = bytes.get_u8();

        if tag_type_id != COMPOUND_ID {
            return Err(Error::NoRootCompound(tag_type_id));
        }

        let mut compound_name = String::new();
        if !is_network {
            compound_name = get_nbt_string(bytes).unwrap();
        }

        Ok(Nbt {
            name: compound_name,
            tag: NbtCompound::deserialize(bytes),
        })
    }

    pub fn write(&self, is_network: bool) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(COMPOUND_ID);

        if !is_network {
            bytes.put(NbtTag::String(self.name.to_string()).serialize_raw());
        }

        bytes.put(self.tag.serialize());
        bytes.put_u8(END_ID);

        bytes.freeze()
    }
}

impl Deref for Nbt {
    type Target = NbtCompound;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
