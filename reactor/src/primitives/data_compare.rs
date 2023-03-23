use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub trait Modify {}

pub trait Compare {
    type Inner;
    fn is_eq(&self, other: &Self::Inner) -> bool;
    fn opt_hash(&self) -> Option<u64>;

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
}

pub struct Data<T>(pub(crate) T);

impl<T> Modify for Data<T> {}

impl<T> Compare for Data<T> {
    type Inner = T;
    fn is_eq(&self, _: &Self::Inner) -> bool {
        false
    }
    fn opt_hash(&self) -> Option<u64> {
        None
    }
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

pub struct EqData<T>(pub(crate) T);

impl<T> Modify for EqData<T> {}

impl<T: PartialEq> Compare for EqData<T> {
    type Inner = T;
    fn is_eq(&self, other: &Self::Inner) -> bool {
        self.0 == *other
    }
    fn opt_hash(&self) -> Option<u64> {
        None
    }
    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

pub struct HashEqData<T>(pub(crate) T);

impl<T> Modify for HashEqData<T> {}

impl<T: PartialEq + Hash> Compare for HashEqData<T> {
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
fn set<T: 'static + Compare>(val1: &T, val2: &T::Inner) -> bool {
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
