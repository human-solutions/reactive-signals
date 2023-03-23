use insta::assert_snapshot;

#[test]
fn reuse_ids() {
    use crate::Tree;

    let mut tree = Tree::create_and_init(0);

    let c1 = tree.add_child(tree.root(), 1);
    let c2 = tree.add_child(tree.root(), 2);
    let c3 = tree.add_child(tree.root(), 3);

    let _ = tree.add_child(c2, 20);
    let _ = tree.add_child(c2, 21);
    let _ = tree.add_child(c2, 22);

    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     ├─ 2
     │   ├─ 20
     │   ├─ 21
     │   └─ 22
     └─ 3
    "###);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 2, [3] 3, [4] 20, [5] 21, [6] 22");

    // assert_eq!(tree.nodes[0].is_used(), true);
    tree.discard(c2, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 3
    "###);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [3] 3");
    assert_eq!(
        tree.availability.0.arr[0],
        0b1000_0000_0000_0000_0000_0000_0000_0000
    );
    let _ = tree.add_child(c1, 11);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 11, [3] 3");

    assert_eq!(
        tree.availability.0.arr[0],
        0b1000_0000_0000_0000_0000_0000_0000_0000
    );
    let _ = tree.add_child(c3, 31);

    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     │   └─ 11
     └─ 3
         └─ 31
    "###);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 11, [3] 3, [4] 31");
}
