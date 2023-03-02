use crate::{Runtime, RuntimeInner};

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Scope {
    pub(crate) id: ScopeId,
    pub(crate) rt: Runtime,
}

impl Scope {
    pub(crate) fn new(id: ScopeId, rt: &RuntimeInner) -> Self {
        Scope { id, rt: rt.id }
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ScopeId(pub(crate) u32);
