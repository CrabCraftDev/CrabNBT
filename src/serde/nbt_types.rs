use crate::{NbtCompound, NbtTag};
use serde::de::value::MapAccessDeserializer;
use serde::{Deserialize, Serialize};

impl Serialize for NbtTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NbtTag::End => serializer.serialize_unit(),
            NbtTag::Byte(byte_val) => serializer.serialize_i8(*byte_val),
            NbtTag::Short(short_val) => serializer.serialize_i16(*short_val),
            NbtTag::Int(int_val) => serializer.serialize_i32(*int_val),
            NbtTag::Long(long_val) => serializer.serialize_i64(*long_val),
            NbtTag::Float(float_val) => serializer.serialize_f32(*float_val),
            NbtTag::Double(double_val) => serializer.serialize_f64(*double_val),
            NbtTag::ByteArray(byte_array) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(byte_array.len()))?;
                for byte in byte_array.iter() {
                    seq.serialize_element(byte)?;
                }
                seq.end()
            }
            NbtTag::String(string_val) => serializer.serialize_str(string_val),
            NbtTag::List(list_items) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(list_items.len()))?;
                for item in list_items.iter() {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            NbtTag::Compound(compound) => compound.serialize(serializer),
            NbtTag::IntArray(int_array) => {
                use serde::ser::SerializeSeq;
                let mut sequence = serializer.serialize_seq(Some(int_array.len()))?;
                for int in int_array.iter() {
                    sequence.serialize_element(int)?;
                }
                sequence.end()
            }
            NbtTag::LongArray(long_array) => {
                use serde::ser::SerializeSeq;
                let mut sequence = serializer.serialize_seq(Some(long_array.len()))?;
                for long in long_array.iter() {
                    sequence.serialize_element(long)?;
                }
                sequence.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for NbtTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NbtTagVisitor;

        impl<'de> serde::de::Visitor<'de> for NbtTagVisitor {
            type Value = NbtTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an NBT tag")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(NbtTag::Byte(value as i8))
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E> {
                Ok(NbtTag::Byte(value))
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E> {
                Ok(NbtTag::Short(value))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
                Ok(NbtTag::Int(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(NbtTag::Long(value))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
                Ok(NbtTag::Float(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(NbtTag::Double(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NbtTag::String(value.to_owned()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut items = Vec::new();
                while let Some(tag) = seq.next_element()? {
                    items.push(tag);
                }
                Ok(NbtTag::List(items))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Ok(NbtTag::Compound(NbtCompound::deserialize(
                    MapAccessDeserializer::new(map),
                )?))
            }
        }

        deserializer.deserialize_any(NbtTagVisitor)
    }
}

impl Serialize for NbtCompound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.child_tags.len()))?;
        for (key, value) in &self.child_tags {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for NbtCompound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NbtCompoundVisitor;

        impl<'de> serde::de::Visitor<'de> for NbtCompoundVisitor {
            type Value = NbtCompound;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an NBT compound")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut compound = NbtCompound::new();
                while let Some((key, value)) = map.next_entry::<String, NbtTag>()? {
                    compound.put(key, value);
                }
                Ok(compound)
            }
        }

        deserializer.deserialize_map(NbtCompoundVisitor)
    }
}
