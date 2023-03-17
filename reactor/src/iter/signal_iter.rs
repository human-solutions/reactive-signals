use crate::{
    iter::{IdVec, RefVecElem},
    runtimes::Runtime,
    scope_inner::ScopeInner,
    signal_id::SignalId,
    signal_inner::SignalInner,
};
use arena_link_tree::Tree;

use super::NodeResolver;

impl<'a, RT: Runtime> IdVec for RefVecElem<'a, SignalInner<RT>> {
    type Output = SignalId<RT>;

    fn get(&self, idx: usize) -> Self::Output {
        self.listeners.get(idx)
    }

    fn len(&self) -> usize {
        self.listeners.len()
    }

    fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }
}

impl<'a, RT: Runtime> NodeResolver<'a> for Tree<ScopeInner<RT>> {
    type Id = SignalId<RT>;
    type Elem = RefVecElem<'a, SignalInner<RT>>;
    fn node(&'a self, id: SignalId<RT>) -> Self::Elem {
        RefVecElem::new(self[id.sx].signals.borrow(), id.id.as_usize())
    }
}
