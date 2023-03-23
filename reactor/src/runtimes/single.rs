use crate::CellType;

use super::{Runtime, RuntimeInner, Scope};

thread_local! {
  pub static RUNTIME: SingleRuntime = Default::default();
}

#[derive(Default)]
pub struct SingleRuntime(CellType<RuntimeInner<SingleRuntimeId>>);

#[derive(Default, Clone, Copy)]
pub struct SingleRuntimeId;

impl Runtime for SingleRuntimeId {
    fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner<SingleRuntimeId>) -> T,
    {
        RUNTIME.with(|rt| f(&mut rt.rt_mut()))
    }

    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<SingleRuntimeId>) -> T,
    {
        RUNTIME.with(|rt| f(&rt.rt_ref()))
    }

}

impl  SingleRuntime {
    pub fn new_root_scope() -> Scope<SingleRuntimeId> {
        RUNTIME.with(|rt| {
            let data = rt.rt_mut();
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
            drop(rt.rt_mut().discard());
            Self::new_root_scope()
        })
    }
}

#[cfg(not(feature = "unsafe-cell"))]
impl SingleRuntime {
    #[inline]
    fn rt_ref(&self) -> cell::Ref<RuntimeInner<SingleRuntimeId>> {
        self.0.borrow()
    }

    #[inline]
    fn rt_mut(&self) -> cell::RefMut<RuntimeInner<SingleRuntimeId>> {
        self.0.borrow_mut()
    }

}
#[cfg(feature = "unsafe-cell")]
impl SingleRuntime {
    #[inline]
    fn rt_ref(&self) -> &RuntimeInner<SingleRuntimeId> {
        unsafe { &*self.0.get() }
    }

    #[inline]
    fn rt_mut(&self) -> &mut RuntimeInner<SingleRuntimeId> {
        unsafe { &mut *self.0.get() }
    }
}