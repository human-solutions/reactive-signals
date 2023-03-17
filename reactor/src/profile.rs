use crate::{Scope, Signal, SingleRuntime, SingleRuntimeId};

pub fn create_1000_nested() -> (
    Scope<SingleRuntimeId>,
    Signal<usize, SingleRuntimeId>,
    Signal<usize, SingleRuntimeId>,
) {
    let mut scope = SingleRuntime::bench_root_scope();

    // don't use the signal! macro, because we want to force the signals to
    // be non equals. Otherwise a propagation wouldn't happen
    let start_sig = Signal::new_data(scope, 0usize);
    let mut next_sig = Signal::new_func(scope, move || start_sig.get() + 1);

    (0..1000).for_each(|_| {
        scope = scope.new_child();
        let sig = Signal::new_func(scope, move || next_sig.get() + 1);
        next_sig = sig;
    });

    let end_sig = next_sig;
    (scope, start_sig, end_sig)
}

pub fn create_1000_siblings() -> (
    Scope<SingleRuntimeId>,
    Signal<usize, SingleRuntimeId>,
    Signal<usize, SingleRuntimeId>,
) {
    let scope = SingleRuntime::bench_root_scope();
    // don't use the signal! macro, because we want to force the signals to
    // be non equals. Otherwise a propagation wouldn't happen
    let start_sig = Signal::new_data(scope, 0usize);
    let mut next_sig = Signal::new_func(scope, move || start_sig.get() + 1);

    (0..1000).for_each(|_| {
        let sx = scope.new_child();
        let sig = Signal::new_func(sx, move || next_sig.get() + 1);
        next_sig = sig;
    });

    let end_sig = next_sig;
    (scope, start_sig, end_sig)
}
