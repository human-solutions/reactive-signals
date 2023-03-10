use criterion::{criterion_group, criterion_main, Criterion};
use effective::{create_data_signal, create_func_signal, Scope};

pub fn create_scopes_with_signal(c: &mut Criterion) {
    c.bench_function("Create 1,000 nested scopes with 1 func signal each", |b| {
        b.iter(|| {
            let mut scope = Scope::bench_root();

            let sig = create_data_signal(scope, 0u32);
            (0..1000).for_each(|_| {
                scope = scope.new_child();
                create_func_signal(scope, move || sig.get() + 1);
            });
        });
    });
    c.bench_function("Create 1,000 sibling scopes with 1 func signal each", |b| {
        b.iter(|| {
            let scope = Scope::bench_root();
            let sig = create_data_signal(scope, 0u32);
            (0..1000).for_each(|_| {
                let sx = scope.new_child();
                create_func_signal(sx, move || sig.get() + 1);
            });
        });
    });
}

criterion_group!(benches, create_scopes_with_signal,);

criterion_main!(benches);
