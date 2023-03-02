#![allow(dead_code)]

use crate::{
    runtime_inner::{RuntimeInner, RUNTIMES},
    scope::Scope,
    signal::SignalId,
    signal_inner::SignalInner,
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Runtime(pub(crate) u32);

impl Runtime {
    pub fn from_pool() -> Self {
        RUNTIMES.with(|pool| pool.borrow())
    }

    pub fn return_to_pool(&self) {
        RUNTIMES.with(|pool| pool.return_to_pool(self))
    }

    pub(crate) fn from(idx: usize) -> Self {
        if idx >= u32::MAX as usize {
            panic!("Too many runtimes. Check your code for leaks. A runtime needs to be discarded");
        }
        Self(idx as u32)
    }

    pub fn insert_signal(&self, signal: SignalInner) -> SignalId {
        RUNTIMES.with(|pool| pool.0.borrow()[self.0 as usize].insert_signal(signal))
    }

    pub(crate) fn with_signal<F, T>(&self, id: SignalId, f: F) -> T
    where
        F: FnOnce(&RuntimeInner, &SignalInner) -> T,
    {
        RUNTIMES.with(|pool| {
            let rt = &pool.0.borrow()[self.0 as usize];
            let signals = rt.signals.borrow();
            let signal = signals.get(id).unwrap();
            f(rt, &signal)
        })
    }

    pub(crate) fn with_signal_mut<F, T>(&self, id: SignalId, f: F) -> T
    where
        F: FnOnce(&RuntimeInner, &mut SignalInner) -> T,
    {
        RUNTIMES.with(|pool| {
            let rt = &pool.0.borrow()[self.0 as usize];
            let mut signals = rt.signals.borrow_mut();
            let mut signal = signals.get_mut(id).unwrap();
            f(&rt, &mut signal)
        })
    }

    pub fn create_scope(&self) -> Scope {
        RUNTIMES.with(|pool| pool.0.borrow()[self.0 as usize].create_scope())
    }

    pub(crate) fn discard_scope(&self, scope: Scope) {
        RUNTIMES.with(|pool| pool.0.borrow()[self.0 as usize].discard_scope(scope))
    }
}
