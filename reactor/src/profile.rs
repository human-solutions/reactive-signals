use crate::{create_data_signal, create_func_signal, Scope, Signal};

pub fn create_1000_nested() -> (Scope, Signal<usize>, Signal<usize>) {
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

pub fn create_1000_siblings() -> (Scope, Signal<usize>, Signal<usize>) {
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
