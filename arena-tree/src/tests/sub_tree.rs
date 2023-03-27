use insta::assert_snapshot;

#[test]
fn sub_tree() {
    use crate::Tree;

    let mut tree = Tree::create_and_init(0);

    let c1 = tree.add_child(tree.root(), 1);
    let c2 = tree.add_child(tree.root(), 2);
    let c3 = tree.add_child(tree.root(), 3);

    let c2_0 = tree.add_child(c2, 20);
    let c2_1 = tree.add_child(c2, 21);
    let c2_2 = tree.add_child(c2, 22);

    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 2, [3] 3, [4] 20, [5] 21, [6] 22");

    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     ├─ 2
     │   ├─ 20
     │   ├─ 21
     │   └─ 22
     └─ 3
    "###);

    let orig_tree = tree.clone();

    tree.discard(c1, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 2
     │   ├─ 20
     │   ├─ 21
     │   └─ 22
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.discard(c2, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 3
    "###);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [3] 3");

    tree = orig_tree.clone();
    tree.discard(c3, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 2
         ├─ 20
         ├─ 21
         └─ 22
    "###);

    tree = orig_tree.clone();
    tree.discard(c2_0, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     ├─ 2
     │   ├─ 21
     │   └─ 22
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.discard(c2_1, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     ├─ 2
     │   ├─ 20
     │   └─ 22
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.discard(c2_2, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     ├─ 2
     │   ├─ 20
     │   └─ 21
     └─ 3
    "###);

    tree = orig_tree.clone();
    tree.discard(c2_0, |_| {});
    tree.discard(c2_1, |_| {});
    tree.discard(c2_2, |_| {});
    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     ├─ 2
     └─ 3
    "###);

    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 2, [3] 3");
}
