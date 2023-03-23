use std::{any::Any, cell::UnsafeCell};

use super::Compare;

pub struct AnyData(pub(crate) Box<UnsafeCell<dyn Any>>);

impl AnyData {
    pub fn new<T: 'static>(val: T) -> Self {
        Self(Box::new(UnsafeCell::new(val)))
    }
}

impl std::fmt::Debug for AnyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefData")
    }
}

impl AnyData {
    pub fn with<T, R>(&self, f: impl Fn(&T::Inner) -> R) -> R
    where
        T: Compare + 'static,
    {
        unsafe {
            let val_any: &dyn Any = &*self.0.get();
            let val = (*val_any).downcast_ref::<T>().unwrap();
            f(val.inner())
        }
    }

    pub fn update<T, R>(&self, f: impl Fn(&mut T::Inner) -> R) -> R
    where
        T: Compare + 'static,
    {
        unsafe {
            let val_any: &mut dyn Any = &mut *self.0.get();
            let val = (*val_any).downcast_mut::<T>().unwrap();
            f(val.inner_mut())
        }
    }

    pub fn cloned<T>(&self) -> T::Inner
    where
        T: Compare + 'static,
        T::Inner: Clone,
    {
        unsafe {
            let val_any: &dyn Any = &*self.0.get();
            let val = (*val_any).downcast_ref::<T>().unwrap();
            val.inner().clone()
        }
    }

    pub fn get<T>(&self) -> T::Inner
    where
        T: Compare + 'static,
        T::Inner: Copy,
    {
        unsafe {
            let val_any: &dyn Any = &*self.0.get();
            let val = (*val_any).downcast_ref::<T>().unwrap();
            *val.inner()
        }
    }

    pub fn set<T: Compare + 'static>(&self, val: T::Inner) -> bool {
        unsafe {
            let val_any: &mut dyn Any = &mut *self.0.get();
            let val_t = (*val_any).downcast_mut::<T>().unwrap();
            let eq = val_t.is_eq(&val);
            (*val_t).set(val);
            eq
        }
    }
}
