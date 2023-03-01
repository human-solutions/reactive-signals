#![allow(dead_code)]

use std::cell::{Cell, RefCell};

use slotmap::SlotMap;

use crate::{
    scope::{Scope, ScopeId},
    signal::SignalId,
    signal_inner::SignalInner,
};

thread_local! {
    pub(crate) static RUNTIMES: RuntimePool = Default::default();
}

#[derive(Default)]
pub(crate) struct RuntimePool(RefCell<Vec<Runtime>>);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct RuntimeId(u32);

impl RuntimeId {
    fn from(idx: usize) -> Self {
        if idx >= u32::MAX as usize {
            panic!("Too many runtimes. Check your code for leaks. A runtime needs to be discarded");
        }
        Self(idx as u32)
    }

    pub fn insert_signal(&self, signal: SignalInner) -> SignalId {
        RUNTIMES.with(|pool| pool.0.borrow()[self.0 as usize].insert_signal(signal))
    }

    pub fn with_signal<F, T>(&self, id: SignalId, f: F) -> T
    where
        F: FnOnce(&Runtime, &SignalInner) -> T,
    {
        RUNTIMES.with(|pool| {
            let rt = &pool.0.borrow()[self.0 as usize];
            let signals = rt.signals.borrow();
            let signal = signals.get(id).unwrap();
            f(rt, &signal)
        })
    }

    pub fn with_signal_mut<F, T>(&self, id: SignalId, f: F) -> T
    where
        F: FnOnce(&Runtime, &mut SignalInner) -> T,
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

impl RuntimePool {
    fn new_rt(&self) -> RuntimeId {
        {
            for (idx, rt) in self.0.borrow().iter().enumerate() {
                if !rt.in_use.get() {
                    rt.in_use.set(true);
                    return RuntimeId::from(idx);
                }
            }
        }
        let mut vec = self.0.borrow_mut();
        let id = RuntimeId::from(vec.len());
        vec.push(Runtime::new(id));
        id
    }

    fn put(&self, runtime: Runtime) {
        let mut pool = self.0.borrow_mut();
        pool.push(runtime);
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Runtime {
    pub(crate) id: RuntimeId,
    in_use: Cell<bool>,
    scope_counter: Cell<u32>,
    pub(crate) signals: RefCell<SlotMap<SignalId, SignalInner>>,
}

impl Runtime {
    pub fn with<F, T>(rt: RuntimeId, f: F) -> T
    where
        F: FnOnce(&Runtime) -> T,
    {
        RUNTIMES.with(|pool| {
            let rt = &pool.0.borrow()[rt.0 as usize];
            f(&rt)
        })
    }

    pub fn from_pool() -> RuntimeId {
        RUNTIMES.with(|pool| pool.new_rt())
    }

    fn new(id: RuntimeId) -> Self {
        Self {
            id,
            in_use: Cell::new(true),
            scope_counter: Cell::new(0),
            signals: RefCell::new(SlotMap::with_key()),
        }
    }

    pub(crate) fn create_scope(&self) -> Scope {
        let count = self.scope_counter.get();
        self.scope_counter.set(count + 1);
        Scope::new(ScopeId(count), &self)
    }

    pub(crate) fn discard_scope(&self, cx: Scope) {
        self.signals
            .borrow_mut()
            .values_mut()
            .filter(|s| s.scope() == cx.id)
            .for_each(|signal| signal.discard(&self));
    }

    pub fn discard(&self) {
        self.in_use.set(false);
        self.signals.borrow_mut().clear();
    }

    pub fn insert_signal(&self, signal: SignalInner) -> SignalId {
        let mut signals = self.signals.borrow_mut();
        let id = signals.insert(signal);
        unsafe {
            signals.get_unchecked_mut(id).set_id(id);
        }
        id
    }
}
