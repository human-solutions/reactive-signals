#![allow(dead_code)]
use std::cell::RefCell;

use crate::signal_inner::SignalInner;

#[derive(Default)]
pub(crate) struct ScopeInner {
    signals: RefCell<Vec<SignalInner>>,
}

impl ScopeInner {
    pub fn insert_signal(&self, signal: SignalInner) -> usize {
        let mut signals = self.signals.borrow_mut();
        let idx = signals.len();
        signals.push(signal);
        idx
    }

    pub fn with_signal<F, T>(&self, id: usize, f: F) -> T
    where
        F: FnOnce(&SignalInner) -> T,
    {
        let signals = self.signals.borrow();
        let signal = signals.get(id).unwrap();
        f(&signal)
    }

    pub fn with_signal_mut<F, T>(&self, id: usize, f: F) -> T
    where
        F: FnOnce(&mut SignalInner) -> T,
    {
        let mut signals = self.signals.borrow_mut();
        let mut signal = signals.get_mut(id).unwrap();
        f(&mut signal)
    }
}
