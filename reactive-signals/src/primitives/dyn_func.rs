use std::any::Any;

use crate::{signals::SignalType, CellType};

use super::any_data::AnyData;

type BoxAnyData = Box<CellType<dyn Any>>;

pub struct DynFunc {
    pub(crate) func: Box<dyn Fn(&BoxAnyData) -> bool>,
    pub(crate) value: AnyData,
}

impl std::fmt::Debug for DynFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefFunc")
    }
}

impl DynFunc {
    pub fn new<F, T, W: SignalType<Inner = T>>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let val = AnyData::new(W::new(func()));
        let func = Box::new(move |val: &BoxAnyData| {
            let new = func();

            #[cfg(not(feature = "unsafe-cell"))]
            let mut old_any = val.borrow_mut();
            #[cfg(feature = "unsafe-cell")]
            let old_any: &mut dyn Any = unsafe { &mut *val.get() };

            let old: &mut T = old_any.downcast_mut::<W>().unwrap().inner_mut();
            *old = new;
            true
        });
        Self { func, value: val }
    }

    pub fn run(&self) -> bool {
        (self.func)(&self.value.0)
    }
}
