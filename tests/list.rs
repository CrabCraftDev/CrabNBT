use crab_nbt::{NbtCompound, NbtList};

#[test]
fn list_types() {
    let empty = NbtList::new();
    let mut iter = empty.clone().into_iter();
    assert_eq!(iter.next(), None);

    let homogeneous = NbtList::new().push(1).push(2).push(3);

    assert_eq!(
        homogeneous,
        NbtList::Homogeneous(vec![1.into(), 2.into(), 3.into()])
    );

    let mut iter = homogeneous.clone().into_iter();
    assert_eq!(iter.next(), Some(1.into()));
    assert_eq!(iter.next(), Some(2.into()));
    assert_eq!(iter.next(), Some(3.into()));
    assert_eq!(iter.next(), None);

    let heterogeneous = homogeneous
        .clone()
        .push("four")
        .push(vec![5])
        .push(NbtCompound {
            child_tags: vec![("six".into(), 7.into())],
        });

    assert_eq!(
        heterogeneous,
        NbtList::Heterogeneous(vec![
            NbtCompound::wrap(1),
            NbtCompound::wrap(2),
            NbtCompound::wrap(3),
            NbtCompound::wrap("four"),
            NbtCompound::wrap(vec![5]),
            NbtCompound {
                child_tags: vec![("six".into(), 7.into())]
            }
        ])
    );

    let mut iter = heterogeneous.clone().into_iter();
    assert_eq!(iter.next(), Some(1.into()));
    assert_eq!(iter.next(), Some(2.into()));
    assert_eq!(iter.next(), Some(3.into()));
    assert_eq!(iter.next(), Some("four".into()));
    assert_eq!(iter.next(), Some(vec![5].into()));
    assert_eq!(
        iter.next(),
        Some(
            NbtCompound {
                child_tags: vec![("six".into(), 7.into())]
            }
            .into()
        )
    );
    assert_eq!(iter.next(), None);
}
