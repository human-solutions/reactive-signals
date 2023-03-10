use std::{
    cell::RefCell,
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
    pub(crate) fn borrow_rt(&self) -> (Runtime, NodeId) {
        let mut vec = self.0.borrow_mut();

        for rt in &mut vec.iter_mut() {
            if !rt.in_use() {
                let id = rt.scope_tree.init(Default::default());
                return (rt.id, id);
            }
        }

        let id = Runtime::from(vec.len());
        let mut rti = RuntimeInner::new(id);
        rti.scope_tree.init(Default::default());
        let sx = rti.scope_tree.root();
        vec.push(rti);
        (id, sx)
    }

    pub(crate) fn return_rt(&self, runtime: &Runtime) {
        let mut pool = self.0.borrow_mut();
        let rt = &mut pool[runtime.0 as usize];
        rt.discard();
    }

    pub(crate) fn bench_clean_all(&self) {
        let mut vec = self.0.borrow_mut();
        vec.iter_mut().for_each(|rt| rt.discard());
        vec.clear();
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub(crate) struct RuntimeInner {
    pub(crate) id: Runtime,
    pub(crate) scope_tree: Tree<ScopeInner>,
}

impl RuntimeInner {
    fn new(id: Runtime) -> Self {
        Self {
            id,
            scope_tree: Tree::create(),
        }
    }

    fn in_use(&self) -> bool {
        self.scope_tree.is_initialized()
    }

    pub fn discard(&mut self) {
        if self.in_use() {
            // also sets the tree to not initialized
            self.scope_tree.discard_all();
        }
    }
}

impl Index<SignalId> for RuntimeInner {
    type Output = ScopeInner;

    fn index(&self, index: SignalId) -> &Self::Output {
        &self.scope_tree[index.sx.sx]
    }
}

impl IndexMut<SignalId> for RuntimeInner {
    fn index_mut(&mut self, index: SignalId) -> &mut Self::Output {
        &mut self.scope_tree[index.sx.sx]
    }
}
