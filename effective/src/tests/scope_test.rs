use std::rc::Rc;

use crate::{create_data_signal, create_func_signal, scope::Scope, tests::StringStore};

#[test]
fn test_scopes_deep() {
    let root = Scope::create_root_scope();

    let mut cx = root.clone();
    let num_sig = create_data_signal(cx, 5);

    (0..3).for_each(|_| cx = cx.new_child_scope());

    let output = Rc::new(StringStore::new());
    let out = output.clone();
    let str_sig = create_func_signal(cx, move || out.push(format!("val: {}", num_sig.get())));

    str_sig.subscribe(num_sig);

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[test]
fn test_scopes_discard() {
    let root = Scope::create_root_scope();

    let cx0 = root.clone();
    let num_sig = create_data_signal(cx0, 5);

    let cx1 = cx0.new_child_scope();
    let cx2 = cx1.new_child_scope();
    let cx3 = cx2.new_child_scope();

    let output = Rc::new(StringStore::new());
    let out = output.clone();
    let str_sig = create_func_signal(cx3, move || out.push(format!("val: {}", num_sig.get())));

    str_sig.subscribe(num_sig);

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");

    cx2.discard();

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}
