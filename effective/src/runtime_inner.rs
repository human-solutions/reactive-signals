use std::{
    cell::{Cell, RefCell},
    ops::{Index, IndexMut},
};

use arena_link_tree::{NodeId, Tree};

use crate::{scope_inner::ScopeInner, signal_id::SignalId, Runtime};

thread_local! {
  pub(crate) static RUNTIMES: RuntimePool = Default::default();
}

#[derive(Default)]
pub(crate) struct RuntimePool(pub(crate) RefCell<Vec<RuntimeInner>>);

impl RuntimePool {
    pub(crate) fn borrow(&self) -> (Runtime, NodeId) {
        {
            for rt in self.0.borrow().iter() {
                if !rt.in_use.get() {
                    rt.in_use.set(true);
                    return (rt.id, rt.root_scope);
                }
            }
        }
        let mut vec = self.0.borrow_mut();
        let id = Runtime::from(vec.len());
        let rti = RuntimeInner::new(id);
        let sx = rti.scopes.root();
        vec.push(rti);
        (id, sx)
    }

    pub(crate) fn return_to_pool(&self, runtime: &Runtime) {
        let mut pool = self.0.borrow_mut();
        let rt = &mut pool[runtime.0 as usize];
        rt.discard();
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub(crate) struct RuntimeInner {
    pub(crate) id: Runtime,
    in_use: Cell<bool>,
    pub(crate) scopes: Tree<ScopeInner>,
    pub(crate) root_scope: NodeId,
}

impl RuntimeInner {
    fn new(id: Runtime) -> Self {
        let scopes = Tree::new_with_root(ScopeInner::default());
        let root_scope = scopes.root();
        Self {
            id,
            in_use: Cell::new(true),
            scopes,
            root_scope,
        }
    }

    pub fn discard(&mut self) {
        self.in_use.set(false);
        self.scopes.reuse_tree(|s| s.reuse());
    }
}

impl Index<SignalId> for RuntimeInner {
    type Output = ScopeInner;

    fn index(&self, index: SignalId) -> &Self::Output {
        &self.scopes[index.sx.sx]
    }
}

impl IndexMut<SignalId> for RuntimeInner {
    fn index_mut(&mut self, index: SignalId) -> &mut Self::Output {
        &mut self.scopes[index.sx.sx]
    }
}
