# 🦀 CrabNBT

Up-to-date Rust crate for easy and intuitive working with NBT data.

## Why not other libraries?

CrabNBT combines best features of existing NBT crates, to create perfect, easy
to use solution.<br> Big thanks to
[simdnbt](https://github.com/azalea-rs/simdnbt) and
[fastnbt](https://github.com/owengage/fastnbt) for ideas!

## Features

✅ Support for serializing to/from Struct (serde)<br> ✅
[Java string](https://docs.oracle.com/javase/8/docs/api/java/io/DataInput.html#modified-utf-8)
support <br> ✅ `nbt!` macro for easy creation <br> ✅ Easy to use system of
retrieving values from NBT <br> ✅ Serialization support for individual tags
<br> ✅ Support for
[Network NBT](https://wiki.vg/NBT#Network_NBT_(Java_Edition))

## Installing

```shell
cargo add crab_nbt
```

## Serializing

```rust
use crab_nbt::{nbt, Nbt, NbtCompound};

// Using NBT macro
let nbt = nbt!("root nbt_inner name", {
    "float": 1.0,
    "key": "value",
    "long_array": [L; 1, 2],
    "int_array": [Int; 1, 10, 25],
    "byte_array": [B; 0, 1, 0, 0, 1],
    "list": ["a", "b", "c"],
    "nbt_inner": {
        "key": "sub value"
    }
});

let nbt = Nbt::new(
    "root".to_owned(),
    NbtCompound::from_iter([
        ("float".to_owned(), 1.0.into()),
        ("key".to_owned(), "value".into()),
        ("nbt_inner".to_owned(), NbtCompound::from_iter([
            ("key".to_owned(), "sub value".into()),
        ]).into())
    ])
);

let network_bytes = nbt.write_unnamed();
let normal_bytes = nbt.write();
```

## Deserializing

```rust
use crab_nbt::{nbt, Nbt, NbtCompound};

fn example(bytes: &[u8]) {
    let nbt = Nbt::read(bytes).unwrap();
    let egg_name = nbt
        .get_compound("nbt_inner")
        .and_then(|compound| compound.get_compound("egg"))
        .and_then(|compound| compound.get_string("name"))
        .unwrap();
}
```

## Serde

_Requires `serde` feature._

```rust ignore
use crab_nbt::serde::{arrays::IntArray, ser::to_bytes_unnamed, de::from_bytes_unnamed};
use crab_nbt::serde::bool::deserialize_bool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test {
    number: i32,
    #[serde(with = "IntArray")]
    int_array: Vec<i32>,
    /// Using [deserialize_bool] is required only if
    /// you are using `#[serde(flatten)]` attribute 
    #[serde(deserialize_with = "deserialize_bool")]
    bool: bool
}

fn cycle() {
    let test = Test {
        number: 5,
        int_array: vec![7, 8],
        bool: false
    };
    let mut bytes = to_bytes_unnamed(&test).unwrap();
    let recreated_struct: Test = from_bytes_unnamed(&mut bytes).unwrap();
    
    assert_eq!(test, recreated_struct);
}
```

## Feature flags
- `serde` - Serde integration.
- `macro` - The `nbt!` macro for easy creation of compounds with json like syntax.
- `nightly` - Additional performance optimizations that require the nightly Rust toolchain.
