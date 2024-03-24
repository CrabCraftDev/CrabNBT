use bytes::Bytes;
use crab_nbt::{nbt, Nbt, NbtTag};

#[test]
fn serialize_raw_string() {
    let serialized = NbtTag::String("How are you?".to_string()).serialize_raw();
    assert_eq!(serialized.to_vec(), b"\0\x0cHow are you?")
}

#[test]
fn serialize_raw() {
    let serialized = NbtTag::Long(2137).serialize_raw();
    assert_eq!(serialized.to_vec(), 2137_i64.to_be_bytes().to_vec())
}

#[test]
fn serialize_named() {
    let serialized = NbtTag::Long(2137).serialize_named("hi");
    assert_eq!(serialized.to_vec(), b"\x04\0\x02hi\0\0\0\0\0\0\x08\x59")
}

#[test]
fn deserialize_bigtest() {
    let bytes = Bytes::from(include_bytes!("data/bigtest.nbt") as &[u8]);
    let nbt = Nbt::read(&mut bytes.clone(), false).unwrap();
    let egg_name = nbt
        .get_compound("nested compound test")
        .and_then(|compound| compound.get_compound("egg"))
        .and_then(|compound| compound.get_string("name"))
        .unwrap();

    assert_eq!(egg_name, "Eggbert");
}

#[test]
fn network_nbt() {
    let expected_nbt = nbt!("", {
        "int": 1_i32,
        "nested": {
            "key": "value"
        }
    });

    let bytes = expected_nbt.write(true);

    let nbt = Nbt::read(&mut bytes.clone(), true).unwrap();

    assert_eq!(nbt, expected_nbt);
}

#[test]
fn correct_end_tags() {
    let heightmap = nbt!("", {
        "WORLD_SURFACE": [L;],
    });

    let expected: &[u8] = b"\n\x0c\0\rWORLD_SURFACE\0\0\0\0\0";
    assert_eq!(heightmap.write(true).as_ref(), expected)
}