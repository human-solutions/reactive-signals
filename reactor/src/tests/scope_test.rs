use std::rc::Rc;

use crate::{runtimes::RuntimePool, signal, tests::StringStore};

#[test]
fn test_scopes_deep() {
    let root = RuntimePool::new_root_scope();

    let mut cx = root.clone();
    let num_sig = signal!(cx, 5);

    (0..3).for_each(|_| cx = cx.new_child());

    let output = Rc::new(StringStore::new());
    let _str_sig = signal!(cx, clone: output, move || output
        .push(format!("val: {}", num_sig.get())));

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[test]
fn test_scopes_discard() {
    let root = RuntimePool::new_root_scope();

    let cx0 = root.clone();
    let num_sig = signal!(cx0, 5);

    let cx1 = cx0.new_child();
    let cx2 = cx1.new_child();
    let cx3 = cx2.new_child();

    let output = Rc::new(StringStore::new());
    let _str_sig = signal!(cx3, clone: output, move || output
        .push(format!("val: {}", num_sig.get())));

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");

    cx2.discard();

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}
