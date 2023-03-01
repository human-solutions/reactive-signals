use std::{any::Any, cell::RefCell, fmt::Debug, marker::PhantomData, ops::Deref, slice};

use slotmap::new_key_type;

use crate::{any_func::AnyFunc, runtime::Runtime, RuntimeId};

new_key_type! { pub struct SignalId; }

#[derive(Default)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct SignalListeners(Vec<SignalId>);

impl SignalListeners {
    pub fn notify_all(&self, rt: &Runtime) {
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
pub enum SignalContent {
    Data(DataSignal, SignalListeners),
    Func(FuncSignal, SignalListeners),
}

impl SignalContent {
    pub fn new_data<T: 'static>(value: T) -> Self {
        Self::Data(DataSignal::new(value), SignalListeners::default())
    }

    pub fn new_func<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        Self::Func(FuncSignal::new(func), SignalListeners::default())
    }

    pub fn listeners_iter(&self) -> slice::Iter<SignalId> {
        self.listeners().iter()
    }

    fn listeners(&self) -> &[SignalId] {
        match self {
            Self::Data(_, listeners) | Self::Func(_, listeners) => &listeners.0,
        }
    }

    fn listeners_mut(&mut self) -> &mut Vec<SignalId> {
        match self {
            Self::Data(_, ref mut listeners) | Self::Func(_, ref mut listeners) => &mut listeners.0,
        }
    }

    fn value(&self) -> &DataSignal {
        match self {
            Self::Data(value, _) | Self::Func(FuncSignal { value, .. }, _) => value,
        }
    }

    pub fn add_listener(&mut self, listener: SignalId) {
        self.listeners_mut().push(listener);
    }

    pub fn get<T: 'static + Clone>(&self) -> T {
        self.value().cloned()
    }

    pub fn set<T: 'static>(&self, rt: &Runtime, new_value: T) {
        {
            let mut val = self.value().borrow_mut();
            *val = Box::new(new_value);
        }
        self.update(rt);
    }

    pub fn update(&self, rt: &Runtime) {
        let list = match self {
            Self::Data(_, list) => list,
            Self::Func(fun, list) => {
                fun.update();
                list
            }
        };
        list.notify_all(rt);
    }
}

pub struct Signal<T> {
    rt: RuntimeId,
    id: SignalId,
    ty: PhantomData<T>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            rt: self.rt,
            id: self.id,
            ty: self.ty,
        }
    }
}
impl<T> Copy for Signal<T> {}

impl<T> Signal<T>
where
    T: Clone + Debug + 'static,
{
    pub fn subscribe<S>(&self, sig: Signal<S>) {
        self.rt
            .with_signal_mut(sig.id, |_, signal| signal.add_listener(self.id));
    }

    pub fn get(&self) -> T {
        self.rt.with_signal(self.id, |_, sig| sig.get())
    }

    pub fn set(&self, val: T) {
        self.rt.with_signal(self.id, |rt, sig| sig.set(rt, val))
    }
}

pub fn create_data_signal<T: 'static>(rt: RuntimeId, value: T) -> Signal<T> {
    Signal {
        rt,
        id: rt.insert_signal(SignalContent::new_data(value)),
        ty: PhantomData,
    }
}

pub fn create_func_signal<F, T>(rt: RuntimeId, func: F) -> Signal<T>
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    Signal {
        rt,
        id: rt.insert_signal(SignalContent::new_func(func)),
        ty: PhantomData,
    }
}
