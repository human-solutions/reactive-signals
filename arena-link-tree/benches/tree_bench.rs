use arena_link_tree::{Node, Tree};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn create_tree_with_1000_nodes(c: &mut Criterion) {
    c.bench_function("Create tree with 1,000 nodes wide", |b| {
        b.iter(|| {
            let mut tree = Tree::new_with_root(0);
            let root = tree.root();
            (0..1000).for_each(|i| _ = tree.add_child(root, i));
        });
    });
    c.bench_function("Create tree with 1,000 nodes deep", |b| {
        b.iter(|| {
            let mut tree = Tree::new_with_root(0);
            let mut node = tree.root();
            (0..1000).for_each(|i| node = tree.add_child(node, i));
        });
    });
}

pub fn delete_tree_with_1000_nodes(c: &mut Criterion) {
    let mut wide_tree = Tree::new_with_root(0);
    let root = wide_tree.root();
    (0..1000).for_each(|i| _ = wide_tree.add_child(root, i));

    c.bench_function("Delete tree with 1,000 nodes wide", |b| {
        b.iter(|| {
            let mut tree = wide_tree.clone();
            tree.reuse_tree(|_| {});
        });
    });

    let mut deep_tree = Tree::new_with_root(0);
    let mut node = deep_tree.root();
    (0..1000).for_each(|i| node = deep_tree.add_child(node, i));

    c.bench_function("Delete tree with 1,000 nodes deep", |b| {
        b.iter(|| {
            let mut tree = deep_tree.clone();
            tree.reuse_tree(|_| {});
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

pub fn sum_tree_with_1000_nodes(c: &mut Criterion) {
    let mut wide_tree = Tree::new_with_root(0);
    let root = wide_tree.root();
    (0..1000).for_each(|i| _ = wide_tree.add_child(root, i));
    c.bench_function("Sum tree 1,000 nodes wide", |b| {
        b.iter(|| {
            wide_tree
                .iter_from(wide_tree.root(), None)
                .map(|n| wide_tree[n])
                .sum::<i32>()
        });
    });
    let mut deep_tree = Tree::new_with_root(0);
    let root = deep_tree.root();
    (0..1000).for_each(|i| _ = deep_tree.add_child(root, i));

    c.bench_function("Sum tree 1,000 nodes deep", |b| {
        b.iter(|| {
            deep_tree
                .iter_from(wide_tree.root(), None)
                .map(|n| wide_tree[n])
                .sum::<i32>()
        });
    });
}

criterion_group!(
    benches,
    create_tree_with_1000_nodes,
    create_vec_with_1000_nodes,
    delete_tree_with_1000_nodes,
    sum_tree_with_1000_nodes,
);

criterion_main!(benches);
