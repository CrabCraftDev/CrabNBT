use bytes::Bytes;
use crab_nbt::{Nbt, NbtCompound, NbtTag, nbt};

#[test]
fn nbt_macro_handles_empty_content() {
    let nbt = nbt!("root", {});
    assert_eq!(nbt.child_tags.len(), 0);
}

#[test]
fn nbt_macro_handles_nested_content() {
    let nbt = nbt!("root", {
        "nested": {
            "key": "value"
        }
    });

    let nested = nbt.get_compound("nested").unwrap();
    let value = nested.get_string("key").unwrap();

    assert_eq!(value, "value");
}

#[test]
fn nbt_macro_handles_array_content() {
    let nbt = nbt!("root", {
        "array": [1, 2, 3]
    });

    let array = nbt.get_list("array").unwrap();

    assert_eq!(array, &vec![1.into(), 2.into(), 3.into()]);
}

#[test]
fn nbt_macro_panics_on_nonexistent_key() {
    let nbt = nbt!("root", {
        "key": "value"
    });

    assert_eq!(nbt.get_string("nonexistent"), None);
}

#[test]
fn nbt_macro_complex_object() {
    let key = "a_key".to_owned();
    let some_bytes = Bytes::from_iter([0, 1, 2, 3]);

    let nbt_expected = Nbt::new(
        "root".to_owned(),
        NbtCompound::from_iter([
            ("float".to_owned(), 1.0_f32.into()),
            ("key".to_owned(), "value".into()),
            ("long_array".to_owned(), NbtTag::LongArray(vec![1])),
            ("int_array".to_owned(), NbtTag::IntArray(vec![1])),
            (
                "list".to_owned(),
                NbtTag::List(vec!["a".into(), "b".into(), "c".into()]),
            ),
            (
                "nbt_inner".to_owned(),
                NbtCompound::from_iter([("key".to_owned(), "sub value".into())]).into(),
            ),
            (key.clone(), some_bytes.clone().into()),
        ]),
    );

    let nbt = nbt!("root", {
        "float": 1.0_f32,
        "key": "value",
        "long_array": [L; 1],
        "int_array": [I; 1],
        "list": ["a", "b", "c"],
        "nbt_inner": {
            "key": "sub value"
        },
        key: some_bytes
    });

    assert_eq!(nbt.child_tags, nbt_expected.child_tags);
}
