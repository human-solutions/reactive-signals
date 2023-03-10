use std::{any::Any, fmt::Debug, ops::Deref};

use crate::{any_func::AnyFunc, signal_id::SignalId};

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct DataSignal(Box<dyn Any>);

impl DataSignal {
    fn new<T: 'static>(value: T) -> Self {
        Self(Box::new(value))
    }

    pub fn cloned<T: 'static + Clone>(&self) -> T {
        self.0.downcast_ref::<T>().unwrap().clone()
    }
}

impl Deref for DataSignal {
    type Target = Box<dyn Any>;

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

    /// when running a signal, other signals are accessed (.get())
    /// so first we run with a non mutable reference and with a
    /// second step we set the value.
    pub(crate) fn run(&self) -> Box<dyn Any> {
        self.func.run_any()
    }

    pub(crate) fn set_value(&mut self, new_value: Box<dyn Any>) {
        self.value.0 = new_value;
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

    pub(crate) fn set<T: 'static>(&mut self, new_value: T) {
        match self.value {
            SignalValue::Data(ref mut value)
            | SignalValue::Func(FuncSignal { ref mut value, .. }) => {
                *value = DataSignal::new(new_value)
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

#[test]
fn size_of_ref_cell_box() {
    use std::cell::RefCell;
    assert_eq!(std::mem::size_of::<RefCell<Box<usize>>>(), 16);
    assert_eq!(std::mem::size_of::<Box<usize>>(), 8);
}
