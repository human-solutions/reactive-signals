use std::marker::PhantomData;

use crate::{
    primitives::{AnyData, DynFunc},
    runtimes::Runtime,
    scope::Scope,
    Signal,
};

use super::{SignalInner, SignalValue};

impl<T: 'static, RT: Runtime> Signal<T, RT> {
    pub(super) fn data(sx: Scope<RT>, data: AnyData) -> Signal<T, RT> {
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

    #[inline]
    pub(crate) fn new_data(sx: Scope<RT>, data: T) -> Signal<T, RT> {
        Self::data(sx, AnyData::new(data))
    }

    pub(super) fn func(sx: Scope<RT>, func: impl FnOnce() -> DynFunc) -> Signal<T, RT> {
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

    #[inline]
    pub(crate) fn new_func<F: Fn() -> T + 'static>(sx: Scope<RT>, func: F) -> Signal<T, RT> {
        Self::func(sx, || DynFunc::new(func))
    }
}

impl<T: PartialEq + 'static, RT: Runtime> Signal<T, RT> {
    #[inline]
    pub(crate) fn new_func_eq<F: Fn() -> T + 'static>(sx: Scope<RT>, func: F) -> Signal<T, RT> {
        Self::func(sx, || DynFunc::new_eq(func))
    }

    #[inline]
    pub(crate) fn new_data_eq(sx: Scope<RT>, data: T) -> Signal<T, RT> {
        Self::data(sx, AnyData::new(data))
    }
}
