//! From https://github.com/tikv/pprof-rs/blob/HEAD/examples/criterion.rs
//!
//! Run with:
//! `cargo run --example creation_flamegraph --features="profile" -- --bench --profile-time 5`
//! Open svg with:
//! `open target/criterion/creation_flamegraph/profile/flamegraph.svg`
//!
//! Note that it frequently fails with a `trace trap`

use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

pub fn creation_flamegraph(c: &mut Criterion) {
    c.bench_function("creation_flamegraph", |b| {
        b.iter(reactor::tests::profile::create_1000_nested_scopes_each_with_a_signal);
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
    targets = creation_flamegraph,
}
criterion_main!(benches);
