use std::{fmt, num::NonZeroU32};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeId(NonZeroU32);

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        Self(unsafe { NonZeroU32::new_unchecked(id as u32 + 1) })
    }
}

impl NodeId {
    pub(crate) fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    pub(crate) fn max() -> usize {
        u32::MAX as usize - 1
    }

    pub(crate) fn is_root(self) -> bool {
        self.0.get() == 1
    }

    pub(crate) fn root() -> Self {
        Self(unsafe { NonZeroU32::new_unchecked(1) })
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ᴺ", self.index())
    }
}
