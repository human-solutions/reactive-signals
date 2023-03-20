#![allow(dead_code)]

mod inner;
mod pool;

pub(crate) use inner::RuntimeInner;

#[cfg(any(test, feature = "profile"))]
pub use self::pool::PoolRuntimeId;

use crate::Scope;

pub use pool::RuntimePool;

#[cfg(not(feature = "use-unsafe"))]
mod single;
#[cfg(not(feature = "use-unsafe"))]
pub use single::SingleRuntime;
#[cfg(all(not(feature = "use-unsafe"), any(test, feature = "profile")))]
pub use single::SingleRuntimeId;

#[cfg(feature = "use-unsafe")]
mod single_unsafe;
#[cfg(feature = "use-unsafe")]
pub use single_unsafe::SingleRuntime;
#[cfg(all(feature = "use-unsafe", any(test, feature = "profile")))]
pub use single_unsafe::SingleRuntimeId;

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
