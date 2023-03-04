use crate::{NodeId, Tree};

pub struct DepthFirstIter<'a, T> {
    tree: &'a Tree<T>,
    start: NodeId,
    /// A node who's children have all been visited
    next: Option<NodeId>,
    /// Node that should not be visited (including it's children)
    skip: Option<NodeId>,
}

impl<'a, T> DepthFirstIter<'a, T> {
    pub(crate) fn new(tree: &'a Tree<T>, start: NodeId, skip: Option<NodeId>) -> Self {
        let next = Some(drill_down(tree, start, skip));
        DEBUG.then(|| println!("Start: {start:?}, Next: {next:?}"));
        Self {
            tree,
            start,
            next,
            skip,
        }
    }
}

impl<'a, T> Iterator for DepthFirstIter<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let (next, current) = next(self.tree, self.start, self.next, self.skip);
        self.next = next;
        return current;
    }
}

fn drill_down<T>(tree: &Tree<T>, mut node: NodeId, skip: Option<NodeId>) -> NodeId {
    while let Some(child) = tree.nodes[node.index()].last_child {
        node = child;
        if Some(child) == skip {
            break;
        }
    }
    node
}

const DEBUG: bool = false;

fn next<T>(
    tree: &Tree<T>,
    start: NodeId,
    mut next: Option<NodeId>,
    skip: Option<NodeId>,
) -> (Option<NodeId>, Option<NodeId>) {
    let Some(mut current) = next else {
        DEBUG.then(|| println!("[STOP]. Current is none"));
        return (None, None);
    };

    if Some(current) == skip {
        if let Some(prev) = tree.nodes[current.index()].prev_sibling {
            DEBUG.then(|| print!("[Skip {current:?} to prev {prev:?}]"));
            current = drill_down(tree, prev, skip);
        } else if let Some(parent) = tree.nodes[current.index()].parent {
            DEBUG.then(|| print!("[Skip {current:?} to parent {parent:?}]"));
            current = parent;
        } else {
            DEBUG.then(|| println!("[STOP]. Current is skipped and has no parent or prev sibling"));
            return (None, None);
        }
    } else {
        DEBUG.then(|| print!("[{current:?}] "));
    }

    if current == start {
        DEBUG.then(|| println!("Stop next. Current same as start: {start:?}"));
        return (None, Some(start));
    } else if let Some(prev) = tree.nodes[current.index()].prev_sibling {
        // we found a previous sibling, let's start at that sibling's deepest child
        next = Some(drill_down(tree, prev, skip));
        DEBUG.then(|| println!("Next is prev siblings deepest child: {next:?}"));
    } else if let Some(parent) = tree.nodes[current.index()].parent {
        // there's no previous sibling so we have to go up knowing
        // that all the children of the parent have been visited
        next = Some(parent);
        DEBUG.then(|| println!("Next is parent: {next:?}"));
    } else {
        panic!("BUG: iteration ended up in a state that should be impossible");
    }

    return (next, Some(current));
}

pub struct MutDepthFirstIter<'a, T> {
    tree: &'a mut Tree<T>,
    start: NodeId,
    /// A node who's children have all been visited
    next: Option<NodeId>,
    skip: Option<NodeId>,
}

impl<'a, T> MutDepthFirstIter<'a, T> {
    pub(crate) fn new(tree: &'a mut Tree<T>, start: NodeId, skip: Option<NodeId>) -> Self {
        let next = Some(drill_down(tree, start, skip));
        DEBUG.then(|| println!("Start: {start:?}, Next: {next:?}"));
        Self {
            tree,
            start,
            next,
            skip,
        }
    }

    pub fn for_each(&mut self, f: impl Fn(&mut Tree<T>, NodeId)) {
        while let (next, Some(current)) = next(self.tree, self.start, self.next, self.skip) {
            self.next = next;
            f(self.tree, current);
        }
    }

    pub fn fold<Acc>(&mut self, mut acc: Acc, f: impl Fn(&mut Tree<T>, Acc, NodeId) -> Acc) -> Acc {
        while let (next, Some(current)) = next(self.tree, self.start, self.next, self.skip) {
            self.next = next;
            acc = f(self.tree, acc, current);
        }
        acc
    }
}
