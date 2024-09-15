#![cfg(feature = "serde")]
use bytes::BytesMut;
use crab_nbt::serde::arrays::IntArray;
use crab_nbt::serde::de::from_bytes_unnamed;
use crab_nbt::serde::ser::to_bytes_unnamed;
use crab_nbt::{nbt, Nbt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test {
    str: String,
    boolean: bool,
    #[serde(with = "IntArray")]
    array: Vec<i32>,
    list: Vec<i16>,
    sub: Inner,
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
    };
    let expected = nbt!("", {
        "str": "hi ❤️",
        "boolean": false,
        "array": [I; 5, 6, 7],
        "list": [1i16, 2i16, 3i16],
        "sub": {
            "int": 5
        }
    });

    let mut bytes = to_bytes_unnamed(&test).unwrap();
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
            }
        })
        .write_unnamed()[..],
    );
    let expected = Test {
        str: "hi ❤️".to_string(),
        boolean: false,
        array: vec![5, 6, 7],
        list: vec![1, 2, 3],
        sub: Inner { int: 5 },
    };

    let parsed: Test = from_bytes_unnamed(&mut test).unwrap();
    assert_eq!(expected, parsed);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum Message {
    Request { id: String, method: i8 },
    Response { uid: i8 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Text {
    #[serde(flatten)]
    pub message: Message
}


#[test]
fn test_map_cycle() {
    let test = Text {
        message: Message::Request {
            id: "a".to_string(),
            method: 0
        }
    };
    let mut bytes = to_bytes_unnamed(&test).unwrap();
    let result: Text = from_bytes_unnamed(&mut bytes).unwrap();
    assert_eq!(result, test);
}
