//! From https://github.com/tikv/pprof-rs/blob/HEAD/examples/criterion.rs
//!
//! Run with:
//! `cargo run --example propagation_perf --features="flamegraph criterion" --profile=dev -- --bench --profile-time 2`
//! Open svg with:
//! `open target/criterion/<name-of-benchmark>/profile/flamegraph.svg`
//!
//! Note that it frequently fails with a `trace trap`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use reactor::{create_data_signal, create_func_signal, Scope};

pub fn signal_propagation(c: &mut Criterion) {
    let mut scope = Scope::bench_root();
    let start_sig = create_data_signal(scope, 0u32);
    let mut next_sig = create_func_signal(scope, move || start_sig.get() + 1);
    next_sig.subscribe(start_sig);

    (0..1000).for_each(|_| {
        scope = scope.new_child();
        let sig = create_func_signal(scope, move || next_sig.get() + 1);
        sig.subscribe(next_sig);
        next_sig = sig;
    });

    let end_sig = next_sig;
    c.bench_function(
        "Profile propagate a change through 1000 signals, each in a nested scope",
        |b| {
            b.iter(|| {
                start_sig.set(2);
                black_box(end_sig.get())
            });
        },
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = signal_propagation,
}
criterion_main!(benches);
