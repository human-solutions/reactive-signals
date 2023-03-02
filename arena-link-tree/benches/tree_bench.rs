use arena_link_tree::{Node, Tree};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn create_tree_with_1000_nodes(c: &mut Criterion) {
    c.bench_function("Create tree with 1,000 nodes wide", |b| {
        b.iter(|| {
            let mut tree = Tree::new_with_root(0);
            let root = tree.root();
            for i in 0..1000 {
                tree.add_child(root, i);
            }
        });
    });
    c.bench_function("Create tree with 1,000 nodes deep", |b| {
        b.iter(|| {
            let mut tree = Tree::new_with_root(0);
            let mut node = tree.root();
            for i in 0..1000 {
                node = tree.add_child(node, i);
            }
        });
    });
}

pub fn create_vec_with_1000_nodes(c: &mut Criterion) {
    c.bench_function("Create vec with 1,000 nodes", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(Node::new(i));
            }
        });
    });
}

criterion_group!(
    benches,
    create_tree_with_1000_nodes,
    create_vec_with_1000_nodes
);
criterion_main!(benches);
