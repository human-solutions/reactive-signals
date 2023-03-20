use crate::runtime_inner::{RuntimeInner, RUNTIMES};

#[derive(Default, Clone, Copy)]
pub struct Runtime(pub(crate) u32);

impl Runtime {
    pub(crate) fn from(idx: usize) -> Self {
        if idx >= u32::MAX as usize {
            panic!("Too many runtimes. Check your code for leaks. A runtime needs to be discarded");
        }
        Self(idx as u32)
    }

    pub(crate) fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner) -> T,
    {
        RUNTIMES.with(|pool| {
            let pool = pool.0.borrow();
            let rt = &pool[self.0 as usize];
            f(rt)
        })
    }

    pub(crate) fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner) -> T,
    {
        RUNTIMES.with(|pool| {
            let mut pool = pool.0.borrow_mut();
            let rt = &mut pool[self.0 as usize];
            f(rt)
        })
    }
}
