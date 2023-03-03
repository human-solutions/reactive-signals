use insta::assert_snapshot;

#[test]
fn deep() {
    use crate::Tree;

    let mut tree = Tree::new_with_root(0);

    let mut deeper = tree.add_child(tree.root(), 1);
    let start = deeper;
    for i in 2..10 {
        deeper = tree.add_child(deeper, i);
    }

    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     └─ 1
         └─ 2
             └─ 3
                 └─ 4
                     └─ 5
                         └─ 6
                             └─ 7
                                 └─ 8
                                     └─ 9
    "###);
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 2, [3] 3, [4] 4, [5] 5, [6] 6, [7] 7, [8] 8, [9] 9");

    tree.reset(start);

    assert_eq!(tree.ascii(&|d| d.to_string()), " 0\n");
    assert_snapshot!(tree.dump_used(), @"[0] 0");
}
