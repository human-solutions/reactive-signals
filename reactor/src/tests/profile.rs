//! This is used in benches and in flamegraph examples
//!

use crate::{
    primitives::{Data, Func},
    runtimes::{SingleRuntime, SingleRuntimeId},
    Scope, Signal,
};

pub fn create_1000_nested() -> (
    Scope<SingleRuntimeId>,
    Signal<Data<usize>, SingleRuntimeId>,
    Signal<Func<usize>, SingleRuntimeId>,
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
    Signal<Data<usize>, SingleRuntimeId>,
    Signal<Func<usize>, SingleRuntimeId>,
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

#[test]
fn test_1000_nested() {
    let (_sx, start, end) = create_1000_nested();

    start.set(1);
    assert_eq!(end.get(), 1002);
}

#[test]
fn test_1000_siblings() {
    let (_sx, start, end) = create_1000_siblings();

    start.set(1);
    assert_eq!(end.get(), 1002);
}
