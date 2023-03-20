use std::cell::RefCell;

use super::{Runtime, RuntimeInner, Scope};

thread_local! {
  pub static RUNTIME_POOL: RuntimePool = Default::default();
}

#[derive(Default, Clone, Copy)]
pub struct PoolRuntimeId(u32);

impl PoolRuntimeId {
    pub(crate) fn from(idx: usize) -> Self {
        if idx >= u32::MAX as usize {
            panic!("Too many runtimes. Check your code for leaks. A runtime needs to be discarded");
        }
        Self(idx as u32)
    }
}

impl Runtime for PoolRuntimeId {
    fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner<PoolRuntimeId>) -> T,
    {
        RUNTIME_POOL.with(|pool| {
            let mut pool = pool.0.borrow_mut();
            let rt = &mut pool[self.0 as usize];
            f(rt)
        })
    }

    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<PoolRuntimeId>) -> T,
    {
        RUNTIME_POOL.with(|pool| {
            let pool = pool.0.borrow();
            let rt = &pool[self.0 as usize];
            f(rt)
        })
    }
}

#[derive(Default)]
pub struct RuntimePool(RefCell<Vec<RuntimeInner<PoolRuntimeId>>>);

impl RuntimePool {
    pub fn new_root_scope() -> Scope<PoolRuntimeId> {
        RUNTIME_POOL.with(|rt| {
            let mut vec = rt.0.borrow_mut();

            for (i, rt) in &mut vec.iter_mut().enumerate() {
                if !rt.in_use() {
                    let id = rt.scope_tree.init(Default::default());
                    return Scope {
                        rt: PoolRuntimeId(i as u32),
                        sx: id,
                    };
                }
            }

            let id = PoolRuntimeId::from(vec.len());
            let mut rti = RuntimeInner::new();
            rti.scope_tree.init(Default::default());
            let sx = rti.scope_tree.root();
            vec.push(rti);
            Scope { rt: id, sx }
        })
    }

    #[cfg(any(test, feature = "profile"))]
    pub fn bench_root_scope() -> Scope<PoolRuntimeId> {
        RUNTIME_POOL.with(|rt| {
            drop(rt.0.borrow_mut().clear());
            Self::new_root_scope()
        })
    }
}
