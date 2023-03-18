use std::{cmp::Ordering, hash::Hash};

use arena_link_tree::NodeId;

use crate::{primitives::u15Bool, runtimes::RuntimeInner, scope::Scope, Runtime};

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
pub(crate) struct SignalId<RT: Runtime> {
    pub(crate) id: u15Bool,
    pub(crate) sx: NodeId,
    pub(crate) rt: RT,
}

impl<RT: Runtime> SignalId<RT> {
    pub(crate) fn new(id: usize, sx: Scope<RT>) -> Self {
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
        F: FnOnce(&RuntimeInner<RT>) -> T,
    {
        self.rt.with_ref(f)
    }
}

impl<RT: Runtime> PartialEq for SignalId<RT> {
    #[inline]
    fn eq(&self, other: &SignalId<RT>) -> bool {
        self.id == other.id && self.sx == other.sx
    }
}

impl<RT: Runtime> Eq for SignalId<RT> {}

// ordering by NodeId (Scope) and then id. The runtime is not considered
// as it is assumed to be the same for all SignalId's running on the same
// thread
impl<RT: Runtime> PartialOrd for SignalId<RT> {
    #[inline]
    fn partial_cmp(&self, other: &SignalId<RT>) -> Option<Ordering> {
        match self.sx.partial_cmp(&other.sx) {
            Some(Ordering::Equal) => self.id.partial_cmp(&other.id),
            cmp => cmp,
        }
    }
}

impl<RT: Runtime> Ord for SignalId<RT> {
    #[inline]
    fn cmp(&self, other: &SignalId<RT>) -> Ordering {
        match self.sx.cmp(&other.sx) {
            Ordering::Equal => self.id.cmp(&other.id),
            cmp => cmp,
        }
    }
}

impl<RT: Runtime> std::fmt::Debug for SignalId<RT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}ˢⁱᵍ", self.sx, self.id.as_u15())
    }
}

impl<RT: Runtime> Hash for SignalId<RT> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // only writes once to the Hasher in order to be compatible
        // with nohash-hasher
        state.write_u32(hash(&self))
    }
}

#[inline]
fn hash<RT: Runtime>(me: &SignalId<RT>) -> u32 {
    ((me.sx.as_raw() as u32) << 16) | (me.id.as_u15() as u32)
}

#[test]
fn test_hash() {
    use crate::runtimes::SingleRuntimeId;
    let sig1 = SignalId {
        id: u15Bool::new(u15Bool::MAX as usize, false),
        sx: NodeId::from(0),
        rt: SingleRuntimeId,
    };

    assert_eq!(sig1.sx.as_raw() as u32, 0x1);
    assert_eq!(sig1.id.as_u15(), 0x7FFE);
    assert_eq!(((sig1.sx.as_raw() as u32) << 16), 0x10000);
    assert_eq!(hash(&sig1), 0x17FFE);

    let sig2 = SignalId {
        id: u15Bool::new(1, false),
        sx: NodeId::from(u16::MAX as usize - 2),
        rt: SingleRuntimeId,
    };

    assert_eq!(sig2.sx.as_raw() as u32, 0xFFFE);
    assert_eq!(sig2.id.as_u15(), 0x1);
    assert_eq!(((sig2.sx.as_raw() as u32) << 16), 0xFFFE0000);
    assert_eq!(hash(&sig2), 0xFFFE0001);
}

#[test]
fn signal_id_size() {
    use crate::runtimes::{PoolRuntimeId, SingleRuntimeId};

    assert_eq!(std::mem::size_of::<SignalId<PoolRuntimeId>>(), 8);
    assert_eq!(std::mem::size_of::<SignalId<SingleRuntimeId>>(), 4);
}

#[test]
fn signal_id_ordering() {
    use crate::runtimes::PoolRuntimeId;

    let sig1_scope1 = SignalId {
        id: u15Bool::new(1, false),
        sx: NodeId::from(1),
        rt: PoolRuntimeId::from(4),
    };

    let sig2_scope1 = SignalId {
        id: u15Bool::new(2, false),
        sx: NodeId::from(1),
        rt: PoolRuntimeId::from(4),
    };

    let sig1_scope2 = SignalId {
        id: u15Bool::new(1, false),
        sx: NodeId::from(2),
        rt: PoolRuntimeId::from(4),
    };

    let sig2_scope2 = SignalId {
        id: u15Bool::new(2, false),
        sx: NodeId::from(2),
        rt: PoolRuntimeId::from(4),
    };

    assert!(sig1_scope1 < sig2_scope1);
    assert!(sig1_scope2 < sig2_scope2);
    assert!(sig2_scope1 < sig1_scope2);
}
