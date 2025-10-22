use bytes::Bytes;
use crab_nbt::{/*nbt::tag::escape_string_value,*/ NbtTag, nbt};


#[test]
fn compound_with_basic_numbers() {
    let test_string = "{\"\": {a: 0b, b: 1s, c: 2, d: 3L}}";
    let nbt = nbt!("", {"a": 0i8, "b": 1i16, "c": 2i32, "d": 3i64});
    assert_eq!(nbt.to_string(), test_string)
}

#[test]
fn byte_array() {
    let test_string = "[B; 45B, -37B, 111B, -90B]";
    let nbt = NbtTag::ByteArray(Bytes::from(vec![45, 219, 111, 166]));
    assert_eq!(nbt.to_string(), test_string)
}

#[test]
fn int_array() {
    let test_string = "[I; 1906, -165, -1073741824]";
    let nbt = NbtTag::IntArray(vec![1906, -165, -1073741824]);
    assert_eq!(nbt.to_string(), test_string)
}

#[test]
fn long_array() {
    let test_string = "[L; 1906L, -165L, -1073741824L, 576460752303423488L]";
    let nbt = NbtTag::LongArray(vec![1906, -165, -1073741824, 576460752303423488]);
    assert_eq!(nbt.to_string(), test_string)
}

#[test]
fn complex_compound() {
    let test_string = "{\"\": {components: {\"minecraft:unbreakable\": {}, \"minecraft:custom_name\": '\"Excalibur\"', \"minecraft:rarity\": \"rare\"}, count: 1, id: \"minecraft:iron_sword\"}}";
    let nbt = nbt!("", {
        "components": {
            "minecraft:unbreakable": {},
            "minecraft:custom_name": "\"Excalibur\"",
            "minecraft:rarity": "rare"
        },
        "count": 1,
        "id": "minecraft:iron_sword"
    });
    assert_eq!(nbt.to_string(), test_string)
}

// #[test]
// fn string_escape() {
//     assert_eq!(escape_string_value("minecraft:grass_block"), "\"minecraft:grass_block\"");
//     assert_eq!(escape_string_value("\"I am a JSON string!\""), "\'\"I am a JSON string!\"\'");
//     assert_eq!(escape_string_value("I am very normal"), "\"I am very normal\"");
//     assert_eq!(escape_string_value("Idontevenhavespaces"), "\"Idontevenhavespaces\"");
// }