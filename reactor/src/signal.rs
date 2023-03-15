use std::{fmt::Debug, marker::PhantomData};

use crate::{
    primitives::{AnyData, DynFunc},
    runtime_inner::RuntimeInner,
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

impl<T> Signal<T>
where
    T: Clone + Debug + 'static,
{
    pub fn subscribe<S>(&self, sig: Signal<S>) {
        self.id.rt_mut(|rt| self.subscribe_rt(sig, rt));
    }

    pub(crate) fn subscribe_rt<S>(&self, sig: Signal<S>, rt: &mut RuntimeInner) {
        rt[sig.id].with_signal(sig.id, |signal| signal.listeners.insert(self.id));
    }

    pub fn get(&self) -> T {
        self.id.rt_ref(|rt| self.get_rt(rt))
    }

    pub(crate) fn get_rt(&self, rt: &RuntimeInner) -> T {
        rt[self.id].with_signal(self.id, |sig| sig.get())
    }

    pub fn set(&self, val: T) {
        self.id.rt_ref(|rt| self.set_rt(val, rt));
    }

    pub(crate) fn set_rt(&self, val: T, rt: &RuntimeInner) {
        rt[self.id].with_signal_mut(self.id, |sig| sig.set(val));
        propagate_change(rt, self.id);
    }
}

pub fn create_data_signal<T: 'static>(sx: Scope, value: T) -> Signal<T> {
    let id = sx.rt.with_mut(|rt| {
        let scope = &rt.scope_tree[sx.sx];
        let signal = SignalInner {
            value: SignalValue::Data(AnyData::new(value)),
            listeners: Default::default(),
        };
        scope.insert_signal(sx, signal)
    });
    Signal {
        id,
        ty: PhantomData,
    }
}

pub fn create_func_signal<F, T>(sx: Scope, func: F) -> Signal<T>
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    // When creating a signal it also runs once to get the initial value
    // We need to keep this out of the rt so there's no mut ref.
    let signal = SignalInner {
        value: SignalValue::Func(DynFunc::new(func)),
        listeners: Default::default(),
    };
    let id = sx.rt.with_mut(|rt| {
        let scope = &rt.scope_tree[sx.sx];
        scope.insert_signal(sx, signal)
    });
    Signal {
        id,
        ty: PhantomData,
    }
}

pub fn create_func_signal_eq<F, T>(sx: Scope, func: F) -> Signal<T>
where
    F: Fn() -> T + 'static,
    T: PartialEq + 'static,
{
    // When creating a signal it also runs once to get the initial value
    // We need to keep this out of the rt so there's no mut ref.
    let signal = SignalInner {
        value: crate::signal_inner::SignalValue::Func(DynFunc::new_eq(func)),
        listeners: Default::default(),
    };
    let id = sx.rt.with_mut(|rt| {
        let scope = &rt.scope_tree[sx.sx];
        scope.insert_signal(sx, signal)
    });
    Signal {
        id,
        ty: PhantomData,
    }
}
