use super::{OptReadable, SignalType};

pub struct ClientFunc<T>(pub(crate) T);

impl<T> OptReadable for ClientFunc<T> {
    const RUN_ON_SERVER: bool = false;
}

impl<T: 'static> SignalType for ClientFunc<T> {
    type Inner = T;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
    fn new(value: Self::Inner) -> Self {
        Self(value)
    }
}

pub struct ClientEqFunc<T>(pub(crate) T);

impl<T> OptReadable for ClientEqFunc<T> {}

impl<T: 'static + PartialEq> SignalType for ClientEqFunc<T> {
    type Inner = T;

    fn is_eq(&self, other: &Self::Inner) -> bool {
        self.0 == *other
    }

    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
    fn new(value: Self::Inner) -> Self {
        Self(value)
    }
}
