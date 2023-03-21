use std::cell::RefCell;

use super::ArrVec;

#[derive(Debug)]
pub(crate) struct SignalSet<const N: usize, T: Ord + Eq + Copy>(RefCell<ArrVec<N, T>>);

impl<const N: usize, T: Ord + Eq + Copy> SignalSet<N, T> {
    pub(crate) fn insert(&self, elem: T) {
        let mut vec = self.0.borrow_mut();
        vec.insert(elem);
    }

    pub(crate) fn clear(&self) {
        self.0.borrow_mut().clear()
    }

    pub(crate) fn retain<F: FnMut(&T) -> bool>(&self, f: F) {
        self.0.borrow_mut().retain(f);
    }

    pub(crate) fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    pub(crate) fn get(&self, index: usize) -> T {
        self.0.borrow().get(index)
    }
}

impl<const N: usize, T: Ord + Eq + Copy> Default for SignalSet<N, T> {
    fn default() -> Self {
        Self(RefCell::new(Default::default()))
    }
}

#[test]
fn test_retain() {
    use crate::runtimes::PoolRuntimeId;
    use crate::{primitives::u15Bool, SignalId};
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
