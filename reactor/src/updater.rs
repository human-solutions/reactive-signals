use crate::{runtime_inner::RuntimeInner, signal_id::SignalId, signal_inner::SignalValue};

pub(crate) fn propagate_change(rt: &RuntimeInner, sig: SignalId) {
    let mut queue = rt.scope_tree[sig.sx].signals.borrow()[sig.index()]
        .listeners
        .clone();

    while let Some(sig) = queue.pop() {
        let scope = &rt.scope_tree[sig.sx];

        // step one
        let upd = {
            let signal = &scope.signals.borrow()[sig.index()];

            queue.extend(signal.listeners.clone());

            if let SignalValue::Func(func) = &signal.value {
                Some(func.run())
            } else {
                None
            }
        };

        if let Some(upd) = upd {
            let signal = &mut scope.signals.borrow_mut()[sig.index()];

            if let SignalValue::Func(func) = &mut signal.value {
                func.set_value(upd)
            } else {
                panic!()
            }
        }
    }
}
