use crate::CellType;

use super::ArrVec;

#[derive(Debug)]
pub(crate) struct SignalSet<const N: usize, T: Ord + Eq + Copy>(CellType<ArrVec<N, T>>);

impl<const N: usize, T: Ord + Eq + Copy> SignalSet<N, T> {
    pub(crate) fn insert(&self, elem: T) {
        self.vec_mut().insert(elem);
    }

    pub(crate) fn clear(&self) {
        self.vec_mut().clear()
    }

    pub(crate) fn retain<F: FnMut(&T) -> bool>(&self, f: F) {
        self.vec_mut().retain(f);
    }

    pub(crate) fn len(&self) -> usize {
        self.vec_ref().len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.vec_ref().is_empty()
    }

    pub(crate) fn get(&self, index: usize) -> T {
        self.vec_ref().get(index)
    }
}

#[cfg(not(feature = "unsafe-cell"))]
impl<const N: usize, T: Ord + Eq + Copy> SignalSet<N, T> {
    #[inline]
    fn vec_mut(&self) -> std::cell::RefMut<ArrVec<N, T>> {
        self.0.borrow_mut()
    }

    #[inline]
    fn vec_ref(&self) -> std::cell::Ref<ArrVec<N, T>> {
        self.0.borrow()
    }
}

#[cfg(feature = "unsafe-cell")]
impl<const N: usize, T: Ord + Eq + Copy> SignalSet<N, T> {
    #[inline]
    fn vec_mut(&self) -> &mut ArrVec<N, T> {
        unsafe { &mut *self.0.get() }
    }

    #[inline]
    fn vec_ref(&self) -> &ArrVec<N, T> {
        unsafe { &*self.0.get() }
    }
}

impl<const N: usize, T: Ord + Eq + Copy> Default for SignalSet<N, T> {
    fn default() -> Self {
        return Self(CellType::new(Default::default()));
    }
}

#[test]
fn test_retain() {
    use crate::primitives::u15Bool;
    use crate::runtimes::ServerRuntime;
    use crate::signal::SignalId;
    use arena_link_tree::NodeId;

    let sig1_scope1 = SignalId {
        id: u15Bool::new(1, false),
        sx: NodeId::from(1),
        rt: ServerRuntime::from(4),
    };

    let sig2_scope1 = SignalId {
        id: u15Bool::new(2, false),
        sx: NodeId::from(1),
        rt: ServerRuntime::from(4),
    };

    let sig1_scope2 = SignalId {
        id: u15Bool::new(1, false),
        sx: NodeId::from(2),
        rt: ServerRuntime::from(4),
    };

    let sig2_scope2 = SignalId {
        id: u15Bool::new(2, false),
        sx: NodeId::from(2),
        rt: ServerRuntime::from(4),
    };

    let vec = SignalSet::<3, SignalId<ServerRuntime>>::default();
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
