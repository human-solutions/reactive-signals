//!
//! Runtimes are the starting point of a reactive-signals based application. Internally, the runtimes presents a
//! simple boolean constant to let the [Scope](crate::Scope)s and [Signal](crate::Signal)s know
//! where they are running. Like that a signal marked with `server` or `client` knows if it should run.
//!
//! There are two types of runtimes:
//!
//! - Pooled runtimes: Allows for many runtimes in a thread.
//! - Single runtimes: Limitied to one runtime per thread.
//!
//! A runtime presents a single function: `new_root_scope()` which returns a root [Scope](crate::Scope).
//! When the root scope is discarded, using it's [discard()](crate::Scope::discard()) function, the
//! runtime is discarded as well.
//!
//! Single runtimes have no memory overhead, whereas pooled runtimes have an overhead of 2 bytes
//! which is the index in the pool. As a consequence a pool can have at most 65k runtimes.
//!

use std::{
    cell::{Cell, RefCell},
    ops::{Index, IndexMut},
};

use crate::{scope::ScopeInner, signals::SignalId, CellType, Scope, Tree};

#[doc(hidden)]
pub struct Runtime {
    pub(crate) inner: RefCell<RuntimeInner>,
}

impl Runtime {
    pub fn new_client_side() -> &'static Self {
        Self::new(true)
    }

    pub fn new_server_side() -> &'static Self {
        Self::new(false)
    }

    fn new(client_side: bool) -> &'static Self {
        let inner = RuntimeInner::new(client_side);
        let rt: &'static Self = Box::leak(Box::new(Self { inner }));
        rt
    }

    pub fn new_root_scope(&'static self) -> Scope {
        let mut rti = self.inner.borrow_mut();
        let sx = rti.scope_tree.init(Default::default());

        Scope { sx, rt: self }
    }

    pub fn client_side(&self) -> bool {
        self.inner.borrow().client_side
    }
}

#[derive(Default)]
pub struct RuntimeInner {
    pub(crate) scope_tree: Tree<ScopeInner>,
    running_signal: Cell<Option<SignalId>>,
    client_side: bool,
}

impl RuntimeInner {
    pub fn new(client_side: bool) -> CellType<Self> {
        CellType::new(Self {
            scope_tree: Tree::create(),
            running_signal: Cell::new(None),
            client_side,
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

    pub(crate) fn get_running_signal(&self) -> Option<SignalId> {
        self.running_signal.get()
    }

    pub(crate) fn set_running_signal(&self, signal: Option<SignalId>) -> Option<SignalId> {
        let previous = self.running_signal.take();
        self.running_signal.set(signal);
        previous
    }
}

impl Index<SignalId> for RuntimeInner {
    type Output = ScopeInner;

    fn index(&self, index: SignalId) -> &Self::Output {
        &self.scope_tree[index.sx]
    }
}

impl IndexMut<SignalId> for RuntimeInner {
    fn index_mut(&mut self, index: SignalId) -> &mut Self::Output {
        &mut self.scope_tree[index.sx]
    }
}
