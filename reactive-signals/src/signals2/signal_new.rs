use std::marker::PhantomData;

use crate::{
    primitives::{any_data2::AnyData, dyn_func2::DynFunc},
    scope2::Scope,
    signals2::Signal,
};

use super::{signal_inner::SignalValue, SignalInner, SignalType};

impl<'rt, T: SignalType> Signal<'rt, T> {
    pub(crate) fn data(sx: Scope<'rt>, data: AnyData) -> Signal<'rt, T> {
        let rt = sx.rt.inner.borrow();
        let id = {
            let id = rt.scope_tree[sx.sx].next_signal_id(sx);
            // let id = scope.next_signal_id(sx);
            let signal = SignalInner {
                value: SignalValue::Data(data),
                listeners: Default::default(),
            };
            rt.scope_tree[sx.sx].insert_signal(signal);
            id
        };
        Signal {
            id,
            ty: PhantomData,
        }
    }

    pub(crate) fn func(sx: Scope<'rt>, func: impl FnOnce() -> DynFunc) -> Signal<'rt, T> {
        let rt = sx.rt.inner.borrow();
        let id = {
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
        };
        Signal {
            id,
            ty: PhantomData,
        }
    }
}
