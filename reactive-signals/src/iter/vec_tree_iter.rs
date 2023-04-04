use std::{fmt::Debug, mem};

use super::DEBUG;
use crate::iter::{IdVec, IdVecIter};

pub(crate) trait NodeResolver<'a> {
    type Id;
    type Elem;
    fn node(&'a self, id: Self::Id) -> Self::Elem;
}

pub(crate) struct VecTreeIter<'a, R>
where
    R: NodeResolver<'a>,
    R::Elem: IdVec<Output = R::Id>,
{
    resolver: &'a R,
    parents: Vec<IdVecIter<R::Elem>>,
    iter: IdVecIter<R::Elem>,
    /// child iterator that is not empty
    queued_children: Option<IdVecIter<R::Elem>>,
}

impl<'a, R> VecTreeIter<'a, R>
where
    R: NodeResolver<'a>,
    R::Elem: IdVec<Output = R::Id>,
    R::Id: Debug + Copy,
{
    pub(crate) fn new(tree: &'a R, id: R::Id) -> Self {
        let iter = IdVecIter::new(tree.node(id));
        Self {
            resolver: tree,
            parents: Vec::new(),
            iter,
            queued_children: None,
        }
    }

    fn next_and_queue_child(&mut self) -> Option<R::Id> {
        if let Some(next) = self.iter.next() {
            let children = self.resolver.node(next);
            if !children.is_empty() {
                self.queued_children = Some(IdVecIter::new(children));
                DEBUG.then(|| println!("Queued children for [next:?]"));
            }
            Some(next)
        } else {
            None
        }
    }

    pub(crate) fn skip_children(&mut self) {
        self.queued_children = None
    }
}

impl<'a, R> Iterator for VecTreeIter<'a, R>
where
    R: NodeResolver<'a>,
    R::Elem: IdVec<Output = R::Id>,
    R::Id: Debug + Copy,
{
    type Item = R::Id;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut iter) = self.queued_children.take() {
            // swap the current iter for the child iter
            mem::swap(&mut self.iter, &mut iter);
            if iter.has_more() {
                // and push the current iter on the parent vec
                self.parents.push(iter);
            }
            DEBUG.then(|| println!("Switch to child iter"));
        }

        if let Some(next) = self.next_and_queue_child() {
            DEBUG.then(|| println!("[{next:?}]"));
            Some(next)
        } else if let Some(parent_iter) = self.parents.pop() {
            // we know that a parent is only queued if it is not empty
            self.iter = parent_iter;

            let next = self
                .next_and_queue_child()
                .expect("BUG: An empty parent iterator was queued");

            DEBUG.then(|| println!("[{next:?}] Parent"));
            return Some(next);
        } else {
            DEBUG.then(|| println!("Stop"));
            None
        }
    }
}
