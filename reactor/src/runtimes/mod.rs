mod client;
mod inner;
mod server;
mod staticrt;

#[cfg(any(test, feature = "profile"))]
pub use self::server::ServerRuntime;

use crate::Scope;
pub use client::{ClientRuntime, SingleClientRuntime};
pub(crate) use inner::RuntimeInner;
pub use server::ServerRuntimePool;
pub use staticrt::{StaticRuntime, StaticRuntimeId};

pub trait Runtime: Default + Copy + 'static {
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
