use std::{any::Any, cell::RefCell, fmt::Debug, ops::Deref, slice};

use crate::{any_func::AnyFunc, runtime_inner::RuntimeInner, scope::ScopeId, signal::SignalId};

#[derive(Default)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct SignalListeners(Vec<SignalId>);

impl SignalListeners {
    pub(crate) fn notify_all(&self, rt: &RuntimeInner) {
        for listener in self.0.iter() {
            {
                let sig = rt.signals.borrow();
                let signal = sig.get(*listener).unwrap();
                signal.update(rt);
            }
        }
    }
}

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

    fn update(&self) {
        let mut value = self.value.borrow_mut();
        *value = self.func.run_any();
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub enum SignalInner {
    Data {
        id: SignalId,
        scope: ScopeId,
        value: DataSignal,
        listeners: SignalListeners,
    },
    Func {
        id: SignalId,
        scope: ScopeId,
        value: FuncSignal,
        listeners: SignalListeners,
    },
}

impl SignalInner {
    pub fn new_data<T: 'static>(scope: ScopeId, value: T) -> Self {
        Self::Data {
            id: SignalId::default(),
            scope,
            value: DataSignal::new(value),
            listeners: SignalListeners::default(),
        }
    }

    pub fn new_func<F, T>(scope: ScopeId, func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        Self::Func {
            id: SignalId::default(),
            scope,
            value: FuncSignal::new(func),
            listeners: SignalListeners::default(),
        }
    }

    pub fn listeners_iter(&self) -> slice::Iter<SignalId> {
        self.listeners().iter()
    }

    pub fn scope(&self) -> ScopeId {
        match self {
            Self::Data { scope, .. } | Self::Func { scope, .. } => *scope,
        }
    }

    pub fn id(&self) -> SignalId {
        match self {
            Self::Data { id, .. } | Self::Func { id, .. } => *id,
        }
    }
    pub fn set_id(&mut self, id: SignalId) {
        match self {
            Self::Data { id: id_, .. } | Self::Func { id: id_, .. } => *id_ = id,
        }
    }

    fn listeners(&self) -> &[SignalId] {
        match self {
            Self::Data { listeners, .. } | Self::Func { listeners, .. } => &listeners.0,
        }
    }

    fn listeners_mut(&mut self) -> &mut Vec<SignalId> {
        match self {
            Self::Data {
                ref mut listeners, ..
            }
            | Self::Func {
                ref mut listeners, ..
            } => &mut listeners.0,
        }
    }

    fn value(&self) -> &DataSignal {
        match self {
            Self::Data { value, .. }
            | Self::Func {
                value: FuncSignal { value, .. },
                ..
            } => value,
        }
    }

    pub fn add_listener(&mut self, listener: SignalId) {
        self.listeners_mut().push(listener);
    }

    pub fn remove_listener(&mut self, listener: SignalId) {
        let listeners = self.listeners_mut();
        let index = listeners.iter().position(|&x| x == listener).unwrap();
        listeners.swap_remove(index);
    }

    pub fn get<T: 'static + Clone>(&self) -> T {
        self.value().cloned()
    }

    pub(crate) fn set<T: 'static>(&self, rt: &RuntimeInner, new_value: T) {
        {
            let mut val = self.value().borrow_mut();
            *val = Box::new(new_value);
        }
        self.update(rt);
    }

    pub(crate) fn update(&self, rt: &RuntimeInner) {
        let list = match self {
            Self::Data { listeners, .. } => listeners,
            Self::Func {
                value, listeners, ..
            } => {
                value.update();
                listeners
            }
        };
        list.notify_all(rt);
    }

    pub(crate) fn discard(&self, rt: &RuntimeInner) {
        let mut sig = rt.signals.borrow_mut();
        for listener in self.listeners_iter() {
            let listener = sig.get_mut(*listener).unwrap();
            listener.remove_listener(self.id());
        }
    }
}
