use std::cell::RefCell;

use super::{Runtime, RuntimeInner, Scope};

thread_local! {
  pub static RUNTIME_POOL: TestClientRuntimePool = Default::default();
}

/// A runtime meant to be used for testing only. It uses a pool of runtimes,
/// so that many runtimes can co-exist on one thread, but simulates running on a client.
///
/// ```no_run
/// use reactor::{Scope, signal, runtimes::TestClientRuntime};
///
/// // when starting a client you create the root scope
/// let sc = TestClientRuntime::new_root_scope();
///
/// // this scope is then used for building a tree of scopes.
/// app(sc);
///
/// // calling discard() on the root scope will discard the TestClientRuntime as well.
/// sc.discard();
///
/// fn app(sc: Scope<TestClientRuntime>) {
///     // a signal marked with `server` will not run in a Scope<TestClientRuntime>
///     let sig = signal!(sc, server, move || println!("server!"));
/// }
/// ```
///
/// See [runtimes](super) for full documentation.
///
#[derive(Default, Clone, Copy)]
pub struct TestClientRuntime(u32);

impl TestClientRuntime {
    pub(crate) fn from(idx: usize) -> Self {
        if idx >= u32::MAX as usize {
            panic!("Too many runtimes. Check your code for leaks. A runtime needs to be discarded");
        }
        Self(idx as u32)
    }
}

impl Runtime for TestClientRuntime {
    const IS_SERVER: bool = false;

    fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner<TestClientRuntime>) -> T,
    {
        RUNTIME_POOL.with(|pool| {
            let mut pool = pool.0.borrow_mut();
            let rt = &mut pool[self.0 as usize];
            f(rt)
        })
    }

    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<TestClientRuntime>) -> T,
    {
        RUNTIME_POOL.with(|pool| {
            let pool = pool.0.borrow();
            let rt = &pool[self.0 as usize];
            f(rt)
        })
    }
}

#[derive(Default)]
pub struct TestClientRuntimePool(RefCell<Vec<RuntimeInner<TestClientRuntime>>>);

impl TestClientRuntime {
    pub fn new_root_scope() -> Scope<TestClientRuntime> {
        RUNTIME_POOL.with(|rt| {
            let mut vec = rt.0.borrow_mut();

            for (i, rt) in &mut vec.iter_mut().enumerate() {
                if !rt.in_use() {
                    let id = rt.scope_tree.init(Default::default());
                    return Scope {
                        rt: TestClientRuntime(i as u32),
                        sx: id,
                    };
                }
            }

            let id = TestClientRuntime::from(vec.len());
            let mut rti = RuntimeInner::new();
            rti.scope_tree.init(Default::default());
            let sx = rti.scope_tree.root();
            vec.push(rti);
            Scope { rt: id, sx }
        })
    }

    #[cfg(any(test, feature = "profile"))]
    pub fn bench_root_scope() -> Scope<TestClientRuntime> {
        RUNTIME_POOL.with(|rt| {
            drop(rt.0.borrow_mut().clear());
            Self::new_root_scope()
        })
    }
}
