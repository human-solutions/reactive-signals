use std::{any::Any, cell::RefCell, fmt::Debug, ops::Deref};

use crate::{any_func::AnyFunc, signal_id::SignalId};

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct DataSignal(RefCell<Box<dyn Any>>);

impl DataSignal {
    fn new<T: 'static>(value: T) -> Self {
        Self(RefCell::new(Box::new(value)))
    }

    pub fn cloned<T: 'static + Clone>(&self) -> T {
        let value = self.0.borrow();

        value.downcast_ref::<T>().unwrap().clone()
    }
}

impl Deref for DataSignal {
    type Target = RefCell<Box<dyn Any>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct FuncSignal {
    value: DataSignal,
    func: AnyFunc,
}

impl FuncSignal {
    fn new<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        Self {
            value: DataSignal::new(func()),
            func: AnyFunc::new(func),
        }
    }

    pub(crate) fn run(&self) {
        let mut value = self.value.borrow_mut();
        *value = self.func.run_any();
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub enum SignalValue {
    Data(DataSignal),
    Func(FuncSignal),
    Reuse,
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct SignalInner {
    pub(crate) value: SignalValue,
    pub(crate) listeners: Vec<SignalId>,
}

impl SignalInner {
    pub(crate) fn new_data<T: 'static>(value: T) -> Self {
        Self {
            value: SignalValue::Data(DataSignal::new(value)),
            listeners: Vec::default(),
        }
    }

    pub(crate) fn new_func<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        Self {
            value: SignalValue::Func(FuncSignal::new(func)),
            listeners: Vec::default(),
        }
    }

    fn value(&self) -> &DataSignal {
        match self.value {
            SignalValue::Data(ref value) | SignalValue::Func(FuncSignal { ref value, .. }) => value,
            SignalValue::Reuse => panic!("BUG: using a reused signal"),
        }
    }

    pub fn get<T: 'static + Clone>(&self) -> T {
        self.value().cloned()
    }

    pub(crate) fn set<T: 'static>(&self, new_value: T) {
        let mut val = self.value().borrow_mut();
        *val = Box::new(new_value);
    }

    pub(crate) fn reuse(&mut self) {
        self.listeners.clear();
        if cfg!(debug_assertions) {
            self.value = SignalValue::Reuse;
        }
    }
}
