use crate::{NodeId, Tree};

pub struct DepthFirstIter<'a, T> {
    tree: &'a Tree<T>,
    start: NodeId,
    /// A node who's children have all been visited
    current: Option<NodeId>,
}

impl<'a, T> DepthFirstIter<'a, T> {
    pub(crate) fn new(tree: &'a Tree<T>, start: NodeId) -> Self {
        Self {
            tree,
            start,
            current: Some(drill_down(tree, start)),
        }
    }
}

fn drill_down<T>(tree: &Tree<T>, mut node: NodeId) -> NodeId {
    while let Some(child) = tree[node].last_child {
        node = child;
    }
    node
}

fn next<T>(
    tree: &Tree<T>,
    start: NodeId,
    mut current: Option<NodeId>,
) -> (Option<NodeId>, Option<NodeId>) {
    let Some(next) = current else {
        return (None, None);
    };

    if let Some(node) = tree[next].prev_sibling {
        // we found a previous sibling, let's start at that sibling's deepest child
        current = Some(drill_down(tree, node));
    } else if next == start {
        return (None, Some(start));
    } else if let Some(parent) = tree[next].parent {
        // there's no previous sibling so we have to go up knowing
        // that all the children of the parent have been visited
        if parent == start {
            // we are at the top
            return (None, Some(parent));
        } else {
            current = Some(parent);
        }
    }
    return (current, Some(next));
}

impl<'a, T> Iterator for DepthFirstIter<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let (current, next) = next(self.tree, self.start, self.current);
        self.current = current;
        return next;
    }
}

pub struct MutDepthFirstIter<'a, T> {
    tree: &'a mut Tree<T>,
    start: NodeId,
    /// A node who's children have all been visited
    current: Option<NodeId>,
}

impl<'a, T> MutDepthFirstIter<'a, T> {
    pub(crate) fn new(tree: &'a mut Tree<T>, start: NodeId) -> Self {
        let current = Some(drill_down(tree, start));
        Self {
            tree,
            start,
            current,
        }
    }
    pub fn for_each(&mut self, f: impl Fn(&mut Tree<T>, NodeId)) {
        while let (current, Some(next)) = next(self.tree, self.start, self.current) {
            f(self.tree, next);
            self.current = current;
        }
    }
}
