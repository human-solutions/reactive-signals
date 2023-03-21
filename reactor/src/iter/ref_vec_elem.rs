use std::{cell::Ref, ops::Deref};

pub(crate) struct RefVecElem<'a, T> {
    pub(crate) idx: usize,
    #[cfg(not(feature = "use-unsafe"))]
    pub(crate) vec: Ref<'a, Vec<T>>,
    #[cfg(feature = "use-unsafe")]
    pub(crate) vec: &'a [T],
}

impl<'a, T> RefVecElem<'a, T> {
    #[cfg(not(feature = "use-unsafe"))]
    pub(crate) fn new(vec: Ref<'a, Vec<T>>, idx: usize) -> Self {
        Self { idx, vec }
    }

    #[cfg(feature = "use-unsafe")]
    pub(crate) fn new(vec: &'a [T], idx: usize) -> Self {
        Self { idx, vec }
    }
}

impl<'a, T> Deref for RefVecElem<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.vec[self.idx]
    }
}
