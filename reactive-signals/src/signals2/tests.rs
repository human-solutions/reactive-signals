use std::{cell::Cell, rc::Rc};

use crate::{runtimes2::ServerRuntime, signal, tests::StringStore};

#[test]
fn test_signal_dep() {
    let sc = ServerRuntime::new_root_scope();

    let num_sig = signal!(sc, 5);
    assert_eq!(num_sig.get(), 5);

    let output = Rc::new(StringStore::new());
    let out = output.clone();

    let _str_sig = signal!(sc, move || out.push(format!("val: {}", num_sig.get())));

    assert_eq!(output.values(), "val: 5");
    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[test]
fn test_signal_update() {
    use std::cell::RefCell;

    let sc = ServerRuntime::new_root_scope();

    let history = Rc::new(RefCell::new(Vec::<String>::new()));

    let string_sig = signal!(sc, "Hi 1".to_string());
    assert_eq!(string_sig.cloned(), "Hi 1".to_string());

    signal!(sc, clone: history, move || history
        .borrow_mut()
        .push(string_sig.cloned()));

    assert_eq!(history.borrow().join(", "), "Hi 1");

    // no change
    string_sig.set("Hi 1".to_string());
    assert_eq!(history.borrow().join(", "), "Hi 1");

    // change
    string_sig.set("Hi 2".to_string());
    assert_eq!(history.borrow().join(", "), "Hi 1, Hi 2");
}

#[test]
fn test_signal_func_val() {
    let sc = ServerRuntime::new_root_scope();

    let num_sig = signal!(sc, 5);

    let output = Rc::new(StringStore::new());

    let a_sig = signal!(sc, move || format!("a{}", num_sig.get()));
    let b_sig = signal!(sc, move || format!("b{}", num_sig.get()));

    let _str_sig = signal!(sc, clone: output, move || {
        output.push(format!("{}-{}", a_sig.cloned(), b_sig.cloned()))
    });

    num_sig.set(4);

    assert_eq!(output.values(), "a5-b5, a4-b5, a4-b4");
}

#[test]
fn test_signal_func_skip_equal() {
    let sc = ServerRuntime::new_root_scope();

    let num_sig = signal!(sc, 10);

    let a_call = Rc::new(Cell::new(0usize));
    let a_sig = signal!(sc, clone: a_call, move || {
        a_call.inc();
        num_sig.get() + 1
    });

    let b_call = Rc::new(Cell::new(0usize));
    let b_sig = signal!(sc, clone: b_call, move || {
        b_call.inc();
        100
    });

    let c_call = Rc::new(Cell::new(0usize));
    let c_sig = signal!(sc, clone: c_call, move || {
        c_call.inc();
        b_sig.get() + 1
    });

    assert_eq!(a_sig.get(), 11);
    assert_eq!(b_sig.get(), 100);
    assert_eq!(c_sig.get(), 101);

    assert_eq!(a_call.get(), 1);
    assert_eq!(b_call.get(), 1);
    assert_eq!(c_call.get(), 1);

    num_sig.set(20);

    assert_eq!(a_sig.get(), 21);
    assert_eq!(b_sig.get(), 100);
    assert_eq!(c_sig.get(), 101);

    assert_eq!(a_call.get(), 2);
    assert_eq!(b_call.get(), 1);
    assert_eq!(c_call.get(), 1);
}

trait CellIncr {
    fn inc(&self);
}

impl CellIncr for Cell<usize> {
    fn inc(&self) {
        let val = self.get();
        self.set(val + 1)
    }
}
