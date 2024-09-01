use serde::{Deserialize, Deserializer};

/// Crates structs, that allow for serializing fields as arrays (not lists)
/// ```ignore
/// #[serde(with = IntArray)]
/// ```
macro_rules! impl_array {
    ($name:ident, $variant:expr) => {
        pub struct $name;

        impl $name {
            pub fn serialize<T, S>(input: T, serializer: S) -> Result<S::Ok, S::Error>
            where
                T: serde::Serialize,
                S: serde::Serializer,
            {
                serializer.serialize_newtype_variant("nbt_array", 0, $variant, &input)
            }

            pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
            where
                T: Deserialize<'de>,
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer)
            }
        }
    };
}

impl_array!(IntArray, "int");
impl_array!(LongArray, "long");
impl_array!(BytesArray, "byte");
