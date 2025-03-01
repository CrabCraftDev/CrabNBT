#![cfg(feature = "serde")]
extern crate core;

use bytes::BytesMut;
use crab_nbt::serde::arrays::IntArray;
use crab_nbt::serde::bool::deserialize_option_bool;
use crab_nbt::serde::de::from_bytes_unnamed;
use crab_nbt::serde::ser::to_bytes_unnamed;
use crab_nbt::{nbt, Nbt, NbtCompound};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum Message {
    Request { id: String, method: i8 },
    Response { uid: i8 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Text {
    #[serde(flatten)]
    pub message: Message,
}

#[test]
fn test_map_cycle() {
    let test = Text {
        message: Message::Request {
            id: "a".to_string(),
            method: 0,
        },
    };
    let mut bytes = to_bytes_unnamed(&test).unwrap();
    let result: Text = from_bytes_unnamed(&mut bytes).unwrap();
    assert_eq!(result, test);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
pub enum Event {
    OpenUrl(Cow<'static, str>),
}

#[test]
fn test_enum_unit_variant() {
    let test = Event::OpenUrl("test".to_string().into());
    let bytes = to_bytes_unnamed(&test).unwrap();
    assert_eq!(
        bytes.as_ref(),
        b"\n\x08\0\x06action\0\x08open_url\x08\0\x05value\0\x04test\0"
    );
}
