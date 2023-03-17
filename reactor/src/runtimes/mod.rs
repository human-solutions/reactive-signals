#![allow(dead_code)]

mod inner;
mod pool;
mod single;

pub(crate) use inner::RuntimeInner;

#[cfg(any(test, feature = "profile"))]
pub use self::{pool::PoolRuntimeId, single::SingleRuntimeId};

use crate::Scope;

pub use pool::RuntimePool;
pub use single::SingleRuntime;

pub trait Runtime: Default + Copy + 'static {
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
