use std::any::Any;

use super::{AnyData, EqFunc, Func, SignalType};

#[cfg(not(feature = "unsafe-cell"))]
type BoxAnyData = Box<std::cell::RefCell<dyn Any>>;
#[cfg(feature = "unsafe-cell")]
type BoxAnyData = Box<std::cell::UnsafeCell<dyn Any>>;

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
    pub fn new<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let val = AnyData::new(Func(func()));
        let func = Box::new(move |val: &BoxAnyData| {
            let new = func();
            #[cfg(not(feature = "unsafe-cell"))]
            {
                let mut old_any = val.borrow_mut();
                let old: &mut T = old_any.downcast_mut::<Func<T>>().unwrap();
                *old = new;
            }
            #[cfg(feature = "unsafe-cell")]
            unsafe {
                let old_any: &mut dyn Any = &mut *val.get();
                let old: &mut T = old_any.downcast_mut::<Func<T>>().unwrap().inner_mut();
                *old = new;
            }
            true
        });
        Self { func, value: val }
    }

    pub fn new_eq<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let val = AnyData::new(EqFunc(func()));
        let func = Box::new(move |val: &BoxAnyData| {
            let new = func();
            #[cfg(not(feature = "unsafe-cell"))]
            {
                let mut old_any = val.borrow_mut();
                let old: &mut T = old_any.downcast_mut::<EqFunc<T>>().unwrap();

                let changed = new != *old;
                *old = new;
                changed
            }
            #[cfg(feature = "unsafe-cell")]
            unsafe {
                let old_any: &mut dyn Any = &mut *val.get();
                let old: &mut T = old_any.downcast_mut::<EqFunc<T>>().unwrap().inner_mut();
                let changed = new != *old;
                *old = new;
                changed
            }
        });
        Self { func, value: val }
    }
    pub fn run(&self) -> bool {
        (self.func)(&self.value.0)
    }
}

#[test]
fn test_usize() {
    let num_fn = DynFunc::new_eq(|| 42usize);
    assert_eq!(num_fn.run(), false);
    assert_eq!(num_fn.value.get::<EqFunc<usize>>(), 42);

    // no equality checking
    let num_fn = DynFunc::new(|| 42usize);
    assert_eq!(num_fn.run(), true);
    assert_eq!(num_fn.value.get::<Func<usize>>(), 42);
}

#[test]
fn test_string() {
    let string_fn = DynFunc::new_eq(|| "hello".to_string());
    assert_eq!(
        string_fn.value.cloned::<EqFunc<String>>(),
        "hello".to_string()
    );

    use std::cell::RefCell;
    use std::rc::Rc;

    let input = Rc::new(RefCell::new(1));

    let input_cp = input.clone();
    let dyn_fn = DynFunc::new_eq(move || {
        let val = input_cp.borrow();
        format!("Val: {}", val)
    });

    assert_eq!(dyn_fn.run(), false);

    assert_eq!(
        dyn_fn.value.cloned::<EqFunc<String>>(),
        "Val: 1".to_string()
    );
    {
        let mut val = input.borrow_mut();
        *val = 2;
    }
    assert_eq!(dyn_fn.run(), true);
    assert_eq!(dyn_fn.run(), false);

    assert_eq!(
        dyn_fn.value.cloned::<EqFunc<String>>(),
        "Val: 2".to_string()
    );
}
