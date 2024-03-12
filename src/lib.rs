#![doc = include_str!("../README.md")]

pub mod error;
mod macros;
mod nbt;

pub use crab_nbt::nbt::compound::NbtCompound;
pub use crab_nbt::nbt::root_nbt::Nbt;
pub use crab_nbt::nbt::tag::NbtTag;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

extern crate self as crab_nbt;
