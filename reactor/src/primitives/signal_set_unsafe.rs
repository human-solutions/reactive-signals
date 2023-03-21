use std::cell::UnsafeCell;

use super::ArrVec;

#[derive(Debug)]
pub(crate) struct SignalSet<const N: usize, T: Ord + Eq + Copy>(UnsafeCell<ArrVec<N, T>>);

impl<const N: usize, T: Ord + Eq + Copy> SignalSet<N, T> {
    pub(crate) fn insert(&self, elem: T) {
        unsafe {
            let vec: &mut ArrVec<N, T> = &mut *self.0.get();
            vec.insert(elem)
        }
    }

    pub(crate) fn clear(&self) {
        unsafe {
            let vec: &mut ArrVec<N, T> = &mut *self.0.get();
            vec.clear();
        }
    }

    pub(crate) fn retain<F: FnMut(&T) -> bool>(&self, f: F) {
        unsafe {
            let vec: &mut ArrVec<N, T> = &mut *self.0.get();
            vec.retain(f);
        }
    }

    pub(crate) fn len(&self) -> usize {
        unsafe {
            let vec: &ArrVec<N, T> = &*self.0.get();
            vec.len()
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        unsafe {
            let vec: &ArrVec<N, T> = &*self.0.get();
            vec.is_empty()
        }
    }

    pub(crate) fn get(&self, index: usize) -> T {
        unsafe {
            let vec: &ArrVec<N, T> = &*self.0.get();
            vec.get(index)
        }
    }
}

impl<const N: usize, T: Ord + Eq + Copy> Default for SignalSet<N, T> {
    fn default() -> Self {
        Self(UnsafeCell::new(Default::default()))
    }
}

#[test]
fn test_retain() {
    use crate::runtimes::PoolRuntimeId;
    use crate::{primitives::u15Bool, signal_id::SignalId};
    use arena_link_tree::NodeId;

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

    let vec = SignalSet::<3, SignalId<PoolRuntimeId>>::default();
    vec.insert(sig2_scope1);
    vec.insert(sig1_scope2);
    vec.insert(sig1_scope1);
    vec.insert(sig2_scope2);

    assert_eq!(vec.get(0), sig1_scope1);
    assert_eq!(vec.get(1), sig2_scope1);
    assert_eq!(vec.get(2), sig1_scope2);
    assert_eq!(vec.get(3), sig2_scope2);

    vec.retain(|id| id.sx != NodeId::from(1));

    assert_eq!(vec.get(0), sig1_scope2);
    assert_eq!(vec.get(1), sig2_scope2);
}
