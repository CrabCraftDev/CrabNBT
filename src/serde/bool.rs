use core::fmt;
use serde::Deserializer;
use serde::de::{Error, Visitor};

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BoolVisitor)
}

pub fn deserialize_option_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BoolVisitor).map(Some)
}

pub struct BoolVisitor;

impl Visitor<'_> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("bool or i8")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(value == 1)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(value)
    }
}
