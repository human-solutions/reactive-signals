use arena_link_tree::Tree;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

pub fn delete_tree_with_1000_nodes(c: &mut Criterion) {
    let mut wide_tree = Tree::create_and_init(0);
    let root = wide_tree.root();
    (0..1000).for_each(|i| _ = wide_tree.add_child(root, i));

    c.bench_function("Delete tree with 1,000 nodes wide", |b| {
        b.iter_batched(
            || wide_tree.clone(),
            |mut tree| tree.discard_all(|_| {}),
            BatchSize::SmallInput,
        );
    });

    let mut deep_tree = Tree::create_and_init(0);
    let mut node = deep_tree.root();
    (0..1000).for_each(|i| node = deep_tree.add_child(node, i));

    c.bench_function("Delete tree with 1,000 nodes deep", |b| {
        b.iter_batched(
            || deep_tree.clone(),
            |mut tree| tree.discard_all(|_| {}),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, delete_tree_with_1000_nodes,);

criterion_main!(benches);
