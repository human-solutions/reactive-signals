use super::{Readable, SignalType};

/// A function that produces a value that doesn't implement [PartialEq]
pub struct Func<T>(pub(crate) T);

impl<T> Readable for Func<T> {}

impl<T: 'static> SignalType for Func<T> {
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

/// A function that produces a value that implements [PartialEq]
pub struct EqFunc<T>(pub(crate) T);

impl<T> Readable for EqFunc<T> {}

impl<T: 'static + PartialEq> SignalType for EqFunc<T> {
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
