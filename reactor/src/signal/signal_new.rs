use std::marker::PhantomData;

use crate::{
    primitives::{AnyData, DynFunc},
    runtimes::Runtime,
    scope::Scope,
    Signal,
};

use super::{SignalInner, SignalType, SignalValue};

impl<T: 'static + SignalType, RT: Runtime> Signal<T, RT> {
    pub(crate) fn data(sx: Scope<RT>, data: AnyData) -> Signal<T, RT> {
        let id = sx.rt.with_ref(|rt| {
            let scope = &rt.scope_tree[sx.sx];
            let id = scope.next_signal_id(sx);
            let signal = SignalInner {
                value: SignalValue::Data(data),
                listeners: Default::default(),
            };
            scope.insert_signal(signal);
            id
        });
        Signal {
            id,
            ty: PhantomData,
        }
    }

    pub(crate) fn func(sx: Scope<RT>, func: impl FnOnce() -> DynFunc) -> Signal<T, RT> {
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
