use crate::{
    arena_tree::Tree,
    iter::{IdVec, RefVecElem},
    runtimes::Runtime,
    signals::{SignalId, SignalInner},
    ScopeInner,
};

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
        RefVecElem::new(self[id.sx].vec_ref(), id.id.as_usize())
    }
}
