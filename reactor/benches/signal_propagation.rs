use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use reactor::{create_data_signal, create_func_signal, Scope, Signal};

fn create_1000_nested() -> (Scope, Signal<usize>, Signal<usize>) {
    let mut scope = Scope::bench_root();
    let start_sig = create_data_signal(scope, 0usize);
    let mut next_sig = create_func_signal(scope, move || start_sig.get() + 1);

    (0..1000).for_each(|_| {
        scope = scope.new_child();
        let sig = create_func_signal(scope, move || next_sig.get() + 1);
        next_sig = sig;
    });

    let end_sig = next_sig;
    (scope, start_sig, end_sig)
}

fn create_1000_siblings() -> (Scope, Signal<usize>, Signal<usize>) {
    let scope = Scope::bench_root();
    let start_sig = create_data_signal(scope, 0usize);
    let mut next_sig = create_func_signal(scope, move || start_sig.get() + 1);

    (0..1000).for_each(|_| {
        let sx = scope.new_child();
        let sig = create_func_signal(sx, move || next_sig.get() + 1);
        next_sig = sig;
    });

    let end_sig = next_sig;
    (scope, start_sig, end_sig)
}

pub fn signal_propagation(c: &mut Criterion) {
    c.bench_function(
        "Propagate a change through 1000 signals, each in a nested scope",
        |b| {
            b.iter_batched(
                create_1000_nested,
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
                create_1000_siblings,
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
