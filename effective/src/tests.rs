use std::{cell::RefCell, mem, rc::Rc};

use crate::{create_data_signal, create_func_signal, Runtime, Signal};

struct StringStore(RefCell<Vec<String>>);

impl StringStore {
    fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    fn push(&self, value: String) {
        self.0.borrow_mut().push(value);
    }

    fn values(&self) -> String {
        self.0
            .borrow()
            .iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[test]
fn test_signal_dep() {
    let cx = Runtime::from_pool().create_scope();

    let num_sig = create_data_signal(cx, 5);

    let output = Rc::new(StringStore::new());
    let out = output.clone();

    let str_sig = create_func_signal(cx, move || out.push(format!("val: {}", num_sig.get())));

    str_sig.subscribe(num_sig);

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[test]
fn test_signal_func_val() {
    let cx = Runtime::from_pool().create_scope();

    let num_sig = create_data_signal(cx, 5);

    let output = Rc::new(StringStore::new());

    let a_sig = create_func_signal(cx, move || format!("a{}", num_sig.get()));
    let b_sig = create_func_signal(cx, move || format!("b{}", num_sig.get()));

    a_sig.subscribe(num_sig);
    b_sig.subscribe(num_sig);

    let out = output.clone();
    let str_sig = create_func_signal(cx, move || {
        out.push(format!("{}-{}", a_sig.get(), b_sig.get()))
    });

    str_sig.subscribe(a_sig);
    str_sig.subscribe(b_sig);

    num_sig.set(4);

    assert_eq!(output.values(), "a5-b5, a4-b5, a4-b4");
}

#[test]
fn test_sizes() {
    assert_eq!(12, mem::size_of::<Signal<String>>());
    assert_eq!(12, mem::size_of::<Signal<usize>>());
}
