use crate::{
    iter::{IdVec, RefVecElem},
    scope_inner::ScopeInner,
    signal_id::SignalId,
    signal_inner::SignalInner,
};
use arena_link_tree::Tree;

use super::ChildVecResolver;

impl<'a> IdVec for RefVecElem<'a, SignalInner> {
    type Output = SignalId;

    fn get(&self, idx: usize) -> Self::Output {
        self.listeners[idx]
    }

    fn len(&self) -> usize {
        self.listeners.len()
    }

    fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }
}

impl<'a> ChildVecResolver<'a> for Tree<ScopeInner> {
    type Id = SignalId;
    type Elem = RefVecElem<'a, SignalInner>;
    fn child_vec(&'a self, id: SignalId) -> Self::Elem {
        RefVecElem::new(self[id.sx].signals.borrow(), id.id.usize())
    }
}
