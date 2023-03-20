use std::cell::UnsafeCell;

use super::{Runtime, RuntimeInner, Scope};

thread_local! {
  pub static RUNTIME: SingleRuntime = Default::default();
}

#[derive(Default)]
pub struct SingleRuntime(UnsafeCell<RuntimeInner<SingleRuntimeId>>);

#[derive(Default, Clone, Copy)]
pub struct SingleRuntimeId;

impl Runtime for SingleRuntimeId {
    fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner<SingleRuntimeId>) -> T,
    {
        RUNTIME.with(|rt| f(unsafe { &mut *rt.0.get() }))
    }

    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<SingleRuntimeId>) -> T,
    {
        RUNTIME.with(|rt| f(unsafe  { &*rt.0.get() }))
    }

}

impl  SingleRuntime {
    pub fn new_root_scope() -> Scope<SingleRuntimeId> {
        RUNTIME.with(|rt| {
            let data = unsafe {&mut *rt.0.get()};
            if data.in_use() {
                panic!("Runtime is already used. Make sure to not call new_root_scope() more than once on a thread");
            }
            let mut rti = RuntimeInner::new();
            let sx = rti.scope_tree.init(Default::default());
            *data = rti;

            Scope {
                sx,
                rt: SingleRuntimeId,
            }
    
        })
    }

    #[cfg(any(test, feature = "profile"))]
    pub fn bench_root_scope() -> Scope<SingleRuntimeId> {
        RUNTIME.with(|rt| {
            let data = unsafe {&mut *rt.0.get()};
            data.discard();
            Self::new_root_scope()
        })
    }

}
