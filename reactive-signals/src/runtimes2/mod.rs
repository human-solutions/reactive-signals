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
mod inner;

use std::cell::RefCell;

pub(crate) use inner::RuntimeInner;

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Runtime<'rt> {
    pub(crate) inner: &'rt RefCell<RuntimeInner<'rt>>,
}
