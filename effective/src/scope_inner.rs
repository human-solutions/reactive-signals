#![allow(dead_code)]
use std::cell::RefCell;

use crate::{scope::Scope, signal_id::SignalId, signal_inner::SignalInner};

#[derive(Debug, Default)]
pub(crate) struct ScopeInner {
    pub(crate) signals: RefCell<Vec<SignalInner>>,
}

impl ScopeInner {
    pub fn insert_signal(&self, sx: Scope, signal: SignalInner) -> SignalId {
        let mut signals = self.signals.borrow_mut();
        let idx = signals.len();
        signals.push(signal);
        SignalId::new(idx, sx)
    }

    pub fn with_signal<F, T>(&self, id: SignalId, f: F) -> T
    where
        F: FnOnce(&SignalInner) -> T,
    {
        let signals = self.signals.borrow();
        let signal = signals.get(id.index()).unwrap();
        f(&signal)
    }

    pub fn with_signal_mut<F, T>(&self, id: SignalId, f: F) -> T
    where
        F: FnOnce(&mut SignalInner) -> T,
    {
        let mut signals = self.signals.borrow_mut();
        let mut signal = signals.get_mut(id.index()).unwrap();
        f(&mut signal)
    }

    pub(crate) fn new_child_scope(&self) -> ScopeInner {
        ScopeInner::default()
    }

    pub(crate) fn discard(&self) {
        todo!()
        // self.signals
        //     .borrow_mut()
        //     .iter()
        //     .for_each(|signal| signal.discard(&self));
    }
}
