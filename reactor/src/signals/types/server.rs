use super::{OptReadable, SignalType};

pub struct ServerFunc<T>(pub(crate) T);

impl<T> OptReadable for ServerFunc<T> {
    const RUN_ON_CLIENT: bool = false;
}

impl<T: 'static> SignalType for ServerFunc<T> {
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

pub struct ServerEqFunc<T>(pub(crate) T);

impl<T> OptReadable for ServerEqFunc<T> {}

impl<T: 'static + PartialEq> SignalType for ServerEqFunc<T> {
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
