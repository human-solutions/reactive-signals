///
/// The original benchmarks from the Leptos repository.
///
/// Run with:
///     `cargo bench`
///
use criterion::{criterion_group, criterion_main, Criterion};
use leptos::create_runtime;

pub fn leptos_create_1000_signals(c: &mut Criterion) {
    let runtime = create_runtime();
    c.bench_function("Leptos create 1000 signals", |b| {
        b.iter(|| benchmarks::leptos_create_1000_signals(runtime));
    });
}

fn leptos_create_and_update_1000_signals(c: &mut Criterion) {
    let runtime = create_runtime();
    c.bench_function("Leptos create and update 1000 signals", |b| {
        b.iter(|| benchmarks::leptos_create_and_update_1000_signals(runtime));
    });
}

fn leptos_create_and_dispose_1000_scopes(c: &mut Criterion) {
    let runtime = create_runtime();
    c.bench_function("Leptos create and dispose 1000 scopes", |b| {
        b.iter(|| benchmarks::leptos_create_and_dispose_1000_scopes(runtime));
    });
}
criterion_group!(
    benches,
    leptos_create_1000_signals,
    leptos_create_and_update_1000_signals,
    leptos_create_and_dispose_1000_scopes,
);
criterion_main!(benches);
