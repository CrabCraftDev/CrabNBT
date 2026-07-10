#![doc = include_str!("../README.md")]

pub mod error;
mod macros;
mod nbt;
mod utils;
#[cfg(feature = "serde")]
pub mod serde;

pub use crab_nbt::nbt::compound::NbtCompound;
pub use crab_nbt::nbt::tag::NbtTag;
pub use crab_nbt::nbt::Nbt;
pub use crab_nbt::utils::{TryAsRef, TryAsMut};

extern crate self as crab_nbt;
