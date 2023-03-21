use std::{any::Any, cell::RefCell};

pub struct AnyData(pub(crate) Box<RefCell<dyn Any>>);

impl AnyData {
    pub fn new<T: 'static>(val: T) -> Self {
        Self(Box::new(RefCell::new(val)))
    }
}

impl std::fmt::Debug for AnyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefData")
    }
}

impl AnyData {
    pub fn with<T: 'static, R>(&self, f: impl Fn(&T) -> R) -> R {
        let val_any = self.0.borrow();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        f(val)
    }

    pub fn update<T: 'static, R>(&self, f: impl Fn(&mut T) -> R) -> R {
        let mut val_any = self.0.borrow_mut();
        let val = (*val_any).downcast_mut::<T>().unwrap();
        f(val)
    }

    pub fn cloned<T: Clone + 'static>(&self) -> T {
        let val_any = self.0.borrow();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        val.clone()
    }

    pub fn get<T: Copy + 'static>(&self) -> T {
        let val_any = self.0.borrow();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        *val
    }

    pub fn set<T: 'static>(&self, val: T) {
        let mut val_any = self.0.borrow_mut();
        let val_t = (*val_any).downcast_mut::<T>().unwrap();
        *val_t = val
    }
}
