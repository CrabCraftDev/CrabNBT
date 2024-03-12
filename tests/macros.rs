use crab_nbt::{nbt, Nbt, NbtCompound, NbtTag};

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
    let nbt_expected = Nbt::new(
        "root",
        NbtCompound::from_values(vec![
            ("float".into(), 1.0_f32.into()),
            ("key".into(), "value".into()),
            ("long_array".into(), NbtTag::LongArray(vec![1])),
            ("int_array".into(), NbtTag::IntArray(vec![1])),
            (
                "list".into(),
                NbtTag::List(vec!["a".into(), "b".into(), "c".into()]),
            ),
            (
                "nbt_inner".into(),
                NbtCompound::from_values(vec![("key".into(), "sub value".into())]).into(),
            ),
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
        }
    });

    assert_eq!(nbt.child_tags, nbt_expected.child_tags);
}
