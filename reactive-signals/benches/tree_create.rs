use criterion::{criterion_group, criterion_main, Criterion};
use reactive_signals::{Node, Tree};

pub fn create_1000_nodes(c: &mut Criterion) {
    c.bench_function("Create tree and node with 1,000 children", |b| {
        b.iter(|| {
            let mut tree = Tree::create_and_init(0);
            let root = tree.root();
            (0..1000).for_each(|i| _ = tree.add_child(root, i));
            tree.discard_all()
        });
    });
    c.bench_function("Create tree and 1,000 nested nodes", |b| {
        b.iter(|| {
            let mut tree = Tree::create_and_init(0);
            let mut node = tree.root();
            (0..1000).for_each(|i| node = tree.add_child(node, i));
            tree.discard_all()
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

criterion_group!(benches, create_1000_nodes, create_vec_with_1000_nodes,);

criterion_main!(benches);
