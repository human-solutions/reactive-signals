use crate::{
    iter::{NodeResolver, VecTreeIter},
    runtimes::{Runtime, RuntimeInner},
    signals::SignalId,
};

pub(crate) fn propagate_change<RT: Runtime>(rt: &RuntimeInner<RT>, sig: SignalId<RT>) {
    let tree = &rt.scope_tree;
    let mut iter = VecTreeIter::new(tree, sig);

    while let Some(next) = iter.next() {
        // println!("upd: {next:?}");
        if !tree.node(next).run(rt, next) {
            iter.skip_children();
        }
    }
}
