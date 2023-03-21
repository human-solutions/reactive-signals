mod inner;
mod pool;

pub(crate) use inner::RuntimeInner;

#[cfg(any(test, feature = "profile"))]
pub use self::pool::PoolRuntimeId;

use crate::Scope;

pub use pool::RuntimePool;

////// StaticRuntime ////////

#[cfg(not(feature = "unsafe-cell"))]
mod staticrt;
#[cfg(not(feature = "unsafe-cell"))]
pub use staticrt::StaticRuntime;
#[cfg(all(not(feature = "unsafe-cell"), any(test, feature = "profile")))]
pub use staticrt::StaticRuntimeId;

#[cfg(feature = "unsafe-cell")]
mod staticrt_unsafe;
#[cfg(feature = "unsafe-cell")]
pub use staticrt_unsafe::StaticRuntime;
#[cfg(all(feature = "unsafe-cell", any(test, feature = "profile")))]
pub use staticrt_unsafe::StaticRuntimeId;

////// SingleRuntime ////////

#[cfg(not(feature = "unsafe-cell"))]
mod single;
#[cfg(not(feature = "unsafe-cell"))]
pub use single::SingleRuntime;
#[cfg(all(not(feature = "unsafe-cell"), any(test, feature = "profile")))]
pub use single::SingleRuntimeId;

#[cfg(feature = "unsafe-cell")]
mod single_unsafe;
#[cfg(feature = "unsafe-cell")]
pub use single_unsafe::SingleRuntime;
#[cfg(all(feature = "unsafe-cell", any(test, feature = "profile")))]
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
