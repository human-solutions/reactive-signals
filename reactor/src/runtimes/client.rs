use crate::CellType;

use super::{Runtime, RuntimeInner, Scope};

thread_local! {
  pub static RUNTIME: SingleClientRuntime = Default::default();
}

#[derive(Default)]
pub struct SingleClientRuntime(CellType<RuntimeInner<ClientRuntime>>);

/// A runtime meant to be used client-side because there can only be one per thread.
/// 
/// 
/// ```no_run
/// use reactor::{Scope, signal, runtimes::ClientRuntime};
/// 
/// // when starting a client you create the root scope
/// let sc = ClientRuntime::new_root_scope();
/// 
/// // this scope is then used for building a tree of scopes.
/// app(sc);
/// 
/// // calling discard() on the root scope will discard the ClientRuntime as well.
/// sc.discard();
/// 
/// fn app(sc: Scope<ClientRuntime>) {
///     // a signal marked with `server` will not run in a Scope<ClientRuntime>
///     let sig = signal!(sc, server, move || println!("server!"));
/// }
/// ```
/// 
/// See [runtimes](super) for full documentation.
/// 
#[derive(Default, Clone, Copy)]
pub struct ClientRuntime;

impl Runtime for ClientRuntime {
    const IS_SERVER: bool = false;
    
    fn with_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner<ClientRuntime>) -> T,
    {
        RUNTIME.with(|rt| f(&mut rt.rt_mut()))
    }

    fn with_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner<ClientRuntime>) -> T,
    {
        RUNTIME.with(|rt| f(&rt.rt_ref()))
    }

}

impl  ClientRuntime {
    pub fn new_root_scope() -> Scope<ClientRuntime> {
        RUNTIME.with(|rt| {
            #[allow(unused_mut)]
            let mut data = rt.rt_mut();
            if data.in_use() {
                panic!("Runtime is already used. Make sure to not call new_root_scope() more than once on a thread");
            }
            let mut rti = RuntimeInner::new();
            let sx = rti.scope_tree.init(Default::default());
            *data = rti;

            Scope {
                sx,
                rt: ClientRuntime,
            }
    
        })
    }

    #[cfg(any(test, feature = "profile"))]
    pub fn bench_root_scope() -> Scope<ClientRuntime> {
        RUNTIME.with(|rt| {
            drop(rt.rt_mut().discard());
            Self::new_root_scope()
        })
    }
}

#[cfg(not(feature = "unsafe-cell"))]
impl SingleClientRuntime {
    #[inline]
    fn rt_ref(&self) -> std::cell::Ref<RuntimeInner<ClientRuntime>> {
        self.0.borrow()
    }

    #[inline]
    fn rt_mut(&self) -> std::cell::RefMut<RuntimeInner<ClientRuntime>> {
        self.0.borrow_mut()
    }

}
#[cfg(feature = "unsafe-cell")]
impl SingleClientRuntime {
    #[inline]
    fn rt_ref(&self) -> &RuntimeInner<ClientRuntime> {
        unsafe { &*self.0.get() }
    }

    #[inline]
    fn rt_mut(&self) -> &mut RuntimeInner<ClientRuntime> {
        unsafe { &mut *self.0.get() }
    }
}