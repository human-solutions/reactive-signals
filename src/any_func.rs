use std::any::Any;

pub struct AnyFunc(Box<dyn Fn() -> Box<dyn Any>>);

#[cfg(feature = "extra-traits")]
impl std::fmt::Debug for AnyFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyFunc").finish()
    }
}

impl AnyFunc {
    pub fn new<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let func: Box<dyn Fn() -> Box<dyn Any>> = Box::new(move || Box::new(func()));

        Self(func)
    }
    #[cfg(test)]
    pub fn run<T: 'static>(&self) -> T {
        let val = (self.0)();
        *val.downcast::<T>().unwrap()
    }

    pub fn run_any(&self) -> Box<dyn Any> {
        (self.0)()
    }
}

#[test]
fn test_any_func() {
    let num_fn = AnyFunc::new(|| 42);
    let any_val = num_fn.run_any();
    let num_val = any_val.downcast_ref::<i32>().unwrap();
    assert_eq!(*num_val, 42);
    assert_eq!(num_fn.run::<i32>(), 42);

    let string_fn = AnyFunc::new(|| "hello".to_string());
    assert_eq!(string_fn.run::<String>(), "hello".to_string());

    use std::cell::RefCell;
    use std::rc::Rc;

    let input = Rc::new(RefCell::new(1));

    let input_cp = input.clone();
    let dyn_fn = AnyFunc::new(move || {
        let val = input_cp.borrow();
        format!("Val: {}", val)
    });

    assert_eq!(dyn_fn.run::<String>(), "Val: 1".to_string());
    {
        let mut val = input.borrow_mut();
        *val = 2;
    }

    assert_eq!(dyn_fn.run::<String>(), "Val: 2".to_string());
}
