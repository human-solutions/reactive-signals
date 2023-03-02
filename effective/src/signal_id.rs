use crate::{runtime_inner::RuntimeInner, scope::Scope};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SignalId {
    pub(crate) sx: Scope,
    id: u32,
}

impl SignalId {
    pub(crate) fn new(id: usize, sx: Scope) -> Self {
        Self { sx, id: id as u32 }
    }

    pub(crate) fn index(&self) -> usize {
        self.id as usize
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
        write!(f, "{:?}{}ˢⁱᵍ", self.sx.sx, self.id)
    }
}
