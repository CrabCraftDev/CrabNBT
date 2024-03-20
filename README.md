# 🦀 CrabNBT
Up-to-date Rust crate for easy and intuitive working with NBT data.

## Why not other libraries?
CrabNBT combines best features of existing NBT crates, to create perfect solution.<br>
Big thanks to [simdnbt](https://github.com/azalea-rs/simdnbt) and [fastnbt](https://github.com/owengage/fastnbt) for ideas!

## Features
🚧 Support for serializing to/from Struct *(soon)*<br>
✅ [Java string](https://docs.oracle.com/javase/8/docs/api/java/io/DataInput.html#modified-utf-8) support <br>
✅ NBT! macro for easy creation <br>
✅ Good system of getting values from NBT <br>
✅ Serializing for single tags <br>
✅ Support of [Network NBT](https://wiki.vg/NBT#Network_NBT_(Java_Edition))

## Installing
```shell
cargo add crab_nbt
```

## Serializing
```rust
use crab_nbt::{nbt, Nbt, NbtCompound};

fn main() {
    // Using NBT macro
    let nbt = nbt!("root nbt_inner name", {
        "float": 1.0,
        "key": "value",
        "long_array": [L; 1],
        "int_array": [Int; 1],
        "list": ["a", "b", "c"],
        "nbt_inner": {
            "key": "sub value"
        }
    });

    let nbt = Nbt::new(
        "root",
        NbtCompound::from_values(vec![
            ("float".into(), 1.0.into()),
            ("key".into(), "value".to_string().into()),
            ("nbt_inner".into(), NbtCompound::from_values(vec![
                ("key".into(), "sub value".into()),
            ]).into())
        ])
    );
}
```

## Deserializing

```rust
use bytes::Bytes;
use crab_nbt::{nbt, Nbt, NbtCompound};

fn example(bytes: &mut Bytes) {
    let nbt = Nbt::read(bytes, false).unwrap();
    let egg_name = nbt
        .get_compound("nbt_inner")
        .and_then(|compound| compound.get_compound("egg"))
        .and_then(|compound| compound.get_string("name"))
        .unwrap();
}
```
