use crate::Tree;
use insta::assert_snapshot;

use super::StringStore;

fn create_tree() -> Tree<usize> {
    let mut tree = Tree::new_with_root(0);

    let c1 = tree.add_child(tree.root(), 1);
    let _c1_1 = tree.add_child(c1, 11);

    let c2 = tree.add_child(tree.root(), 2);
    let _c2_0 = tree.add_child(c2, 20);
    let _c2_1 = tree.add_child(c2, 21);
    let c2_2 = tree.add_child(c2, 22);
    let _c2_2_0 = tree.add_child(c2_2, 220);
    let _c2_2_1 = tree.add_child(c2_2, 221);
    let _c3 = tree.add_child(tree.root(), 3);
    let _c4 = tree.add_child(tree.root(), 4);

    tree
}

#[test]
fn show_tree() {
    let tree = create_tree();
    assert_snapshot!(tree.dump_used(), @"[0] 0, [1] 1, [2] 11, [3] 2, [4] 20, [5] 21, [6] 22, [7] 220, [8] 221, [9] 3, [10] 4");

    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     │   └─ 11
     ├─ 2
     │   ├─ 20
     │   ├─ 21
     │   └─ 22
     │       ├─ 220
     │       └─ 221
     ├─ 3
     └─ 4
    "###);
}

const ITER_ORDER: &str = "4, 3, 221, 220, 22, 21, 20, 2, 11, 0";

#[test]
fn iter() {
    let tree = create_tree();
    let mut visits = vec![];
    for node in tree.iter_from(tree.root()) {
        visits.push(format!("{}", tree.nodes[node.index()].data));
    }

    assert_eq!(visits.join(", "), ITER_ORDER);
}

#[test]
fn iter_single_node() {
    let mut tree = Tree::new_with_root(0);

    let c1 = tree.add_child(tree.root(), 1);
    let mut visits = vec![];
    for node in tree.iter_from(c1) {
        visits.push(format!("{}", tree.nodes[node.index()].data));
    }

    assert_eq!(visits.join(", "), "1");
}

#[test]
fn iter_single_with_sibling() {
    let mut tree = Tree::new_with_root(0);

    let c1 = tree.add_child(tree.root(), 1);
    let _ = tree.add_child(tree.root(), 2);

    assert_snapshot!(tree.ascii(&|d| d.to_string()), @r###"
    0
     ├─ 1
     └─ 2
    "###);
    let mut visits = vec![];
    for node in tree.iter_from(c1) {
        visits.push(format!("{}", tree.nodes[node.index()].data));
    }

    assert_eq!(visits.join(", "), "1");
}

#[test]
fn mut_iter() {
    let mut tree = create_tree();

    let visits = StringStore::new();
    tree.iter_mut_from(tree.root()).for_each(|tree, node| {
        visits.push(format!("{}", tree.nodes[node.index()].data));
        // just to make sure we can write to it
        tree.nodes[node.index()].data += 1;
    });

    assert_eq!(visits.values(), ITER_ORDER);
}
