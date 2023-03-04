use criterion::{black_box, criterion_group, criterion_main, Criterion};
use effective::{create_data_signal, create_func_signal, Scope};

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
        "Propagate a change through 1000 signals, each in a nested scope",
        |b| {
            b.iter(|| {
                start_sig.set(2);
                black_box(end_sig.get())
            });
        },
    );
    // if end_sig.get() != 1001 {
    //     panic!("end_sig.get() {}", end_sig.get());
    // }

    let scope = Scope::bench_root();
    let start_sig = create_data_signal(scope, 0u32);
    let mut next_sig = create_func_signal(scope, move || start_sig.get() + 1);
    next_sig.subscribe(start_sig);

    (0..1000).for_each(|_| {
        let sx = scope.new_child();
        let sig = create_func_signal(sx, move || next_sig.get() + 1);
        sig.subscribe(next_sig);
        next_sig = sig;
    });

    let end_sig = next_sig;
    c.bench_function(
        "Propagate a change through 1000 signals, each in a sibling scope",
        |b| {
            b.iter(|| {
                start_sig.set(2);
                black_box(end_sig.get())
            });
        },
    );

    // if end_sig.get() != 1003 {
    //     panic!("end_sig.get() {}", end_sig.get());
    // }
}

criterion_group!(benches, signal_propagation,);

criterion_main!(benches);
