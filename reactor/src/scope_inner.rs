use std::cell::{Ref, RefCell};

use arena_link_tree::NodeBitVec;

use crate::{runtimes::Runtime, scope::Scope, signal::SignalId, signal::SignalInner};

#[derive(Debug, Default)]
pub(crate) struct ScopeInner<RT: Runtime> {
    signals: RefCell<Vec<SignalInner<RT>>>,
}

impl<RT: Runtime> ScopeInner<RT> {
    /// **Warning!**
    ///
    /// This signal id is not yet valid. There has to be a subsequent
    /// call to `insert_signal` before it is valid
    pub fn next_signal_id(&self, sx: Scope<RT>) -> SignalId<RT> {
        let idx = self.signals.borrow().len();
        SignalId::new(idx, sx)
    }

    pub fn insert_signal(&self, signal: SignalInner<RT>) {
        self.signals.borrow_mut().push(signal);
    }

    pub fn with_signal<F, T>(&self, id: SignalId<RT>, f: F) -> T
    where
        F: FnOnce(&SignalInner<RT>) -> T,
    {
        let signals = self.signals.borrow();
        let signal = signals.get(id.index()).unwrap();
        f(&signal)
    }

    pub(crate) fn remove_scopes(&mut self, discarded_scopes: &NodeBitVec) {
        let mut signals = self.signals.borrow_mut();
        signals
            .iter_mut()
            .for_each(|signal| signal.listeners.retain(|s| !discarded_scopes[s.sx]));
    }

    pub(crate) fn reuse(&self) {
        let mut signals = self.signals.borrow_mut();
        signals.iter_mut().for_each(|signal| signal.reuse());
        signals.clear();
    }

    pub(crate) fn vec_ref(&self) -> Ref<Vec<SignalInner<RT>>> {
        self.signals.borrow()
    }
}
