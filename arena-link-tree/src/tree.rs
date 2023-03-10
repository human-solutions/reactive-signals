use crate::{
    availability::NodeSlotAvailability,
    iter::{DepthFirstIter, MutDepthFirstIter},
    Node, NodeId,
};
use bitvec::prelude::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
pub struct Tree<T> {
    pub(crate) initialized: bool,
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
            initialized: self.initialized,
            nodes: self.nodes.clone(),
            availability: Default::default(),
        }
    }
}

impl<T: Default> Default for Tree<T> {
    fn default() -> Self {
        Self::create()
    }
}

impl<T: Default> Tree<T> {
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

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn create() -> Self {
        Self {
            initialized: false,
            nodes: vec![],
            availability: Default::default(),
        }
    }

    pub fn create_and_init(data: T) -> Tree<T> {
        let mut me = Self::create();
        me.init(data);
        me
    }

    pub fn init(&mut self, data: T) -> NodeId {
        debug_assert!(
            !self.initialized,
            "tree already initialized. did you forget to discard it before reusing it?"
        );
        let root_id = self.availability.init();

        self.push(Node::new(data));
        self.initialized = true;
        root_id
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
        debug_assert!(
            self.initialized,
            "cannot add a child to a tree that is not initialized"
        );

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

    fn detach(&mut self, node: NodeId) {
        let Some(parent) = self.nodes[node.index()].parent else {
            return;
        };
        let Some(mut curr_id) = self.nodes[parent.index()].last_child else {
            return;
        };
        let mut prev_id: Option<NodeId> = None;
        // iterate through the parent's children until we find the node
        // we want to remove, together with its previous sibling
        loop {
            match (prev_id, curr_id, self.nodes[curr_id.index()].prev_sibling) {
                (Some(prev), curr, next) if curr == node => {
                    self.nodes[prev.index()].prev_sibling = next;
                    break;
                }
                (None, curr, next) if curr == node => {
                    self.nodes[parent.index()].last_child = next;
                    break;
                }
                (_, _, Some(next)) => {
                    prev_id = Some(curr_id);
                    curr_id = next;
                }
                _ => {}
            }
        }
    }

    pub fn discard_all(&mut self, f: impl Fn(&mut T)) {
        debug_assert!(
            self.initialized,
            "tree cannot be discarded because it is not initialized"
        );
        self.discard(self.root(), f);
        self.availability.discard();
        self.nodes.clear();
        self.initialized = false;
    }

    pub fn discard(&mut self, node: NodeId, reuse_data: impl Fn(&mut T)) -> NodeBitVec {
        self.detach(node);

        let ids = NodeBitVec(bitvec![u32, Msb0; 0; self.nodes.len()]);
        let ids = self.iter_mut_from(node).fold(ids, |tree, mut ids, id| {
            tree.nodes[id.index()].reuse();
            tree.availability.set_available(id);
            reuse_data(&mut tree.nodes[id.index()].data);
            ids.set(id);
            ids
        });
        self.nodes[node.index()].reuse();
        self.availability.set_available(node);
        ids
    }
}

pub struct NodeBitVec(BitVec<u32, Msb0>);

impl NodeBitVec {
    pub(crate) fn set(&mut self, id: NodeId) {
        self.0.set(id.index(), true);
    }
}

impl Index<NodeId> for NodeBitVec {
    type Output = bool;
    fn index(&self, id: NodeId) -> &Self::Output {
        &self.0[id.index()]
    }
}
