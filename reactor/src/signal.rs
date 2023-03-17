use std::marker::PhantomData;

use crate::{
    primitives::{AnyData, DynFunc},
    scope::Scope,
    signal_id::SignalId,
    signal_inner::{SignalInner, SignalValue},
    updater::propagate_change,
};

pub struct Signal<T> {
    id: SignalId,
    ty: PhantomData<T>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            ty: self.ty,
        }
    }
}
impl<T> Copy for Signal<T> {}

impl<T: 'static> Signal<T> {
    fn data(sx: Scope, data: AnyData) -> Signal<T> {
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
    pub(crate) fn new_data(sx: Scope, data: T) -> Signal<T> {
        Self::data(sx, AnyData::new(data))
    }

    fn func(sx: Scope, func: impl FnOnce() -> DynFunc) -> Signal<T> {
        let id = sx.rt.with_ref(|rt| {
            let scope = &rt.scope_tree[sx.sx];
            let id = scope.next_signal_id(sx);

            let previous = rt.set_running_signal(Some(id));
            let signal = SignalInner {
                value: crate::signal_inner::SignalValue::Func(func()),
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
    pub(crate) fn new_func<F: Fn() -> T + 'static>(sx: Scope, func: F) -> Signal<T> {
        Self::func(sx, || DynFunc::new(func))
    }

    // #[cfg(test)]
    // fn with_inner<I, F: Fn(&SignalInner) -> I>(&self, f: F) -> I {
    //     self.id.rt_ref(|rt| rt[self.id].with_signal(self.id, f))
    // }
}

impl<T: Clone + 'static> Signal<T> {
    pub fn get(&self) -> T {
        self.id.rt_ref(|rt| {
            if let Some(listener) = rt.get_running_signal() {
                rt[self.id].with_signal(self.id, |signal| {
                    signal.listeners.insert(listener);
                    signal.get()
                })
            } else {
                rt[self.id].with_signal(self.id, |signal| signal.get())
            }
        })
    }

    pub fn set(&self, val: T) {
        self.id.rt_ref(|rt| {
            rt[self.id].with_signal_mut(self.id, |sig| sig.set(val));

            propagate_change(rt, self.id);
        });
    }
}
impl<T: PartialEq + 'static> Signal<T> {
    #[inline]
    pub(crate) fn new_func_eq<F: Fn() -> T + 'static>(sx: Scope, func: F) -> Signal<T> {
        Self::func(sx, || DynFunc::new_eq(func))
    }

    #[inline]
    pub(crate) fn new_data_eq(sx: Scope, data: T) -> Signal<T> {
        Self::data(sx, AnyData::new(data))
    }
}
