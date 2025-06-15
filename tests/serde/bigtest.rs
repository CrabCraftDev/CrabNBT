use crate::serde::test_data_definitions::BigTest;
use bytes::BytesMut;
use crab_nbt::serde::{
    de::{from_bytes, from_bytes_unnamed},
    ser::{to_bytes, to_bytes_unnamed},
};
use std::mem;

#[test]
fn test_roundtrip_bigtest() {
    let bytes = BytesMut::from(include_bytes!("../data/bigtest.nbt") as &[u8]);
    let deserialized = from_bytes::<BigTest>(&mut bytes.clone()).unwrap();
    let bytes2 = to_bytes(&deserialized, "Level".to_string()).unwrap();
    let deserialized2 = from_bytes::<BigTest>(&mut bytes2.clone()).unwrap();

    assert_eq!(deserialized, deserialized2);
    assert_eq!(bytes.len(), bytes2.len());
}

#[test]
fn test_roundtrip_bigtest_unnamed() {
    let bytes = BytesMut::from(include_bytes!("../data/bigtest.nbt") as &[u8]);
    let deserialized = from_bytes::<BigTest>(&mut bytes.clone()).unwrap();
    let bytes2 = to_bytes_unnamed(&deserialized).unwrap();
    let deserialized2 = from_bytes_unnamed::<BigTest>(&mut bytes2.clone()).unwrap();

    assert_eq!(deserialized, deserialized2);
    assert_eq!(
        bytes.len() - mem::size_of::<u16>() - "Level".len(),
        bytes2.len()
    );
}
