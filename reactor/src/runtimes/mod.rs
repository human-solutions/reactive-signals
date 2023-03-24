mod client;
mod inner;
mod server;
// mod staticrt;

use crate::Scope;
pub use client::ClientRuntime;
pub(crate) use inner::RuntimeInner;
pub use server::ServerRuntime;
// pub use staticrt::{StaticRuntime, StaticRuntimeId};

#[doc(hidden)]
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
