use std::assert_matches;

#[test]
pub fn adversarial() {
    let bytes = include_bytes!("./data/adversarial.dat").to_vec();
    let nbt = crab_nbt::NbtTag::deserialize(&mut bytes.as_slice());
    assert_matches!(nbt, Ok(_));
    let nbt = nbt.unwrap();
    let mut reserialized = nbt.serialize();

    let nbt2 = crab_nbt::NbtTag::deserialize(&mut reserialized);
    assert_matches!(nbt2, Ok(_));
    let nbt2 = nbt2.unwrap();
    assert_eq!(nbt, nbt2);
}
