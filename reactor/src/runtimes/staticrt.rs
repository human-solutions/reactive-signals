use std::cell::RefCell;

use crate::scope::Scope;

use super::{Runtime, RuntimeInner};

pub struct StaticRuntime(StaticRuntimeId);

#[derive(Clone, Copy)]
pub struct StaticRuntimeId(&'static RefCell<RuntimeInner<StaticRuntimeId>>);

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
        f(&mut self.0.borrow_mut())
    }

    #[inline]
    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<StaticRuntimeId>) -> T,
    {
        f(&self.0.borrow())
    }
}

impl StaticRuntime {
    pub fn new_root_scope() -> Scope<StaticRuntimeId> {
        let mut rti = RuntimeInner::new();
        let sx = rti.scope_tree.init(Default::default());
        let rt = StaticRuntimeId(Box::leak(Box::new(RefCell::new(rti))));
        Scope { sx, rt }
    }

    // #[cfg(any(test, feature = "profile"))]
    // pub fn bench_root_scope() -> Scope<StaticRuntimeId> {
    //     RUNTIME.with(|rt| {
    //         drop(rt.0.borrow_mut().discard());
    //         Self::new_root_scope()
    //     })
    // }
}
