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
mod client;
mod inner;
mod server;
mod test_client;
// mod staticrt;

use crate::Scope;
pub use client::ClientRuntime;
pub(crate) use inner::RuntimeInner;
pub use server::ServerRuntime;
pub use test_client::TestClientRuntime;
// pub use staticrt::{StaticRuntime, StaticRuntimeId};

#[doc(hidden)]
pub trait Runtime: Default + Copy {
    const IS_SERVER: bool;

    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<Self>) -> T;

    fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner<Self>) -> T;

    fn discard(&self) {
        self.with_mut(|rt| rt.discard());
    }
}
