use crate::error::{Error, Result};
use crate::nbt::utils::{
    BYTE_ARRAY_ID, BYTE_ID, COMPOUND_ID, INT_ARRAY_ID, INT_ID, LIST_ID, LONG_ARRAY_ID, LONG_ID,
};
use crate::NbtTag;
use bytes::{Buf, BytesMut};
use crab_nbt::nbt::utils::{get_nbt_string, END_ID};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{forward_to_deserialize_any, Deserialize};

#[derive(Debug)]
pub struct Deserializer<'de> {
    input: &'de mut BytesMut,
    tag_to_deserialize: Option<u8>,
    is_named: bool,
}

impl<'de> Deserializer<'de> {
    pub fn new(input: &'de mut BytesMut, is_named: bool) -> Self {
        Deserializer {
            input,
            tag_to_deserialize: None,
            is_named,
        }
    }
}

/// Deserializes struct using Serde Deserializer from unnamed (network) NBT
pub fn from_bytes<'a, T>(s: &'a mut BytesMut) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::new(s, true);
    T::deserialize(&mut deserializer)
}

/// Deserializes struct using Serde Deserializer from normal NBT
pub fn from_bytes_unnamed<'a, T>(s: &'a mut BytesMut) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::new(s, false);
    T::deserialize(&mut deserializer)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    forward_to_deserialize_any!(i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 seq char str string bytes byte_buf tuple tuple_struct enum ignored_any unit unit_struct option newtype_struct);

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tag_to_deserialize = self.tag_to_deserialize.unwrap();

        let list_type = match tag_to_deserialize {
            LIST_ID => Some(self.input.get_u8()),
            INT_ARRAY_ID => Some(INT_ID),
            LONG_ARRAY_ID => Some(LONG_ID),
            BYTE_ARRAY_ID => Some(BYTE_ID),
            _ => None,
        };

        if let Some(list_type) = list_type {
            let remaining_values = self.input.get_u32();
            return visitor.visit_seq(ListAccess {
                de: self,
                list_type,
                remaining_values,
            });
        }

        let res: Result<V::Value> = Ok(
            match NbtTag::deserialize_data(self.input, tag_to_deserialize)? {
                NbtTag::End => {
                    unimplemented!("end")
                }
                NbtTag::Byte(value) => visitor.visit_i8(value)?,
                NbtTag::Short(value) => visitor.visit_i16(value)?,
                NbtTag::Int(value) => visitor.visit_i32(value)?,
                NbtTag::Long(value) => visitor.visit_i64(value)?,
                NbtTag::Float(value) => visitor.visit_f32(value)?,
                NbtTag::Double(value) => visitor.visit_f64(value)?,
                NbtTag::String(value) => visitor.visit_string(value)?,
                _ => unreachable!(),
            },
        );
        self.tag_to_deserialize = None;
        res
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.tag_to_deserialize.unwrap() == BYTE_ID {
            let value = self.input.get_u8();
            if value != 0 {
                return visitor.visit_bool(true);
            }
        }
        visitor.visit_bool(false)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.tag_to_deserialize.is_none() {
            let next_byte = self.input.get_u8();
            if next_byte != COMPOUND_ID {
                return Err(Error::NoRootCompound(next_byte));
            }

            if self.is_named {
                // Consume struct name
                NbtTag::deserialize(self.input)?;
            }
        }

        let value = visitor.visit_map(CompoundAccess { de: self })?;
        Ok(value)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let str = get_nbt_string(&mut self.input)?;
        visitor.visit_string(str)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct CompoundAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> MapAccess<'de> for CompoundAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let tag = self.de.input.get_u8();
        self.de.tag_to_deserialize = Some(tag);

        if tag == END_ID {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct ListAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining_values: u32,
    list_type: u8,
}

impl<'a, 'de> SeqAccess<'de> for ListAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remaining_values == 0 {
            return Ok(None);
        }

        self.remaining_values -= 1;
        self.de.tag_to_deserialize = Some(self.list_type);
        seed.deserialize(&mut *self.de).map(Some)
    }
}
