use crate::{
    iter::{IdVec, RefVecElem},
    scope_inner::ScopeInner,
    signal_id::SignalId,
    signal_inner::SignalInner,
};
use arena_link_tree::Tree;

use super::NodeResolver;

impl<'a> IdVec for RefVecElem<'a, SignalInner> {
    type Output = SignalId;

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

impl<'a> NodeResolver<'a> for Tree<ScopeInner> {
    type Id = SignalId;
    type Elem = RefVecElem<'a, SignalInner>;
    fn node(&'a self, id: SignalId) -> Self::Elem {
        RefVecElem::new(self[id.sx].signals.borrow(), id.id.usize())
    }
}
