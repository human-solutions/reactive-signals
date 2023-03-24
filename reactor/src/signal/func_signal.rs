use super::SignalType;

pub trait FuncSignal {}

pub struct Func<T>(pub(crate) T);

impl<T> FuncSignal for Func<T> {}

impl<T> SignalType for Func<T> {
    type Inner = T;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

pub struct EqFunc<T>(pub(crate) T);

impl<T> FuncSignal for EqFunc<T> {}

impl<T: PartialEq> SignalType for EqFunc<T> {
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
}
