use arena_link_tree::Tree;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn sum_1000_nodes(c: &mut Criterion) {
    let mut wide_tree = Tree::create_and_init(0);
    let root = wide_tree.root();
    (0..1000).for_each(|i| _ = wide_tree.add_child(root, i));
    c.bench_function("Sum numeric tree node with 1,000 children", |b| {
        b.iter(|| {
            wide_tree
                .iter_from(wide_tree.root())
                .map(|n| wide_tree[n])
                .sum::<i32>()
        });
    });
    let mut deep_tree = Tree::create_and_init(0);
    let root = deep_tree.root();
    (0..1000).for_each(|i| _ = deep_tree.add_child(root, i));

    c.bench_function("Sum numeric tree with 1,000 nested nodes", |b| {
        b.iter(|| {
            deep_tree
                .iter_from(wide_tree.root())
                .map(|n| wide_tree[n])
                .sum::<i32>()
        });
    });
}

criterion_group!(benches, sum_1000_nodes,);

criterion_main!(benches);
