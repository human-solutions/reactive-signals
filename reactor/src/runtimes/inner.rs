use std::{
    cell::Cell,
    ops::{Index, IndexMut},
};

use arena_link_tree::Tree;

use crate::{scope_inner::ScopeInner, signal::SignalId};

use super::Runtime;

#[derive(Default)]
pub struct RuntimeInner<RT: Runtime> {
    pub(crate) scope_tree: Tree<ScopeInner<RT>>,
    running_signal: Cell<Option<SignalId<RT>>>,
}

impl<RT: Runtime> RuntimeInner<RT> {
    pub(crate) fn new() -> Self {
        Self {
            scope_tree: Tree::create(),
            running_signal: Cell::new(None),
        }
    }

    pub(crate) fn in_use(&self) -> bool {
        self.scope_tree.is_initialized()
    }

    pub fn discard(&mut self) {
        if self.in_use() {
            // also sets the tree to not initialized
            self.scope_tree.discard_all();
        }
    }

    pub(crate) fn get_running_signal(&self) -> Option<SignalId<RT>> {
        self.running_signal.get()
    }

    pub(crate) fn set_running_signal(&self, signal: Option<SignalId<RT>>) -> Option<SignalId<RT>> {
        let previous = self.running_signal.take();
        self.running_signal.set(signal);
        previous
    }
}

impl<RT: Runtime> Index<SignalId<RT>> for RuntimeInner<RT> {
    type Output = ScopeInner<RT>;

    fn index(&self, index: SignalId<RT>) -> &Self::Output {
        &self.scope_tree[index.sx]
    }
}

impl<RT: Runtime> IndexMut<SignalId<RT>> for RuntimeInner<RT> {
    fn index_mut(&mut self, index: SignalId<RT>) -> &mut Self::Output {
        &mut self.scope_tree[index.sx]
    }
}
