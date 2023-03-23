use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

pub trait Modify {}

pub trait Compare {
    type Inner;
    fn is_eq(&self, other: &Self::Inner) -> bool;
    fn opt_hash(&self) -> Option<u64>;

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;

    fn set(&mut self, val: Self::Inner);
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
    fn set(&mut self, val: Self::Inner) {
        self.0 = val;
    }
}

impl<T> Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Data<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct EqData<T>(pub(crate) T);

impl<T> Modify for EqData<T> {}

impl<T> Deref for EqData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for EqData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

    fn set(&mut self, val: Self::Inner) {
        self.0 = val;
    }
}

pub struct HashEqData<T>(pub(crate) T);

impl<T> Modify for HashEqData<T> {}

impl<T> Deref for HashEqData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for HashEqData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

    fn set(&mut self, val: Self::Inner) {
        self.0 = val;
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

    assert_eq!(set(&d1, &d2), false);
    assert_eq!(set(&d1, &d1), false);

    let d1 = EqData(3);
    let d2 = EqData(2);

    assert_eq!(set(&d1, &d2), false);
    assert_eq!(set(&d1, &d1), true);
}
