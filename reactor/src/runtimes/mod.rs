mod inner;
mod pool;
mod single;
mod staticrt;

#[cfg(any(test, feature = "profile"))]
pub use self::pool::PoolRuntimeId;

use crate::Scope;
pub(crate) use inner::RuntimeInner;
pub use pool::RuntimePool;
pub use single::{SingleRuntime, SingleRuntimeId};
pub use staticrt::{StaticRuntime, StaticRuntimeId};

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
