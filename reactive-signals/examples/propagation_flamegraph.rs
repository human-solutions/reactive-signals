//! From https://github.com/tikv/pprof-rs/blob/HEAD/examples/criterion.rs
//!
//! Run with:
//! `cargo run --example propagation_flamegraph --features="profile" -- --bench --profile-time 5`
//! Open svg with:
//! `open target/criterion/propagation_flamegraph/profile/flamegraph.svg`
//!
//! Note that it frequently fails with a `trace trap`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

pub fn propagation_flamegraph(c: &mut Criterion) {
    c.bench_function("propagation_flamegraph", |b| {
        b.iter_batched(
            reactive_signals::tests::profile::create_1000_nested_scopes_each_with_a_signal,
            |(_, start_sig, end_sig)| {
                start_sig.set(2);
                black_box(end_sig.get())
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
    targets = propagation_flamegraph,
}
criterion_main!(benches);
