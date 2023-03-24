use criterion::{criterion_group, criterion_main, Criterion};

pub fn create_scopes_with_signal(c: &mut Criterion) {
    c.bench_function("Create 1,000 nested scopes with 1 func signal each", |b| {
        b.iter(reactor::tests::profile::create_1000_nested_scopes_each_with_a_signal);
    });
    c.bench_function("Create 1,000 sibling scopes with 1 func signal each", |b| {
        b.iter(reactor::tests::profile::create_1000_siblings);
    });

    c.bench_function("Create 1,000 nested scopes", |b| {
        b.iter(reactor::tests::profile::create_1000_nested_scopes);
    });
}

criterion_group!(benches, create_scopes_with_signal,);

criterion_main!(benches);
