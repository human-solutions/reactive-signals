use super::Tree;
use insta::assert_snapshot;

#[test]
fn three_children() {
    let mut tree = Tree::create_and_init(0);

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

    tree.discard(c1, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 2
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.discard(c2, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.discard(c3, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 2
    "###);

    tree = orig_tree.clone();
    tree.discard(c1, |_| {});
    tree.discard(c2, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.discard(c3, |_| {});
    tree.discard(c2, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     └─ 1
    "###);

    tree = orig_tree.clone();
    tree.discard(c3, |_| {});
    tree.discard(c2, |_| {});
    tree.discard(c1, |_| {});
    assert_eq!(tree.ascii(&|d| d.to_string()), " 0\n");
    assert_snapshot!(tree.dump_used(), @"[0] 0");

    tree = orig_tree.clone();
    tree.discard(c1, |_| {});
    tree.discard(c2, |_| {});
    tree.discard(c3, |_| {});
    assert_eq!(tree.ascii(&|d| d.to_string()), " 0\n");
}
