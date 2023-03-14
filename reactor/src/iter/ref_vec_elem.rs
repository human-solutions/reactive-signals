use std::{cell::Ref, ops::Deref};

pub(crate) struct RefVecElem<'a, T> {
    pub(crate) idx: usize,
    pub(crate) vec: Ref<'a, Vec<T>>,
}

impl<'a, T> RefVecElem<'a, T> {
    pub(crate) fn new(vec: Ref<'a, Vec<T>>, idx: usize) -> Self {
        Self { idx, vec }
    }
}

impl<'a, T> Deref for RefVecElem<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.vec[self.idx]
    }
}
