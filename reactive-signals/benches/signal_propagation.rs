use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

pub fn signal_propagation(c: &mut Criterion) {
    c.bench_function(
        "Propagate a change through 1000 signals, each in a nested scope",
        |b| {
            b.iter_batched(
                reactive_signals::tests::profile::create_1000_nested_scopes_each_with_a_signal,
                |(_scope, start_sig, end_sig)| {
                    start_sig.set(2);
                    black_box(end_sig.get())
                },
                BatchSize::SmallInput,
            );
        },
    );
    // if end_sig.get() != 1001 {
    //     panic!("end_sig.get() {}", end_sig.get());
    // }

    c.bench_function(
        "Propagate a change through 1000 signals, each in a sibling scope",
        |b| {
            b.iter_batched(
                reactive_signals::tests::profile::create_1000_siblings,
                |(_scope, start_sig, end_sig)| {
                    start_sig.set(2);
                    black_box(end_sig.get())
                },
                BatchSize::SmallInput,
            );
        },
    );

    // if end_sig.get() != 1003 {
    //     panic!("end_sig.get() {}", end_sig.get());
    // }
}

criterion_group!(benches, signal_propagation,);

criterion_main!(benches);
