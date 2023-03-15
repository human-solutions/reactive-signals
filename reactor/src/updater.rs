use crate::{
    iter::{NodeResolver, VecTreeIter},
    runtime_inner::RuntimeInner,
    signal_id::SignalId,
};

pub(crate) fn propagate_change(rt: &RuntimeInner, sig: SignalId) {
    let tree = &rt.scope_tree;
    let mut iter = VecTreeIter::new(tree, sig);

    while let Some(next) = iter.next() {
        // println!("upd: {next:?}");
        if !tree.node(next).run(rt, next) {
            iter.skip_children();
        }
    }
}
