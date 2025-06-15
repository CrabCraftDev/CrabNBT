use crate::{serde::test_data_definitions::ComplexPlayer, util::decompress_data};
use bytes::BytesMut;
use crab_nbt::serde::{
    de::{from_bytes, from_bytes_unnamed},
    ser::{to_bytes, to_bytes_unnamed},
};

#[test]
fn test_roundtrip_complex() {
    let bytes = BytesMut::from_iter(decompress_data(
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
    let bytes = BytesMut::from_iter(decompress_data(
        include_bytes!("../data/complex_player.dat") as &[u8],
    ));
    let deserialized = from_bytes::<ComplexPlayer>(&mut bytes.clone()).unwrap();
    let bytes2 = to_bytes_unnamed(&deserialized).unwrap();
    let deserialized2 = from_bytes_unnamed::<ComplexPlayer>(&mut bytes2.clone()).unwrap();

    assert_eq!(deserialized, deserialized2);
    const NAME_LEN: usize = 2; // u32 + no name string>
    assert_eq!(bytes.len() - NAME_LEN, bytes2.len());
}
