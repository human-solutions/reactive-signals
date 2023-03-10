use crate::Tree;

const ASCII_REF: &str = r###"0
 ├─ 1
 ├─ 2
 │   ├─ 20
 │   ├─ 21
 │   └─ 22
 └─ 3
"###;

const TREE_NODES: &str = "[0] 0, [1] 1, [2] 2, [3] 3, [4] 20, [5] 21, [6] 22";
const EMPTY_NODES: &str = "";

#[test]
fn reuse_ids() {
    let mut tree = Tree::create();
    for _ in 0..3 {
        setup_tree(&mut tree);
        assert_eq!(tree.dump_used(), TREE_NODES);
        assert_eq!(tree.ascii(&|d| d.to_string()), ASCII_REF);

        tree.discard_all(|_| {});
        assert_eq!(tree.dump_used(), EMPTY_NODES);
    }
}

fn setup_tree(tree: &mut Tree<usize>) {
    tree.init(0);
    let _ = tree.add_child(tree.root(), 1);
    let c2 = tree.add_child(tree.root(), 2);
    let _ = tree.add_child(tree.root(), 3);

    let _ = tree.add_child(c2, 20);
    let _ = tree.add_child(c2, 21);
    let _ = tree.add_child(c2, 22);
}
