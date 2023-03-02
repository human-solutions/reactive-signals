#[cfg(feature = "ascii-tree")]
#[test]
fn reuse_ids() {
    use insta::assert_snapshot;

    use crate::Tree;

    let mut tree = Tree::new_with_root(0);

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

    tree.reset(c2);
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 3
    "###);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [3] 3");

    let _ = tree.add_child(c1, 11);
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
