use crate::{runtime_inner::RuntimeInner, signal_id::SignalId, signal_inner::SignalValue};

pub(crate) fn propagate_change(rt: &RuntimeInner, sig: SignalId) {
    let mut queue = rt.scope_tree[sig.sx].signals.borrow()[sig.index()]
        .listeners
        .clone();

    while let Some(sig) = queue.pop() {
        let scope = &rt.scope_tree[sig.sx];

        let signal = &scope.signals.borrow()[sig.index()];

        if let SignalValue::Func(func) = &signal.value {
            let did_change = func.run();
            if did_change {
                queue.extend(signal.listeners.clone());
            }
        }
    }
}
