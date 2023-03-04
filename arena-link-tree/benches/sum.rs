use arena_link_tree::Tree;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn sum_tree_with_1000_nodes(c: &mut Criterion) {
    let mut wide_tree = Tree::new_with_root(0);
    let root = wide_tree.root();
    (0..1000).for_each(|i| _ = wide_tree.add_child(root, i));
    c.bench_function("Sum tree 1,000 nodes wide", |b| {
        b.iter(|| {
            wide_tree
                .iter_from(wide_tree.root())
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
                .iter_from(wide_tree.root())
                .map(|n| wide_tree[n])
                .sum::<i32>()
        });
    });
}

criterion_group!(benches, sum_tree_with_1000_nodes,);

criterion_main!(benches);
