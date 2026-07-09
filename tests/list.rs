use crab_nbt::{NbtCompound, NbtTag};

#[allow(clippy::perf)]
#[test]
fn list_types() {
    let empty: Vec<NbtTag> = Vec::new();
    let mut iter = empty.clone().into_iter();
    assert_eq!(iter.next(), None);

    let mut homogeneous = Vec::new();
    homogeneous.push(1.into());
    homogeneous.push(2.into());
    homogeneous.push(3.into());
    let mut iter = homogeneous.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&2.into()));
    assert_eq!(iter.next(), Some(&3.into()));
    assert_eq!(iter.next(), None);

    let mut serialized = NbtTag::List(homogeneous).serialize();
    let Ok(NbtTag::List(homogeneous)) = NbtTag::deserialize(&mut serialized) else {
        panic!("homogeneous list did not survive round trip");
    };

    let mut heterogeneous = homogeneous.clone();
    heterogeneous.push("four".into());
    heterogeneous.push(vec![5].into());
    heterogeneous.push(
        NbtCompound {
            child_tags: vec![("six".into(), 7.into())],
        }
        .into(),
    );
    let mut iter = heterogeneous.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&2.into()));
    assert_eq!(iter.next(), Some(&3.into()));
    assert_eq!(iter.next(), Some(&"four".into()));
    assert_eq!(iter.next(), Some(&vec![5].into()));
    assert_eq!(
        iter.next(),
        Some(
            &NbtCompound {
                child_tags: vec![("six".into(), 7.into())]
            }
            .into()
        )
    );
    assert_eq!(iter.next(), None);

    let mut serialized = NbtTag::List(heterogeneous).serialize();
    let Ok(NbtTag::List(_heterogeneous)) = NbtTag::deserialize(&mut serialized) else {
        panic!("heterogeneous list did not survive round trip");
    };
}
