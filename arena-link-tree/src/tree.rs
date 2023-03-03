use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::{
    availability::NodeSlotAvailability,
    iter::{DepthFirstIter, MutDepthFirstIter},
    Node, NodeId,
};

#[derive(Debug)]
pub struct Tree<T> {
    pub(crate) nodes: Vec<Node<T>>,
    pub(crate) availability: NodeSlotAvailability,
}

impl<T> Index<NodeId> for Tree<T> {
    type Output = T;
    fn index(&self, id: NodeId) -> &Self::Output {
        &self.nodes[id.index()].data
    }
}

impl<T> IndexMut<NodeId> for Tree<T> {
    fn index_mut(&mut self, id: NodeId) -> &mut Self::Output {
        &mut self.nodes[id.index()].data
    }
}

impl<T> Deref for Tree<T> {
    type Target = Vec<Node<T>>;
    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl<T> DerefMut for Tree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nodes
    }
}

impl<T: Clone> Clone for Tree<T> {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            availability: Default::default(),
        }
    }
}

impl<T: Default> Tree<T> {
    /// A node doesn't have the id of the next child
    /// so we have to find it by iterating over the parent's children
    fn next_child(&self, id: NodeId) -> Option<NodeId> {
        let parent = self.nodes[id.index()].parent?;
        let mut child = self.nodes[parent.index()].last_child?;
        while let Some(prev) = self.nodes[child.index()].prev_sibling {
            if prev == id {
                return Some(child);
            }
            child = prev;
        }
        None
    }

    fn add_node(&mut self) -> NodeId {
        if let Some(id) = self.availability.get_available(&self.nodes) {
            debug_assert!(
                !self.nodes[id.index()].is_used(),
                "BUG: node {} is already used",
                id.index()
            );
            return id;
        } else {
            let idx = self.len();
            if idx > NodeId::max() {
                panic!("too many nodes");
            }
            self.push(Default::default());
            idx.into()
        }
    }

    pub fn new_with_root(data: T) -> Self {
        let node = Node::new(data);
        Self {
            nodes: vec![node],
            availability: Default::default(),
        }
    }

    pub fn discard_all(&mut self) {
        self.reset(self.root());
    }

    pub fn root(&self) -> NodeId {
        NodeId::root()
    }

    pub fn iter_from(&self, id: NodeId) -> DepthFirstIter<T> {
        DepthFirstIter::new(self, id)
    }

    pub fn iter_mut_from(&mut self, id: NodeId) -> MutDepthFirstIter<T> {
        MutDepthFirstIter::new(self, id)
    }

    pub fn add_child(&mut self, to: NodeId, data: T) -> NodeId {
        let prev_sibling = self.nodes[to.index()].last_child;
        let new_id = self.add_node();
        {
            let node = &mut self.nodes[new_id.index()];
            node.data = data;
            node.parent = Some(to);
            node.prev_sibling = prev_sibling;
        }

        self.nodes[to.index()].last_child = Some(new_id);
        new_id
    }

    fn detach_child(&mut self, parent: NodeId, remove: NodeId) -> bool {
        let Some(mut curr_id) = self.nodes[parent.index()].last_child else {
          return false;
        };
        let mut prev_id: Option<NodeId> = None;
        loop {
            match (prev_id, curr_id, self.nodes[curr_id.index()].prev_sibling) {
                (Some(prev), curr, next) if curr == remove => {
                    self.nodes[prev.index()].prev_sibling = next;
                    return true;
                }
                (None, curr, next) if curr == remove => {
                    self.nodes[parent.index()].last_child = next;
                    return true;
                }
                (_, _, Some(next)) => {
                    prev_id = Some(curr_id);
                    curr_id = next;
                }
                _ => {}
            }
        }
    }

    pub fn reset(&mut self, node: NodeId) {
        if let Some(first_child) = self.nodes[node.index()].last_child {
            let mut stack = vec![first_child];
            while let Some(id) = stack.pop() {
                if let Some(next_sibling) = self.nodes[id.index()].prev_sibling {
                    stack.push(next_sibling);
                }
                if let Some(first_child) = self.nodes[id.index()].last_child {
                    stack.push(first_child);
                }
                self.nodes[id.index()].reset();
                self.availability.set_available(id);
            }
        }

        if let Some(previous_child) = self.next_child(node) {
            self.nodes[previous_child.index()].prev_sibling = self.nodes[node.index()].prev_sibling;
        } else if let Some(parent_id) = self.nodes[node.index()].parent {
            if !self.detach_child(parent_id, node) {
                panic!("BUG")
            }
        }
        self.nodes[node.index()].reset();
        self.availability.set_available(node);
    }
}
