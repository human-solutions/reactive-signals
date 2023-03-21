use std::cell::UnsafeCell;

use arena_link_tree::NodeBitVec;

use crate::{runtimes::Runtime, scope::Scope, signal::SignalId, signal::SignalInner};

#[derive(Debug, Default)]
pub(crate) struct ScopeInner<RT: Runtime> {
    pub(crate) signals: UnsafeCell<Vec<SignalInner<RT>>>,
}

impl<RT: Runtime> ScopeInner<RT> {
    /// **Warning!**
    ///
    /// This signal id is not yet valid. There has to be a subsequent
    /// call to `insert_signal` before it is valid
    pub fn next_signal_id(&self, sx: Scope<RT>) -> SignalId<RT> {
        unsafe {
            let idx = (&*self.signals.get()).len();
            SignalId::new(idx, sx)
        }
    }

    pub fn insert_signal(&self, signal: SignalInner<RT>) {
        unsafe {
            (&mut *self.signals.get()).push(signal);
        }
    }

    pub fn with_signal<F, T>(&self, id: SignalId<RT>, f: F) -> T
    where
        F: FnOnce(&SignalInner<RT>) -> T,
    {
        unsafe {
            let signals = &*self.signals.get();
            let signal = signals.get(id.index()).unwrap();
            f(&signal)
        }
    }

    pub(crate) fn remove_scopes(&mut self, discarded_scopes: &NodeBitVec) {
        unsafe {
            let signals = &mut *self.signals.get();
            signals
                .iter_mut()
                .for_each(|signal| signal.listeners.retain(|s| !discarded_scopes[s.sx]));
        }
    }

    pub(crate) fn reuse(&self) {
        unsafe {
            let signals = &mut *self.signals.get();
            signals.iter_mut().for_each(|signal| signal.reuse());
            signals.clear();
        }
    }

    pub(crate) fn vec_ref(&self) -> &[SignalInner<RT>] {
        unsafe { &*self.signals.get() }
    }
}
