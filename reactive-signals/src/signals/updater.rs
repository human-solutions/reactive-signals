#![allow(dead_code)]

use std::mem;

use crate::{
    runtime::RuntimeInner,
    scope::{RefVec, ScopeInner},
    signals::SignalId,
    Tree,
};

struct ListenerIter<'a> {
    idx: usize,
    pos: usize,
    len: usize,
    vec: RefVec<'a>,
}

impl<'a: 'a> ListenerIter<'a> {
    fn new(tree: &'a Tree<ScopeInner>, sig: SignalId) -> Self {
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

    fn next(&mut self) -> Option<SignalId> {
        if self.pos < self.len {
            self.pos += 1;
            Some(self.vec[self.idx].listeners.get(self.pos - 1))
        } else {
            None
        }
    }
}

pub(crate) fn propagate_change<'a: 'a>(rt: &'a RuntimeInner, sig: SignalId) {
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

fn next<'a: 'a>(
    tree: &'a Tree<ScopeInner>,
    iter: &mut ListenerIter<'a>,
    parents: &mut Vec<ListenerIter<'a>>,
    queued_children: &mut Option<ListenerIter<'a>>,
) -> Option<SignalId> {
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

fn next_and_queue_child<'a: 'a>(
    tree: &'a Tree<ScopeInner>,
    iter: &mut ListenerIter<'a>,
    queued_children: &mut Option<ListenerIter<'a>>,
) -> Option<SignalId> {
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
