use arena_link_tree::NodeId;

use crate::{primitives::u15Bool, runtime_inner::RuntimeInner, scope::Scope, Runtime};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignalId {
    pub(crate) id: u15Bool,
    pub(crate) sx: NodeId,
    pub(crate) rt: Runtime,
}

impl SignalId {
    pub(crate) fn new(id: usize, sx: Scope) -> Self {
        Self {
            sx: sx.sx,
            id: u15Bool::new(id, false),
            rt: sx.rt,
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
        self.rt.with_ref(f)
    }

    #[inline]
    pub(crate) fn rt_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RuntimeInner) -> T,
    {
        self.rt.with_mut(f)
    }
}

impl std::fmt::Debug for SignalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}ˢⁱᵍ", self.sx, self.id.u15())
    }
}

#[test]
fn signal_id_size() {
    assert_eq!(std::mem::size_of::<SignalId>(), 8);
}
