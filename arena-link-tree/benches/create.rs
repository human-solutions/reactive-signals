use arena_link_tree::{Node, Tree};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn create_tree_with_1000_nodes(c: &mut Criterion) {
    c.bench_function("Create tree with 1,000 nodes wide", |b| {
        b.iter(|| {
            let mut tree = Tree::create_and_init(0);
            let root = tree.root();
            (0..1000).for_each(|i| _ = tree.add_child(root, i));
            tree.discard_all(|_| {})
        });
    });
    c.bench_function("Create tree with 1,000 nodes deep", |b| {
        b.iter(|| {
            let mut tree = Tree::create_and_init(0);
            let mut node = tree.root();
            (0..1000).for_each(|i| node = tree.add_child(node, i));
            tree.discard_all(|_| {})
        });
    });
}

pub fn create_vec_with_1000_nodes(c: &mut Criterion) {
    c.bench_function("Create vec with 1,000 nodes", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            (0..1000).for_each(|i| vec.push(Node::new(i)));
        });
    });
}

criterion_group!(
    benches,
    create_tree_with_1000_nodes,
    create_vec_with_1000_nodes,
);

criterion_main!(benches);
