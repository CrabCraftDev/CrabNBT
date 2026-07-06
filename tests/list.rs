use crab_nbt::{NbtCompound, NbtList};

#[test]
fn list_types() {
    let empty = NbtList::new();
    let mut iter = empty.clone().into_iter();
    assert_eq!(iter.next(), None);

    let homogeneous = NbtList::new().push(1).push(2).push(3);
    let mut iter = homogeneous.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&2.into()));
    assert_eq!(iter.next(), Some(&3.into()));
    assert_eq!(iter.next(), None);

    let heterogeneous = homogeneous
        .clone()
        .push("four")
        .push(vec![5])
        .push(NbtCompound {
            child_tags: vec![("six".into(), 7.into())],
        });
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
}
