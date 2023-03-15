use std::fmt::Debug;

use crate::{
    primitives::{AnyData, DynFunc},
    signal_id::SignalId,
};

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub enum SignalValue {
    Data(AnyData),
    Func(DynFunc),
    Reuse,
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct SignalInner {
    pub(crate) value: SignalValue,
    pub(crate) listeners: Vec<SignalId>,
}

impl SignalInner {
    fn value(&self) -> &AnyData {
        match self.value {
            SignalValue::Data(ref value) | SignalValue::Func(DynFunc { ref value, .. }) => value,
            SignalValue::Reuse => panic!("BUG: using a reused signal"),
        }
    }

    pub fn get<T: 'static + Clone>(&self) -> T {
        self.value().cloned()
    }

    pub(crate) fn set<T: 'static>(&mut self, new_value: T) {
        match self.value {
            SignalValue::Data(ref mut value) | SignalValue::Func(DynFunc { ref mut value, .. }) => {
                *value = AnyData::new(new_value)
            }
            SignalValue::Reuse => panic!("BUG: using a reused signal"),
        }
    }

    pub(crate) fn reuse(&mut self) {
        self.listeners.clear();
        if cfg!(debug_assertions) {
            self.value = SignalValue::Reuse;
        }
    }
}
