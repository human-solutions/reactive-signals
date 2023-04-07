#![allow(dead_code)]

use std::{cell::Ref, mem};

use crate::{
    runtimes::RuntimeInner,
    scope::ScopeInner,
    signals::{SignalId, SignalInner},
    Tree,
};

struct ListenerIter<'a, 'rt> {
    idx: usize,
    pos: usize,
    len: usize,
    vec: Ref<'a, Vec<SignalInner<'rt>>>,
}

impl<'a, 'rt: 'a> ListenerIter<'a, 'rt> {
    fn new(tree: &'a Tree<ScopeInner<'rt>>, sig: SignalId<'rt>) -> Self {
        let vec = tree[sig.sx].vec_ref();
        let idx = sig.id.as_usize();
        let len = vec[idx].listeners.len();
        Self {
            idx,
            pos: 0,
            len,
            vec,
        }
    }

    fn has_more(&self) -> bool {
        self.pos + 1 < self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn next(&mut self) -> Option<SignalId<'rt>> {
        if self.pos < self.len {
            self.pos += 1;
            Some(self.vec[self.idx].listeners.get(self.pos - 1))
        } else {
            None
        }
    }
}

pub(crate) fn propagate_change<'a, 'rt: 'a>(rt: &'a RuntimeInner<'rt>, sig: SignalId<'rt>) {
    let tree = &rt.scope_tree;

    let mut iter: ListenerIter = ListenerIter::new(tree, sig);
    let mut parents: Vec<ListenerIter> = Vec::new();
    let mut queued_children: Option<ListenerIter> = None;

    while let Some(next) = next(tree, &mut iter, &mut parents, &mut queued_children) {
        let vec = tree[next.sx].vec_ref();
        if !vec[next.id.as_usize()].run(rt, next) {
            queued_children = None;
        }
    }
}

fn next<'a, 'rt: 'a>(
    tree: &'a Tree<ScopeInner<'rt>>,
    iter: &mut ListenerIter<'a, 'rt>,
    parents: &mut Vec<ListenerIter<'a, 'rt>>,
    queued_children: &mut Option<ListenerIter<'a, 'rt>>,
) -> Option<SignalId<'rt>> {
    if let Some(child_iter) = queued_children.take() {
        if iter.has_more() {
            parents.push(mem::replace(iter, child_iter));
        } else {
            *iter = child_iter;
        }
    }
    if let Some(next) = next_and_queue_child(tree, iter, queued_children) {
        Some(next)
    } else if let Some(parent_iter) = parents.pop() {
        // we know that a parent is only queued if it is not empty
        *iter = parent_iter;

        let next = next_and_queue_child(tree, iter, queued_children)
            .expect("BUG: An empty parent iterator was queued");

        return Some(next);
    } else {
        None
    }
}

fn next_and_queue_child<'a, 'rt: 'a>(
    tree: &'a Tree<ScopeInner<'rt>>,
    iter: &mut ListenerIter<'a, 'rt>,
    queued_children: &mut Option<ListenerIter<'a, 'rt>>,
) -> Option<SignalId<'rt>> {
    if let Some(next) = iter.next() {
        let children = ListenerIter::new(tree, next);
        if !children.is_empty() {
            *queued_children = Some(children);
        }
        Some(next)
    } else {
        None
    }
}
