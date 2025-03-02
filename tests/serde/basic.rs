use bytes::BytesMut;
use crab_nbt::serde::arrays::IntArray;
use crab_nbt::serde::bool::deserialize_option_bool;
use crab_nbt::serde::de::from_bytes_unnamed;
use crab_nbt::serde::ser::to_bytes_unnamed;
use crab_nbt::{nbt, Nbt, NbtCompound};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test {
    str: String,
    boolean: bool,
    #[serde(with = "IntArray")]
    array: Vec<i32>,
    list: Vec<i16>,
    sub: Inner,
    sub_vec: Vec<Inner>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Inner {
    int: i32,
}

#[test]
fn test_serialize() {
    let test = Test {
        str: "hi ❤️".to_string(),
        boolean: false,
        array: vec![5, 6, 7],
        list: vec![1, 2, 3],
        sub: Inner { int: 5 },
        sub_vec: vec![Inner { int: 5 }],
    };
    let expected = nbt!("", {
        "str": "hi ❤️",
        "boolean": false,
        "array": [I; 5, 6, 7],
        "list": [1i16, 2i16, 3i16],
        "sub": {
            "int": 5
        },
        "sub_vec": [
            {
                "int": 5
            }
        ]
    });

    let mut bytes = to_bytes_unnamed(&test).unwrap();
    // not okay
    // b"\n\x08\0\x03str\0\thi \xe2\x9d\xa4\xef\xb8\x8f\x01\0\x07boolean\0\x0b\0\x05array\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\t\0\x04list\x02\0\0\0\x03\0\x01\0\x02\0\x03\n\0\x03sub\x03\0\x03int\0\0\0\x05\0\t\0\x07sub_vec\n          \x03\0\x03int\0\0\0\x05\0\0"
    // ok
    // b"\n\x08\0\x03str\0\thi \xe2\x9d\xa4\xef\xb8\x8f\x01\0\x07boolean\0\x0b\0\x05array\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\t\0\x04list\x02\0\0\0\x03\0\x01\0\x02\0\x03\n\0\x03sub\x03\0\x03int\0\0\0\x05\0\t\0\x07sub_vec\n\0\0\0\x01\x03\0\x03int\0\0\0\x05\0\0"
    // panic!("{:?}", bytes);
    let nbt = Nbt::read_unnamed(&mut bytes).unwrap();
    assert_eq!(nbt, expected);
}

#[test]
fn test_deserialize() {
    let mut test = BytesMut::from(
        &nbt!("", {
            "str": "hi ❤️",
            "boolean": false,
            "array": [I; 5, 6, 7],
            "list": [1i16, 2i16, 3i16],
            "sub": {
                "int": 5
            },
            "sub_vec": []
        })
        .write_unnamed()[..],
    );
    let expected = Test {
        str: "hi ❤️".to_string(),
        boolean: false,
        array: vec![5, 6, 7],
        list: vec![1, 2, 3],
        sub: Inner { int: 5 },
        sub_vec: Vec::new(),
    };

    let parsed: Test = from_bytes_unnamed(&mut test).unwrap();
    assert_eq!(expected, parsed);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Outer {
    #[serde(flatten)]
    pub content: InnerBool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct InnerBool {
    #[serde(default, deserialize_with = "deserialize_option_bool")]
    pub bold: Option<bool>,
}

#[test]
fn boolean_flatten() {
    let nbt = Nbt::new(
        "root".to_owned(),
        NbtCompound::from_iter([("bold".to_owned(), false.into())]),
    );

    let parsed: Outer = from_bytes_unnamed(&mut nbt.write_unnamed()).unwrap();

    assert_eq!(
        parsed,
        Outer {
            content: InnerBool { bold: Some(false) },
        }
    )
}
