use std::{
    cell::Cell,
    ops::{Index, IndexMut},
};

use crate::{arena_tree::Tree, CellType};

use crate::scope::ScopeInner;
use crate::signals::SignalId;

#[derive(Default)]
pub struct RuntimeInner<'rt> {
    pub(crate) scope_tree: Tree<ScopeInner<'rt>>,
    running_signal: Cell<Option<SignalId<'rt>>>,
}

impl<'rt> RuntimeInner<'rt> {
    pub fn new() -> CellType<Self> {
        CellType::new(Self {
            scope_tree: Tree::create(),
            running_signal: Cell::new(None),
        })
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

    pub(crate) fn get_running_signal(&self) -> Option<SignalId<'rt>> {
        self.running_signal.get()
    }

    pub(crate) fn set_running_signal(
        &self,
        signal: Option<SignalId<'rt>>,
    ) -> Option<SignalId<'rt>> {
        let previous = self.running_signal.take();
        self.running_signal.set(signal);
        previous
    }
}

impl<'rt> Index<SignalId<'rt>> for RuntimeInner<'rt> {
    type Output = ScopeInner<'rt>;

    fn index(&self, index: SignalId<'rt>) -> &Self::Output {
        &self.scope_tree[index.sx]
    }
}

impl<'rt> IndexMut<SignalId<'rt>> for RuntimeInner<'rt> {
    fn index_mut(&mut self, index: SignalId) -> &mut Self::Output {
        &mut self.scope_tree[index.sx]
    }
}
