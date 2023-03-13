#![allow(dead_code)]

use std::{any::Any, cell::RefCell};

pub struct RefFunc {
    func: Box<dyn Fn(&Box<RefCell<dyn Any>>) -> bool>,
    val: Box<RefCell<dyn Any>>,
}

impl RefFunc {
    pub fn new<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let val = Box::new(RefCell::new(func()));
        let func = Box::new(move |val: &Box<RefCell<dyn Any>>| {
            let new = func();
            let mut old_any = val.borrow_mut();
            let old: &mut T = old_any.downcast_mut::<T>().unwrap();
            *old = new;
            true
        });
        Self { func, val }
    }

    pub fn new_eq<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let val = Box::new(RefCell::new(func()));
        let func = Box::new(move |val: &Box<RefCell<dyn Any>>| {
            let new = func();
            let mut old_any = val.borrow_mut();
            let old: &mut T = old_any.downcast_mut::<T>().unwrap();
            let changed = new != *old;
            *old = new;
            changed
        });
        Self { func, val }
    }
    pub fn run(&self) -> bool {
        (self.func)(&self.val)
    }

    pub fn with<T: 'static>(&self, f: impl Fn(&T)) {
        let val_any = self.val.borrow();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        f(val)
    }

    pub fn with_mut<T: 'static>(&self, f: impl Fn(&mut T)) {
        let mut val_any = self.val.borrow_mut();
        let val = (*val_any).downcast_mut::<T>().unwrap();
        f(val)
    }

    pub fn cloned<T: Clone + 'static>(&self) -> T {
        let val_any = self.val.borrow();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        val.clone()
    }

    pub fn get<T: Copy + 'static>(&self) -> T {
        let val_any = self.val.borrow();
        let val = (*val_any).downcast_ref::<T>().unwrap();
        *val
    }

    pub fn set<T: Copy + 'static>(&self, val: T) {
        let mut val_any = self.val.borrow_mut();
        let val_t = (*val_any).downcast_mut::<T>().unwrap();
        *val_t = val
    }

    pub fn update<T: Copy + 'static>(&self, f: impl Fn(T) -> T) {
        let mut val_any = self.val.borrow_mut();
        let val_t = (*val_any).downcast_mut::<T>().unwrap();
        *val_t = f(*val_t)
    }
}

#[test]
fn test_usize() {
    let num_fn = RefFunc::new_eq(|| 42usize);
    assert_eq!(num_fn.run(), false);
    assert_eq!(num_fn.run(), false);
    assert_eq!(num_fn.get::<usize>(), 42);
}

#[test]
fn test_string() {
    let string_fn = RefFunc::new_eq(|| "hello".to_string());
    assert_eq!(string_fn.cloned::<String>(), "hello".to_string());

    use std::cell::RefCell;
    use std::rc::Rc;

    let input = Rc::new(RefCell::new(1));

    let input_cp = input.clone();
    let dyn_fn = RefFunc::new_eq(move || {
        let val = input_cp.borrow();
        format!("Val: {}", val)
    });

    assert_eq!(dyn_fn.run(), false);

    assert_eq!(dyn_fn.cloned::<String>(), "Val: 1".to_string());
    {
        let mut val = input.borrow_mut();
        *val = 2;
    }
    assert_eq!(dyn_fn.run(), true);
    assert_eq!(dyn_fn.run(), false);

    assert_eq!(dyn_fn.cloned::<String>(), "Val: 2".to_string());
}
