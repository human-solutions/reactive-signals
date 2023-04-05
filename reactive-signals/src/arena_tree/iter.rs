use super::{NodeId, Tree};

pub struct DepthFirstIter<'a, T> {
    tree: &'a Tree<T>,
    start: NodeId,
    /// A node who's children have all been visited
    next: Option<NodeId>,
}

impl<'a, T> DepthFirstIter<'a, T> {
    #[cfg(any(test, feature = "profile"))]
    pub(crate) fn new(tree: &'a Tree<T>, start: NodeId) -> Self {
        let next = Some(drill_down(tree, start));
        DEBUG.then(|| println!("Start: {start:?}, Next: {next:?}"));
        Self { tree, start, next }
    }
}

impl<'a, T> Iterator for DepthFirstIter<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let (next, current) = next(self.tree, self.start, self.next);
        self.next = next;
        current
    }
}

fn drill_down<T>(tree: &Tree<T>, mut node: NodeId) -> NodeId {
    while let Some(child) = tree.nodes[node.index()].last_child {
        node = child;
    }
    node
}

const DEBUG: bool = false;

fn next<T>(
    tree: &Tree<T>,
    start: NodeId,
    mut next: Option<NodeId>,
) -> (Option<NodeId>, Option<NodeId>) {
    let Some( current) = next else {
        DEBUG.then(|| println!("[STOP]. Current is none"));
        return (None, None);
    };

    DEBUG.then(|| print!("[{current:?}] "));

    if current == start {
        DEBUG.then(|| println!("Stop next. Current same as start: {start:?}"));
        return (None, Some(start));
    } else if let Some(prev) = tree.nodes[current.index()].prev_sibling {
        // we found a previous sibling, let's start at that sibling's deepest child
        next = Some(drill_down(tree, prev));
        DEBUG.then(|| println!("Next is prev siblings deepest child: {next:?}"));
    } else if let Some(parent) = tree.nodes[current.index()].parent {
        // there's no previous sibling so we have to go up knowing
        // that all the children of the parent have been visited
        next = Some(parent);
        DEBUG.then(|| println!("Next is parent: {next:?}"));
    } else {
        panic!("BUG: iteration ended up in a state that should be impossible");
    }

    (next, Some(current))
}

pub struct MutDepthFirstIter<'a, T> {
    tree: &'a mut Tree<T>,
    start: NodeId,
    /// A node who's children have all been visited
    next: Option<NodeId>,
}

impl<'a, T> MutDepthFirstIter<'a, T> {
    pub(crate) fn new(tree: &'a mut Tree<T>, start: NodeId) -> Self {
        let next = Some(drill_down(tree, start));
        DEBUG.then(|| println!("Start: {start:?}, Next: {next:?}"));
        Self { tree, start, next }
    }

    pub fn for_each(&mut self, f: impl Fn(&mut Tree<T>, NodeId)) {
        while let (next, Some(current)) = next(self.tree, self.start, self.next) {
            self.next = next;
            f(self.tree, current);
        }
    }

    pub fn fold<Acc>(&mut self, mut acc: Acc, f: impl Fn(&mut Tree<T>, Acc, NodeId) -> Acc) -> Acc {
        while let (next, Some(current)) = next(self.tree, self.start, self.next) {
            self.next = next;
            acc = f(self.tree, acc, current);
        }
        acc
    }
}
