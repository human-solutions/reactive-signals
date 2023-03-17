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
    #[cfg(test)]
    /// to determine if the signal is an implementation of PartialEq
    pub eq: bool,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            ty: self.ty,
            #[cfg(test)]
            eq: self.eq,
        }
    }
}
impl<T> Copy for Signal<T> {}

impl<T> Signal<T>
where
    T: Clone + Debug + 'static,
{
    pub fn get(&self) -> T {
        let val = self.id.rt_ref(|rt| {
            if let Some(listener) = rt.get_running_signal() {
                rt[self.id].with_signal(self.id, |signal| {
                    signal.listeners.insert(listener);
                    signal.get()
                })
            } else {
                rt[self.id].with_signal(self.id, |signal| signal.get())
            }
        });
        // println!("got: {:?} - {val:?}", self.id);

        val
    }

    pub fn set(&self, val: T) {
        // println!("set: {:?} - {val:?}", self.id);
        self.id.rt_ref(|rt| self.set_rt(val, rt));
    }

    pub(crate) fn set_rt(&self, val: T, rt: &RuntimeInner) {
        rt[self.id].with_signal_mut(self.id, |sig| sig.set(val));
        propagate_change(rt, self.id);
    }
}

pub fn create_data_signal<T: 'static>(sx: Scope, value: T) -> Signal<T> {
    let id = sx.rt.with_ref(|rt| {
        let scope = &rt.scope_tree[sx.sx];
        let id = scope.next_signal_id(sx);
        let signal = SignalInner {
            value: SignalValue::Data(AnyData::new(value)),
            listeners: Default::default(),
        };
        scope.insert_signal(signal);
        id
    });
    Signal {
        id,
        ty: PhantomData,
        #[cfg(test)]
        eq: false,
    }
}

pub fn create_func_signal<F, T>(sx: Scope, func: F) -> Signal<T>
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    let id = sx.rt.with_ref(|rt| {
        let scope = &rt.scope_tree[sx.sx];
        let id = scope.next_signal_id(sx);

        let previous = rt.set_running_signal(Some(id));
        let signal = SignalInner {
            value: SignalValue::Func(DynFunc::new(func)),
            listeners: Default::default(),
        };
        rt.set_running_signal(previous);
        scope.insert_signal(signal);
        id
    });
    Signal {
        id,
        ty: PhantomData,
        #[cfg(test)]
        eq: false,
    }
}

pub fn create_func_signal_eq<F, T>(sx: Scope, func: F) -> Signal<T>
where
    F: Fn() -> T + 'static,
    T: PartialEq + 'static,
{
    let id = sx.rt.with_ref(|rt| {
        let scope = &rt.scope_tree[sx.sx];
        let id = scope.next_signal_id(sx);

        let previous = rt.set_running_signal(Some(id));
        let signal = SignalInner {
            value: crate::signal_inner::SignalValue::Func(DynFunc::new_eq(func)),
            listeners: Default::default(),
        };
        rt.set_running_signal(previous);

        scope.insert_signal(signal);
        id
    });
    Signal {
        id,
        ty: PhantomData,
        #[cfg(test)]
        eq: true,
    }
}
