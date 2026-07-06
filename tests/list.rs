use crab_nbt::{NbtCompound, NbtList};

#[test]
fn list_types() {
    let empty = NbtList::new();
    let mut iter = empty.clone().into_iter();
    assert_eq!(iter.next(), None);

    let mut homogeneous = NbtList::new();
    homogeneous.push(1);
    homogeneous.push(2);
    homogeneous.push(3);
    let mut iter = homogeneous.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&2.into()));
    assert_eq!(iter.next(), Some(&3.into()));
    assert_eq!(iter.next(), None);

    let mut heterogeneous = homogeneous.clone();
    heterogeneous.push("four");
    heterogeneous.push(vec![5]);
    heterogeneous.push(NbtCompound {
        child_tags: vec![("six".into(), 7.into())],
    });
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
}
