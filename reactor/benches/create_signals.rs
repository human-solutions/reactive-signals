use criterion::{criterion_group, criterion_main, Criterion};

pub fn create_scopes_with_signal(c: &mut Criterion) {
    c.bench_function("Create 1,000 data signals", |b| {
        b.iter(reactor::tests::profile::create_1000_data_signals);
    });
    c.bench_function("Create 1,000 func signals", |b| {
        b.iter(reactor::tests::profile::create_1000_func_signals);
    });
    c.bench_function("Comparative with leptos create 1000 signals", |b| {
        b.iter(reactor::tests::profile::comparative_with_leptos_create_1000_signals);
    });
}

criterion_group!(benches, create_scopes_with_signal,);

criterion_main!(benches);
