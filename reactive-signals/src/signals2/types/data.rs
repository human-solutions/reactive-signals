use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{Modifiable, Readable, SignalType};

/// Data that doesn't implement [PartialEq]
pub struct Data<T>(pub(crate) T);

impl<T> Modifiable for Data<T> {}
impl<T> Readable for Data<T> {}

impl<T: 'static> SignalType for Data<T> {
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

/// Data that implements [PartialEq]
pub struct EqData<T>(pub(crate) T);

impl<T> Modifiable for EqData<T> {}
impl<T> Readable for EqData<T> {}

impl<T: 'static + PartialEq> SignalType for EqData<T> {
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

/// Data that implements [PartialEq] and [Hash](std::hash::Hash)
pub struct HashEqData<T>(pub(crate) T);

impl<T> Modifiable for HashEqData<T> {}
impl<T> Readable for HashEqData<T> {}

impl<T: 'static + PartialEq + Hash> SignalType for HashEqData<T> {
    type Inner = T;
    fn is_eq(&self, other: &Self::Inner) -> bool {
        self.0 == *other
    }

    fn opt_hash(&self) -> Option<u64> {
        let mut h = DefaultHasher::new();
        self.0.hash(&mut h);
        Some(h.finish())
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
