use std::cell::UnsafeCell;

use crate::scope::Scope;

use super::{Runtime, RuntimeInner};

pub struct StaticRuntime(StaticRuntimeId);

#[derive(Clone, Copy)]
pub struct StaticRuntimeId(&'static UnsafeCell<RuntimeInner<StaticRuntimeId>>);

impl Default for StaticRuntimeId {
    fn default() -> Self {
        todo!()
    }
}
impl Runtime for StaticRuntimeId {
    #[inline]
    fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner<StaticRuntimeId>) -> T,
    {
        f(unsafe { &mut *self.0.get() })
    }

    #[inline]
    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<StaticRuntimeId>) -> T,
    {
        f(unsafe { &*self.0.get() })
    }
}

impl StaticRuntime {
    pub fn new_root_scope() -> Scope<StaticRuntimeId> {
        let mut rti = RuntimeInner::new();
        let sx = rti.scope_tree.init(Default::default());
        let rt = StaticRuntimeId(Box::leak(Box::new(UnsafeCell::new(rti))));
        Scope { sx, rt }
    }

    #[cfg(any(test, feature = "profile"))]
    pub fn bench_root_scope() -> Scope<StaticRuntimeId> {
        Self::new_root_scope()
    }
}
