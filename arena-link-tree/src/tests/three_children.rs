use crate::Tree;

#[cfg(feature = "ascii-tree")]
#[test]
fn three_children() {
    use insta::assert_snapshot;

    let mut tree = Tree::new_with_root(0);

    let c1 = tree.add_child(tree.root(), 1);
    let c2 = tree.add_child(tree.root(), 2);
    let c3 = tree.add_child(tree.root(), 3);

    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     ├─ 2
     └─ 3
    "###);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 2, [3] 3");

    let orig_tree = tree.clone();

    tree.reset(c1);
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 2
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.reset(c2);
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.reset(c3);
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 2
    "###);

    tree = orig_tree.clone();
    tree.reset(c1);
    tree.reset(c2);
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.reset(c3);
    tree.reset(c2);
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     └─ 1
    "###);

    tree = orig_tree.clone();
    tree.reset(c3);
    tree.reset(c2);
    tree.reset(c1);
    assert_eq!(tree.ascii(&|d| d.to_string()), " 0\n");
    assert_snapshot!(tree.dump_used(), @"[0] 0");

    tree = orig_tree.clone();
    tree.reset(c1);
    tree.reset(c2);
    tree.reset(c3);
    assert_eq!(tree.ascii(&|d| d.to_string()), " 0\n");
}
