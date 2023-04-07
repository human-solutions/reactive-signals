use std::any::Any;

use crate::{signals2::SignalType, CellType};

pub struct AnyData(pub(crate) Box<CellType<dyn Any>>);

impl std::fmt::Debug for AnyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefData")
    }
}

impl AnyData {
    pub fn new<T: 'static>(val: T) -> Self {
        Self(Box::new(CellType::new(val)))
    }

    pub fn with<T, R>(&self, f: impl Fn(&T::Inner) -> R) -> R
    where
        T: SignalType + 'static,
    {
        let val_any = self.val_ref();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        f(val.inner())
    }

    pub fn update<T, R>(&self, f: impl Fn(&mut T::Inner) -> R) -> (bool, R)
    where
        T: SignalType + 'static,
    {
        #[allow(unused_mut)]
        let mut val_any = self.val_mut();
        let val = (*val_any).downcast_mut::<T>().unwrap();
        let hash_before = val.opt_hash();
        let r = f(val.inner_mut());
        let hash_after = val.opt_hash();
        let eq = match (hash_before, hash_after) {
            (Some(h1), Some(h2)) => h1 == h2,
            _ => false,
        };
        (eq, r)
    }

    pub fn cloned<T>(&self) -> T::Inner
    where
        T: SignalType + 'static,
        T::Inner: Clone,
    {
        let val_any = self.val_ref();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        val.inner().clone()
    }

    pub fn get<T>(&self) -> T::Inner
    where
        T: SignalType + 'static,
        T::Inner: Copy,
    {
        let val_any = self.val_ref();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        *val.inner()
    }

    pub fn set<T: SignalType + 'static>(&self, val: T::Inner) -> bool {
        #[allow(unused_mut)]
        let mut val_any = self.val_mut();
        let val_t = (*val_any).downcast_mut::<T>().unwrap();
        let eq = val_t.is_eq(&val);
        *val_t.inner_mut() = val;
        eq
    }
}

#[cfg(not(feature = "unsafe-cell"))]
impl AnyData {
    #[inline]
    fn val_ref(&self) -> std::cell::Ref<dyn Any> {
        self.0.borrow()
    }
    #[inline]
    fn val_mut(&self) -> std::cell::RefMut<dyn Any> {
        self.0.borrow_mut()
    }
}

#[cfg(feature = "unsafe-cell")]
impl AnyData {
    #[inline]
    fn val_ref(&self) -> &dyn Any {
        unsafe { &*self.0.get() }
    }

    #[inline]
    fn val_mut(&self) -> &mut dyn Any {
        unsafe { &mut *self.0.get() }
    }
}
