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
    ptr::NonNull,
};

use crate::{scope::ScopeInner, signals::SignalId, CellType, Scope, Tree};

pub struct RuntimeGuard(&'static Runtime);

impl Drop for RuntimeGuard {
    fn drop(&mut self) {
        // the official rust docs proposes to use this to
        // drop something previously leaked
        // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
        // but on the rust discord there's many different opinions.
        let nn = NonNull::from(self.0);
        let b = unsafe { Box::from_raw(nn.as_ptr()) };
        drop(b);
    }
}

#[doc(hidden)]
pub(crate) struct Runtime {
    pub(crate) inner: RefCell<RuntimeInner>,
}

impl Runtime {
    pub(crate) fn new(client_side: bool) -> &'static Runtime {
        let inner = RuntimeInner::new(client_side);
        Box::leak(Box::new(Self { inner }))
    }

    pub fn new_root_scope(&'static self) -> Scope {
        let mut rti = self.inner.borrow_mut();
        let sx = rti.scope_tree.init(Default::default());

        Scope { sx, rt: self }
    }

    pub(crate) fn client_side(&self) -> bool {
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

    pub fn discard(&mut self) {
        self.scope_tree.discard_all();
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
