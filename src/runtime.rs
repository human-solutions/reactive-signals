#![allow(dead_code)]

use std::cell::RefCell;

use signal::{SignalContent, SignalId};

use crate::signal;

thread_local! {
    pub(crate) static RUNTIME: Runtime = Runtime::new();
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Runtime {
    pub(crate) signals: RefCell<slotmap::SlotMap<SignalId, SignalContent>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            signals: RefCell::new(slotmap::SlotMap::with_key()),
        }
    }

    pub fn insert_signal(&self, signal: SignalContent) -> SignalId {
        let mut signals = self.signals.borrow_mut();
        signals.insert(signal)
    }
}
