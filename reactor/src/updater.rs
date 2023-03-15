use crate::{
    iter::{ChildVecResolver, VecTreeIter},
    runtime_inner::RuntimeInner,
    signal_id::SignalId,
    signal_inner::SignalValue,
};

pub(crate) fn propagate_change(rt: &RuntimeInner, sig: SignalId) {
    let tree = &rt.scope_tree;
    let mut iter = VecTreeIter::new(tree, sig);

    while let Some(next) = iter.next() {
        if let SignalValue::Func(func) = &tree.child_vec(next).value {
            let changed = func.run();
            if !changed {
                iter.skip_children();
            }
        }
    }
}
