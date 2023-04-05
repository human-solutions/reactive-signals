use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use reactive_signals::Tree;

pub fn discard_1000_nodes(c: &mut Criterion) {
    let mut wide_tree = Tree::create_and_init(0);
    let root = wide_tree.root();
    (0..1000).for_each(|i| _ = wide_tree.add_child(root, i));

    c.bench_function("Discard node with 1,000 children", |b| {
        b.iter_batched(
            || wide_tree.clone(),
            |mut tree| tree.discard(tree.root(), |_| {}),
            BatchSize::SmallInput,
        );
    });

    let mut deep_tree = Tree::create_and_init(0);
    let mut node = deep_tree.root();
    (0..1000).for_each(|i| node = deep_tree.add_child(node, i));

    c.bench_function("Discard 1,000 nested nodes", |b| {
        b.iter_batched(
            || deep_tree.clone(),
            |mut tree| tree.discard(tree.root(), |_| {}),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, discard_1000_nodes,);

criterion_main!(benches);
