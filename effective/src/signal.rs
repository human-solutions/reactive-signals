use std::{fmt::Debug, marker::PhantomData};

use slotmap::new_key_type;

use crate::{scope::Scope, signal_inner::SignalInner, Runtime};

new_key_type! { pub struct SignalId; }

pub struct Signal<T> {
    rt: Runtime,
    id: SignalId,
    ty: PhantomData<T>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            rt: self.rt,
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
        self.rt
            .with_signal_mut(sig.id, |_, signal| signal.add_listener(self.id));
    }

    pub fn get(&self) -> T {
        self.rt.with_signal(self.id, |_, sig| sig.get())
    }

    pub fn set(&self, val: T) {
        self.rt.with_signal(self.id, |rt, sig| sig.set(rt, val))
    }
}

pub fn create_data_signal<T: 'static>(cx: Scope, value: T) -> Signal<T> {
    Signal {
        rt: cx.rt,
        id: cx.rt.insert_signal(SignalInner::new_data(cx.id, value)),
        ty: PhantomData,
    }
}

pub fn create_func_signal<F, T>(cx: Scope, func: F) -> Signal<T>
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    Signal {
        rt: cx.rt,
        id: cx.rt.insert_signal(SignalInner::new_func(cx.id, func)),
        ty: PhantomData,
    }
}
