use std::marker::PhantomData;

use crate::{
    primitives::{AnyData, DynFunc},
    scope::Scope,
    signals::Signal,
};

use super::{signal_inner::SignalValue, SignalInner, SignalType};

impl<T: SignalType> Signal<T> {
    pub(crate) fn data(sx: Scope, data: AnyData) -> Signal<T> {
        let id = sx.rt.with_ref(|rt| {
            let id = rt.scope_tree[sx.sx].next_signal_id(sx);
            // let id = scope.next_signal_id(sx);
            let signal = SignalInner {
                value: SignalValue::Data(data),
                listeners: Default::default(),
            };
            rt.scope_tree[sx.sx].insert_signal(signal);
            id
        });
        Signal {
            id,
            ty: PhantomData,
        }
    }

    pub(crate) fn func(sx: Scope, func: impl FnOnce() -> DynFunc) -> Signal<T> {
        let id = sx.rt.with_ref(|rt| {
            let scope = &rt.scope_tree[sx.sx];
            let id = scope.next_signal_id(sx);

            let previous = rt.set_running_signal(Some(id));
            let signal = SignalInner {
                value: SignalValue::Func(func()),
                listeners: Default::default(),
            };
            rt.set_running_signal(previous);

            scope.insert_signal(signal);
            id
        });
        Signal {
            id,
            ty: PhantomData,
        }
    }
}
