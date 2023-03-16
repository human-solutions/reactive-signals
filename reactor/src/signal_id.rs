use std::cmp::Ordering;

use arena_link_tree::NodeId;

use crate::{primitives::u15Bool, runtime_inner::RuntimeInner, scope::Scope, Runtime};

/// The SignalId has three components:
///
/// - `rt`: The Runtime id
/// - `sx`: The Scope id (actually a NodeId as the Scope is stored in an arena Tree)
/// - `id`: The id of the signal.
///
///
/// ## Ordering and equality
///
/// The SignalId doesn't use the `rt` for ordering or equality operations,
/// because it is up to the user to make sure that signals are not used
/// across runtimes.
///
/// They are ordered by Scope and then by `id`.
#[derive(Clone, Copy)]
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
        self.id.as_usize()
    }

    #[inline]
    pub(crate) fn rt_ref<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RuntimeInner) -> T,
    {
        self.rt.with_ref(f)
    }
}

impl PartialEq for SignalId {
    #[inline]
    fn eq(&self, other: &SignalId) -> bool {
        self.id == other.id && self.sx == other.sx
    }
}

impl Eq for SignalId {}

// ordering by NodeId (Scope) and then id. The runtime is not considered
// as it is assumed to be the same for all SignalId's running on the same
// thread
impl PartialOrd for SignalId {
    #[inline]
    fn partial_cmp(&self, other: &SignalId) -> Option<Ordering> {
        match self.sx.partial_cmp(&other.sx) {
            Some(Ordering::Equal) => self.id.partial_cmp(&other.id),
            cmp => cmp,
        }
    }
}

impl Ord for SignalId {
    #[inline]
    fn cmp(&self, other: &SignalId) -> Ordering {
        match self.sx.cmp(&other.sx) {
            Ordering::Equal => self.id.cmp(&other.id),
            cmp => cmp,
        }
    }
}

impl std::fmt::Debug for SignalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}ˢⁱᵍ", self.sx, self.id.as_u15())
    }
}

#[test]
fn signal_id_size() {
    assert_eq!(std::mem::size_of::<SignalId>(), 8);
}

#[test]
fn signal_id_ordering() {
    let sig1_scope1 = SignalId {
        id: u15Bool::new(1, false),
        sx: NodeId::from(1),
        rt: Runtime::from(4),
    };

    let sig2_scope1 = SignalId {
        id: u15Bool::new(2, false),
        sx: NodeId::from(1),
        rt: Runtime::from(4),
    };

    let sig1_scope2 = SignalId {
        id: u15Bool::new(1, false),
        sx: NodeId::from(2),
        rt: Runtime::from(4),
    };

    let sig2_scope2 = SignalId {
        id: u15Bool::new(2, false),
        sx: NodeId::from(2),
        rt: Runtime::from(4),
    };

    assert!(sig1_scope1 < sig2_scope1);
    assert!(sig1_scope2 < sig2_scope2);
    assert!(sig2_scope1 < sig1_scope2);
}
