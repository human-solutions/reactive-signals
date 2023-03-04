use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use effective::{create_data_signal, create_func_signal, Scope};

pub fn delete_scopes_with_signal(c: &mut Criterion) {
    c.bench_function("Delete 1,000 nested scopes with 1 func signal each", |b| {
        b.iter_batched(
            || {
                let base = Scope::bench_root().new_child();
                let mut scope = base;
                let sig = create_data_signal(scope, 0u32);
                (0..1000).for_each(|_| {
                    scope = scope.new_child();
                    create_func_signal(scope, move || sig.get() + 1);
                });
                base
            },
            |base| base.discard(),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("Delete 1,000 sibling scopes with 1 func signal each", |b| {
        b.iter_batched(
            || {
                let root = Scope::bench_root();
                let scope = root.new_child();
                let sig = create_data_signal(root, 0u32);
                (0..1000).for_each(|_| {
                    let sx = scope.new_child();
                    create_func_signal(sx, move || sig.get() + 1);
                });
                scope
            },
            |scope| scope.discard(),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, delete_scopes_with_signal,);

criterion_main!(benches);
