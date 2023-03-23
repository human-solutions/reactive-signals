use std::{hash::Hash, marker::PhantomData};

use crate::{
    primitives::{AnyData, Compare, Data, DynFunc, EqData, HashEqData},
    runtimes::Runtime,
    scope::Scope,
    Signal,
};

use super::{SignalInner, SignalValue};

impl<T: 'static + Compare, RT: Runtime> Signal<T, RT> {
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
}

impl<T: 'static, RT: Runtime> Signal<Data<T>, RT> {
    #[inline]
    pub(crate) fn new_func<F: Fn() -> T + 'static>(sx: Scope<RT>, func: F) -> Signal<Data<T>, RT> {
        Self::func(sx, || DynFunc::new(func))
    }

    #[inline]
    pub(crate) fn new_data(sx: Scope<RT>, data: T) -> Signal<Data<T>, RT> {
        Self::data(sx, AnyData::new(Data(data)))
    }
}

impl<T: PartialEq + 'static, RT: Runtime> Signal<EqData<T>, RT> {
    #[inline]
    pub(crate) fn new_func_eq<F: Fn() -> T + 'static>(
        sx: Scope<RT>,
        func: F,
    ) -> Signal<EqData<T>, RT> {
        Self::func(sx, || DynFunc::new_eq(func))
    }

    #[inline]
    pub(crate) fn new_data_eq(sx: Scope<RT>, data: T) -> Signal<EqData<T>, RT> {
        Self::data(sx, AnyData::new(EqData(data)))
    }
}

impl<T: PartialEq + Hash + 'static, RT: Runtime> Signal<EqData<T>, RT> {
    #[inline]
    pub(crate) fn new_data_hash_eq(sx: Scope<RT>, data: T) -> Signal<EqData<T>, RT> {
        Self::data(sx, AnyData::new(HashEqData(data)))
    }
}
