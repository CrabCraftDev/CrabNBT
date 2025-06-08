use crate::serde::test_data_definitions::BigTest;
use bytes::BytesMut;
use crab_nbt::serde::de::{from_bytes, from_bytes_unnamed};
use crab_nbt::serde::ser::to_bytes_unnamed;

#[test]
fn test_roundtrip_bigtest() {
    let mut bytes = BytesMut::from(include_bytes!("../data/bigtest.nbt") as &[u8]);
    let deserialized = from_bytes::<BigTest>(&mut bytes).unwrap();
    let mut bytes = to_bytes_unnamed(&deserialized).unwrap();
    let deserialized2 = from_bytes_unnamed::<BigTest>(&mut bytes).unwrap();

    assert_eq!(deserialized, deserialized2);
}
