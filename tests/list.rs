use crab_nbt::{NbtCompound, NbtList};

#[test]
fn list_types() {
    let homogeneous = NbtList::new().push(1).push(2).push(3);

    assert_eq!(
        homogeneous,
        NbtList::Homogeneous((3, vec![1.into(), 2.into(), 3.into()]))
    );

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
}
