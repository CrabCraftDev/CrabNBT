#![doc = include_str!("../README.md")]

pub mod error;
mod macros;
mod nbt;
#[cfg(feature = "serde")]
pub mod serde;

pub use crab_nbt::nbt::Nbt;
pub use crab_nbt::nbt::compound::NbtCompound;
pub use crab_nbt::nbt::tag::NbtTag;

extern crate self as crab_nbt;
