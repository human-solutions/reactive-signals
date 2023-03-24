use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub trait DataSignal {}

pub trait FuncSignal {}

pub trait SignalType {
    type Inner;
    fn is_eq(&self, _other: &Self::Inner) -> bool {
        false
    }
    fn opt_hash(&self) -> Option<u64> {
        None
    }

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
}

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

#[cfg(test)]
fn set<T: 'static + SignalType>(val1: &T, val2: &T::Inner) -> bool {
    val1.is_eq(&val2)
}

#[test]
fn cmp_test() {
    let d1 = Data(3);
    let d2 = Data(2);

    assert_eq!(set(&d1, &d2.inner()), false);
    assert_eq!(set(&d1, &d1.inner()), false);

    let d1 = EqData(3);
    let d2 = EqData(2);

    assert_eq!(set(&d1, &d2.inner()), false);
    assert_eq!(set(&d1, &d1.inner()), true);
}
