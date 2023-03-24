use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::SignalType;

pub trait DataSignal {}

pub struct Data<T>(pub(crate) T);

impl<T> DataSignal for Data<T> {}

impl<T> SignalType for Data<T> {
    type Inner = T;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

pub struct EqData<T>(pub(crate) T);

impl<T> DataSignal for EqData<T> {}

impl<T: PartialEq> SignalType for EqData<T> {
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

pub struct HashEqData<T>(pub(crate) T);

impl<T> DataSignal for HashEqData<T> {}

impl<T: PartialEq + Hash> SignalType for HashEqData<T> {
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
}
