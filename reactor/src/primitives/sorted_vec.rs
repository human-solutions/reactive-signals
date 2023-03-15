use std::{cell::RefCell, ops::Index};

#[cfg_attr(feature = "extra-traits", derive(Debug))]

pub(crate) struct SortedVec<T: Ord>(RefCell<Vec<T>>);

impl<T: Ord + Copy> SortedVec<T> {
    pub(crate) fn insert(&self, elem: T) {
        let mut vec = self.0.borrow_mut();
        match vec.binary_search(&elem) {
            Ok(_) => {} // already present
            Err(index) => vec.insert(index, elem),
        }
    }

    pub(crate) fn clear(&self) {
        self.0.borrow_mut().clear()
    }

    pub(crate) fn retain<F: FnMut(&T) -> bool>(&self, f: F) {
        let mut vec = self.0.borrow_mut();
        vec.retain(f);
        // TODO might leave the vec in un-ordered state
        // check that it is still ordered
    }

    pub(crate) fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    pub(crate) fn get(&self, index: usize) -> T {
        self.0.borrow()[index]
    }
}

impl<T: Ord> Default for SortedVec<T> {
    fn default() -> Self {
        Self(RefCell::new(Vec::new()))
    }
}
