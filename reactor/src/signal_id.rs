use crate::{primitives::u15Bool, runtime_inner::RuntimeInner, scope::Scope};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SignalId {
    id: u15Bool,
    pub(crate) sx: Scope,
}

impl SignalId {
    pub(crate) fn new(id: usize, sx: Scope) -> Self {
        Self {
            sx,
            id: u15Bool::new(id, false),
        }
    }

    pub(crate) fn index(&self) -> usize {
        self.id.usize()
    }

    #[inline]
    pub(crate) fn rt_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner) -> T,
    {
        self.sx.rt.with_ref(f)
    }

    #[inline]
    pub(crate) fn rt_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner) -> T,
    {
        self.sx.rt.with_mut(f)
    }
}

impl std::fmt::Debug for SignalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}ˢⁱᵍ", self.sx.sx, self.id.u15())
    }
}

#[test]
fn signal_id_size() {
    assert_eq!(std::mem::size_of::<SignalId>(), 12);
}
