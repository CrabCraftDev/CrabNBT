use crate::error::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_nbt::nbt::compound::NbtCompound;
use crab_nbt::nbt::tag::NbtTag;
use crab_nbt::nbt::utils::*;
use std::io::{Cursor, Write};
use std::ops::Deref;

pub mod compound;
pub mod tag;
pub mod utils;

/// Represents the main NBT structure.
/// It contains the root compound tag of the NBT structure and its associated name
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
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

    pub fn read(bytes: &mut impl Buf) -> Result<Nbt, Error> {
        let tag_type_id = bytes.get_u8();

        if tag_type_id != COMPOUND_ID {
            return Err(Error::NoRootCompound(tag_type_id));
        }

        Ok(Nbt {
            name: get_nbt_string(bytes)?,
            root_tag: NbtCompound::deserialize_content(bytes)?,
        })
    }

    pub fn read_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Nbt, Error> {
        Self::read(cursor)
    }

    /// Reads an NBT tag that doesn't contain the name of the root compound.
    /// Used in [Network NBT](https://wiki.vg/NBT#Network_NBT_(Java_Edition)).
    pub fn read_unnamed(bytes: &mut impl Buf) -> Result<Nbt, Error> {
        let tag_type_id = bytes.get_u8();

        if tag_type_id != COMPOUND_ID {
            return Err(Error::NoRootCompound(tag_type_id));
        }

        Ok(Nbt {
            name: String::new(),
            root_tag: NbtCompound::deserialize_content(bytes)?,
        })
    }

    pub fn read_unnamed_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Nbt, Error> {
        Self::read_unnamed(cursor)
    }

    pub fn write(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(COMPOUND_ID);
        bytes.put(NbtTag::String(self.name.to_string()).serialize_data());
        bytes.put(self.root_tag.serialize_content());
        bytes.freeze()
    }

    pub fn write_to_writer<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        writer.write_all(&self.write())?;
        Ok(())
    }

    /// Writes NBT tag, without name of root compound.
    /// Used in [Network NBT](https://wiki.vg/NBT#Network_NBT_(Java_Edition)).
    pub fn write_unnamed(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(COMPOUND_ID);
        bytes.put(self.root_tag.serialize_content());
        bytes.freeze()
    }

    pub fn write_unnamed_to_writer<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        writer.write_all(&self.write_unnamed())?;
        Ok(())
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

impl<T> AsRef<T> for Nbt
where
    T: ?Sized,
    <Nbt as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl AsMut<NbtCompound> for Nbt {
    fn as_mut(&mut self) -> &mut NbtCompound {
        &mut self.root_tag
    }
}
