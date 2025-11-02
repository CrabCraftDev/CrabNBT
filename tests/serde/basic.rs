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
    let nbt = Nbt::read_unnamed(&mut bytes).unwrap();
    assert_eq!(nbt, expected);
}

#[test]
fn test_deserialize() {
    let mut test = Vec::from(
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
