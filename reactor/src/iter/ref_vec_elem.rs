use std::ops::Deref;

pub(crate) struct RefVecElem<'a, T> {
    pub(crate) idx: usize,
    #[cfg(not(feature = "unsafe-cell"))]
    pub(crate) vec: std::cell::Ref<'a, Vec<T>>,
    #[cfg(feature = "unsafe-cell")]
    pub(crate) vec: &'a [T],
}

impl<'a, T> RefVecElem<'a, T> {
    #[cfg(not(feature = "unsafe-cell"))]
    pub(crate) fn new(vec: std::cell::Ref<'a, Vec<T>>, idx: usize) -> Self {
        Self { idx, vec }
    }

    #[cfg(feature = "unsafe-cell")]
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
