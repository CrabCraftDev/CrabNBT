use crate::{
    decompress::decompress_data,
    serde::test_data_definitions::{BigTest, ComplexPlayer},
};
use crab_nbt::serde::{
    de::{from_bytes, from_bytes_unnamed},
    ser::{to_bytes, to_bytes_unnamed},
};

#[test]
fn test_roundtrip_bigtest() {
    let bytes = Vec::from(include_bytes!("../data/bigtest.nbt") as &[u8]);
    let deserialized = from_bytes::<BigTest>(&mut bytes.clone()).unwrap();
    let bytes2 = to_bytes(&deserialized, "Level".to_string()).unwrap();
    let deserialized2 = from_bytes::<BigTest>(&mut bytes2.clone()).unwrap();

    assert_eq!(deserialized, deserialized2);
    assert_eq!(bytes.len(), bytes2.len());
}

#[test]
fn test_roundtrip_bigtest_unnamed() {
    let bytes = Vec::from(include_bytes!("../data/bigtest.nbt") as &[u8]);
    let deserialized = from_bytes::<BigTest>(&mut bytes.clone()).unwrap();
    let bytes2 = to_bytes_unnamed(&deserialized).unwrap();
    let deserialized2 = from_bytes_unnamed::<BigTest>(&mut bytes2.clone()).unwrap();

    assert_eq!(deserialized, deserialized2);
    const NAME_LEN: usize = 2 + 5; // u32 + "Level"
    assert_eq!(bytes.len() - NAME_LEN, bytes2.len());
}

#[test]
fn test_roundtrip_complex() {
    let bytes = Vec::from_iter(decompress_data(
        include_bytes!("../data/complex_player.dat") as &[u8],
    ));
    let deserialized = from_bytes::<ComplexPlayer>(&mut bytes.clone()).unwrap();
    let bytes2 = to_bytes(&deserialized, "".to_owned()).unwrap();
    let deserialized2 = from_bytes::<ComplexPlayer>(&mut bytes2.clone()).unwrap();

    assert_eq!(deserialized, deserialized2);
    assert_eq!(bytes.len(), bytes2.len());
}

#[test]
fn test_roundtrip_complex_unnamed() {
    let bytes = Vec::from_iter(decompress_data(
        include_bytes!("../data/complex_player.dat") as &[u8],
    ));
    let deserialized = from_bytes::<ComplexPlayer>(&mut bytes.clone()).unwrap();
    let bytes2 = to_bytes_unnamed(&deserialized).unwrap();
    let deserialized2 = from_bytes_unnamed::<ComplexPlayer>(&mut bytes2.clone()).unwrap();

    assert_eq!(deserialized, deserialized2);
    const NAME_LEN: usize = 2; // u32 + no name string>
    assert_eq!(bytes.len() - NAME_LEN, bytes2.len());
}
