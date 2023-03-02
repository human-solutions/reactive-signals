use std::cell::{Cell, RefCell};

use slotmap::SlotMap;

use crate::{
    scope::{Scope, ScopeId},
    signal::SignalId,
    signal_inner::SignalInner,
    Runtime,
};

thread_local! {
  pub(crate) static RUNTIMES: RuntimePool = Default::default();
}

#[derive(Default)]
pub(crate) struct RuntimePool(pub(crate) RefCell<Vec<RuntimeInner>>);

impl RuntimePool {
    pub(crate) fn borrow(&self) -> Runtime {
        {
            for (idx, rt) in self.0.borrow().iter().enumerate() {
                if !rt.in_use.get() {
                    rt.in_use.set(true);
                    return Runtime::from(idx);
                }
            }
        }
        let mut vec = self.0.borrow_mut();
        let id = Runtime::from(vec.len());
        vec.push(RuntimeInner::new(id));
        id
    }

    pub(crate) fn return_to_pool(&self, runtime: &Runtime) {
        let mut pool = self.0.borrow_mut();
        let rt = &mut pool[runtime.0 as usize];
        rt.discard();
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub(crate) struct RuntimeInner {
    pub(crate) id: Runtime,
    in_use: Cell<bool>,
    scope_counter: Cell<u32>,
    pub(crate) signals: RefCell<SlotMap<SignalId, SignalInner>>,
}

impl RuntimeInner {
    // pub fn with<F, T>(rt: Runtime, f: F) -> T
    // where
    //     F: FnOnce(&RuntimeInner) -> T,
    // {
    //     RUNTIMES.with(|pool| {
    //         let rt = &pool.0.borrow()[rt.0 as usize];
    //         f(&rt)
    //     })
    // }

    fn new(id: Runtime) -> Self {
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
