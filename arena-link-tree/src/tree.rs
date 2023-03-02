use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::{Node, NodeId};

pub struct Tree<T> {
    pub(crate) nodes: Vec<Node<T>>,
}

impl<T> Index<NodeId> for Tree<T> {
    type Output = Node<T>;
    fn index(&self, id: NodeId) -> &Self::Output {
        &self.nodes[id.index()]
    }
}

impl<T> IndexMut<NodeId> for Tree<T> {
    fn index_mut(&mut self, id: NodeId) -> &mut Self::Output {
        &mut self.nodes[id.index()]
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
        }
    }
}

impl<T: Default> Tree<T> {
    fn last_sibling(&self, mut id: NodeId) -> NodeId {
        while let Some(next_sibling) = self[id].next_sibling {
            id = next_sibling;
        }
        id
    }

    fn previous_child(&self, id: NodeId) -> Option<NodeId> {
        let parent = self[id].parent?;
        let mut child = self[parent].first_child?;
        while let Some(next_sibling) = self[child].next_sibling {
            if next_sibling == id {
                return Some(child);
            }
            child = next_sibling;
        }
        None
    }

    fn new_node_id(&mut self) -> NodeId {
        for (idx, node) in self.nodes.iter().enumerate() {
            if idx != 0 && !node.is_used() {
                return idx.into();
            }
        }
        let idx = self.len();
        if idx > NodeId::max() {
            panic!("too many nodes");
        }
        self.push(Default::default());
        idx.into()
    }

    pub fn new_with_root(data: T) -> Self {
        let node = Node::new(data);
        Self { nodes: vec![node] }
    }

    pub fn root(&self) -> NodeId {
        NodeId::root()
    }

    pub fn add_child(&mut self, to: NodeId, data: T) -> NodeId {
        let new_id = self.new_node_id();
        {
            let node = &mut self[new_id];
            node.data = data;
            node.parent = Some(to);
        }
        let last_child = self[to].first_child.map(|id| self.last_sibling(id));

        if let Some(child_id) = last_child {
            self[child_id].next_sibling = Some(new_id);
        } else {
            debug_assert!(self[to].first_child.is_none());
            self[to].first_child = Some(new_id);
        }
        new_id
    }

    pub fn detach_child(&mut self, parent: NodeId, remove: NodeId) -> bool {
        let Some(mut curr_id) = self[parent].first_child else {
          return false;
        };
        let mut prev_id: Option<NodeId> = None;
        loop {
            match (prev_id, curr_id, self[curr_id].next_sibling) {
                (Some(prev), curr, next) if curr == remove => {
                    self[prev].next_sibling = next;
                    return true;
                }
                (None, curr, next) if curr == remove => {
                    self[parent].first_child = next;
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
        if node.is_root() {
            panic!("cannot reset root node");
        }

        if let Some(first_child) = self[node].first_child {
            let mut stack = vec![first_child];
            while let Some(id) = stack.pop() {
                if let Some(next_sibling) = self[id].next_sibling {
                    stack.push(next_sibling);
                }
                if let Some(first_child) = self[id].first_child {
                    stack.push(first_child);
                }
                self[id].reset();
            }
        }

        if let Some(previous_child) = self.previous_child(node) {
            self[previous_child].next_sibling = self[node].next_sibling;
        } else if let Some(parent_id) = self[node].parent {
            if !self.detach_child(parent_id, node) {
                panic!("BUG")
            }
        }
        self[node].reset();
    }
}
