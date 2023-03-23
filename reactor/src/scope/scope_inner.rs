use arena_tree::FlagVec;

use crate::{runtimes::Runtime, scope::Scope, signals::SignalId, signals::SignalInner, CellType};

#[derive(Debug, Default)]
pub(crate) struct ScopeInner<RT: Runtime> {
    signals: CellType<Vec<SignalInner<RT>>>,
}

impl<RT: Runtime> ScopeInner<RT> {
    /// **Warning!**
    ///
    /// This signal id is not yet valid. There has to be a subsequent
    /// call to `insert_signal` before it is valid
    pub fn next_signal_id(&self, sx: Scope<RT>) -> SignalId<RT> {
        let idx = self.vec_ref().len();
        SignalId::new(idx, sx)
    }

    pub fn insert_signal(&self, signal: SignalInner<RT>) {
        self.vec_mut().push(signal);
    }

    pub fn with_signal<F, T>(&self, id: SignalId<RT>, f: F) -> T
    where
        F: FnOnce(&SignalInner<RT>) -> T,
    {
        let signals = self.vec_ref();
        let signal = signals.get(id.index()).unwrap();
        f(&signal)
    }

    pub(crate) fn remove_scopes(&mut self, discarded_scopes: &FlagVec) {
        #[allow(unused_mut)]
        let mut signals = self.vec_mut();
        signals.iter_mut().for_each(|signal| {
            signal
                .listeners
                .retain(|s| !discarded_scopes.get(s.sx.as_raw() as usize))
        });
    }

    pub(crate) fn reuse(&self) {
        #[allow(unused_mut)]
        let mut signals = self.vec_mut();
        signals.iter_mut().for_each(|signal| signal.reuse());
        signals.clear();
    }
}

#[cfg(not(feature = "unsafe-cell"))]
impl<RT: Runtime> ScopeInner<RT> {
    #[inline]
    pub(crate) fn vec_ref(&self) -> std::cell::Ref<Vec<SignalInner<RT>>> {
        self.signals.borrow()
    }

    #[inline]
    fn vec_mut(&self) -> std::cell::RefMut<Vec<SignalInner<RT>>> {
        self.signals.borrow_mut()
    }
}
#[cfg(feature = "unsafe-cell")]
impl<RT: Runtime> ScopeInner<RT> {
    #[inline]
    pub(crate) fn vec_ref(&self) -> &Vec<SignalInner<RT>> {
        unsafe { &*self.signals.get() }
    }

    #[inline]
    fn vec_mut(&self) -> &mut Vec<SignalInner<RT>> {
        unsafe { &mut *self.signals.get() }
    }
}
