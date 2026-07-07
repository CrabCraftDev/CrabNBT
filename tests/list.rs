use crab_nbt::{Nbt, NbtCompound, NbtList, NbtTag};

#[test]
fn list_types() {
    let empty = NbtList::new();
    let mut iter = empty.clone().into_iter();
    assert_eq!(iter.next(), None);

    let mut homogeneous = NbtList::new();
    homogeneous.push(1);
    homogeneous.push(2);
    homogeneous.push(3);
    assert!(homogeneous.is_homogeneous());
    let mut iter = homogeneous.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&2.into()));
    assert_eq!(iter.next(), Some(&3.into()));
    assert_eq!(iter.next(), None);

    let mut serialized = NbtTag::List(homogeneous).serialize();
    let NbtTag::List(homogeneous) = NbtTag::deserialize(&mut serialized).unwrap() else {
        panic!("homogeneous list did not survive round trip");
    };
    assert!(homogeneous.is_homogeneous());

    let mut heterogeneous = homogeneous.clone();
    heterogeneous.push("four");
    heterogeneous.push(vec![5]);
    heterogeneous.push(NbtCompound {
        child_tags: vec![("six".into(), 7.into())],
    });
    assert!(heterogeneous.is_heterogeneous());
    let mut iter = heterogeneous.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1.into()));
    assert_eq!(iter.next(), Some(&mut 2.into()));
    assert_eq!(iter.next(), Some(&mut 3.into()));
    assert_eq!(iter.next(), Some(&mut "four".into()));
    assert_eq!(iter.next(), Some(&mut vec![5].into()));
    assert_eq!(
        iter.next(),
        Some(
            &mut NbtCompound {
                child_tags: vec![("six".into(), 7.into())]
            }
            .into()
        )
    );
    assert_eq!(iter.next(), None);

    let mut serialized = NbtTag::List(heterogeneous).serialize();
    eprintln!("serialized: {serialized:?}");
    let NbtTag::List(heterogeneous) = NbtTag::deserialize(&mut serialized).unwrap() else {
        panic!("heterogeneous list did not survive round trip");
    };
    assert!(heterogeneous.is_heterogeneous());
}
