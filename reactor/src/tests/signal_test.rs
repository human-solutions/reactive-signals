use std::{cell::Cell, rc::Rc};

use crate::{
    create_data_signal, create_func_signal, create_func_signal_eq, scope::Scope, tests::StringStore,
};

#[test]
fn test_signal_dep() {
    let cx = Scope::new_root();

    let num_sig = create_data_signal(cx, 5);

    let output = Rc::new(StringStore::new());
    let out = output.clone();

    let _str_sig = create_func_signal(cx, move || out.push(format!("val: {}", num_sig.get())));

    assert_eq!(output.values(), "val: 5");
    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[test]
fn test_signal_func_val() {
    let cx = Scope::new_root();

    let num_sig = create_data_signal(cx, 5);

    let output = Rc::new(StringStore::new());

    let a_sig = create_func_signal(cx, move || format!("a{}", num_sig.get()));
    let b_sig = create_func_signal(cx, move || format!("b{}", num_sig.get()));

    let out = output.clone();
    let _str_sig = create_func_signal(cx, move || {
        out.push(format!("{}-{}", a_sig.get(), b_sig.get()))
    });

    num_sig.set(4);

    assert_eq!(output.values(), "a5-b5, a4-b5, a4-b4");
}

#[test]
fn test_signal_func_skip_equal() {
    let cx = Scope::new_root();

    let num_sig = create_data_signal(cx, 10);

    let a_call = Rc::new(Cell::new(0usize));
    let a_sig = {
        let a_call = a_call.clone();
        create_func_signal_eq(cx, move || {
            a_call.inc();
            num_sig.get() + 1
        })
    };

    let b_call = Rc::new(Cell::new(0usize));
    let b_sig = {
        let b_call = b_call.clone();
        create_func_signal_eq(cx, move || {
            b_call.inc();
            100
        })
    };

    let c_call = Rc::new(Cell::new(0usize));
    let c_sig = {
        let c_call = c_call.clone();
        create_func_signal_eq(cx, move || {
            c_call.inc();
            b_sig.get() + 1
        })
    };

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
