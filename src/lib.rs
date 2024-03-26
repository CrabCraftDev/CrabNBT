#![doc = include_str!("../README.md")]

pub mod error;
mod macros;
mod nbt;

pub use crab_nbt::nbt::compound::NbtCompound;
pub use crab_nbt::nbt::tag::NbtTag;
pub use crab_nbt::nbt::Nbt;

extern crate self as crab_nbt;
