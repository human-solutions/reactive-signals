use crate::{runtime_inner::RuntimeInner, signal_id::SignalId, signal_inner::SignalValue};

pub(crate) fn propagate_change(rt: &RuntimeInner, sig: SignalId) {
    let mut queue = rt.scopes[sig.sx.sx].signals.borrow()[sig.index()]
        .listeners
        .clone();

    while let Some(sig) = queue.pop() {
        let scope = &rt.scopes[sig.sx.sx];

        let signal = &scope.signals.borrow()[sig.index()];

        queue.extend(signal.listeners.clone());

        if let SignalValue::Func(func) = &signal.value {
            func.run();
        }
    }
}
