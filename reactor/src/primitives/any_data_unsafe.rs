#![allow(dead_code)]

use std::{
    any::Any,
    cell::{RefCell, UnsafeCell},
};

pub struct AnyData(pub(crate) Box<UnsafeCell<dyn Any>>);

impl AnyData {
    pub fn new<T: 'static>(val: T) -> Self {
        Self(Box::new(UnsafeCell::new(val)))
    }
}

#[cfg(feature = "extra-traits")]
impl std::fmt::Debug for AnyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefData")
    }
}

impl AnyData {
    pub fn with<T: 'static>(&self, f: impl Fn(&T)) {
        unsafe {
            let val_any: &dyn Any = &*self.0.get();
            let val = (*val_any).downcast_ref::<T>().unwrap();
            f(val)
        }
    }

    pub fn with_mut<T: 'static>(&self, f: impl Fn(&mut T)) {
        unsafe {
            let val_any: &mut dyn Any = &mut *self.0.get();
            let val = (*val_any).downcast_mut::<T>().unwrap();
            f(val)
        }
    }

    pub fn cloned<T: Clone + 'static>(&self) -> T {
        unsafe {
            let val_any: &dyn Any = &*self.0.get();
            let val = (*val_any).downcast_ref::<T>().unwrap();
            val.clone()
        }
    }

    pub fn get<T: Copy + 'static>(&self) -> T {
        unsafe {
            let val_any: &dyn Any = &*self.0.get();
            let val = (*val_any).downcast_ref::<T>().unwrap();
            *val
        }
    }

    pub fn set<T: Copy + 'static>(&self, val: T) {
        unsafe {
            let val_any: &mut dyn Any = &mut *self.0.get();
            let val_t = (*val_any).downcast_mut::<T>().unwrap();
            *val_t = val
        }
    }

    pub fn update<T: Copy + 'static>(&self, f: impl Fn(T) -> T) {
        unsafe {
            let val_any: &mut dyn Any = &mut *self.0.get();
            let val_t = (*val_any).downcast_mut::<T>().unwrap();
            *val_t = f(*val_t)
        }
    }
}
