#![allow(dead_code)]

use std::cell::{Cell, RefCell};

use signal::{SignalContent, SignalId};
use slotmap::SlotMap;

use crate::signal;

thread_local! {
    pub(crate) static RUNTIMES: RuntimePool = Default::default();
}

#[derive(Default)]
pub(crate) struct RuntimePool(RefCell<Vec<Runtime>>);

#[derive(Default, Clone, Copy)]
pub struct RuntimeId(u32);

impl RuntimeId {
    fn from(idx: usize) -> Self {
        if idx >= u32::MAX as usize {
            panic!("Too many runtimes. Check your code for leaks. A runtime needs to be discarded");
        }
        Self(idx as u32)
    }

    pub fn insert_signal(&self, signal: SignalContent) -> SignalId {
        RUNTIMES.with(|pool| {
            let rt = &pool.0.borrow()[self.0 as usize];
            rt.insert_signal(signal)
        })
    }

    pub fn with_signal<F, T>(&self, id: SignalId, f: F) -> T
    where
        F: FnOnce(&Runtime, &SignalContent) -> T,
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
        F: FnOnce(&Runtime, &mut SignalContent) -> T,
    {
        RUNTIMES.with(|pool| {
            let rt = &pool.0.borrow()[self.0 as usize];
            let mut signals = rt.signals.borrow_mut();
            let mut signal = signals.get_mut(id).unwrap();
            f(&rt, &mut signal)
        })
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
        vec.push(Runtime::new());
        RuntimeId::from(vec.len() - 1)
    }

    fn put(&self, runtime: Runtime) {
        let mut pool = self.0.borrow_mut();
        pool.push(runtime);
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Runtime {
    in_use: Cell<bool>,
    pub(crate) signals: RefCell<SlotMap<SignalId, SignalContent>>,
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

    fn new() -> Self {
        Self {
            in_use: Cell::new(true),
            signals: RefCell::new(SlotMap::with_key()),
        }
    }

    pub fn discard(&self) {
        self.in_use.set(false);
        self.signals.borrow_mut().clear();
    }

    pub fn insert_signal(&self, signal: SignalContent) -> SignalId {
        let mut signals = self.signals.borrow_mut();
        signals.insert(signal)
    }
}
