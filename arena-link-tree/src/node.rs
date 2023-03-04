use std::{fmt, num::NonZeroU16};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(NonZeroU16);

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        Self(unsafe { NonZeroU16::new_unchecked(id as u16 + 1) })
    }
}

impl NodeId {
    pub(crate) fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    pub(crate) fn max() -> usize {
        u16::MAX as usize - 1
    }

    pub(crate) fn root() -> Self {
        Self(unsafe { NonZeroU16::new_unchecked(1) })
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ᴺ", self.index())
    }
}

#[derive(Default, Debug)]
pub struct Node<T> {
    pub data: T,
    pub parent: Option<NodeId>,
    pub last_child: Option<NodeId>,
    pub prev_sibling: Option<NodeId>,
}

impl<T> Node<T> {
    pub fn is_used(&self) -> bool {
        self.parent.is_some()
    }
}
impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            parent: None,
            last_child: None,
            prev_sibling: None,
        }
    }

    pub fn reuse(&mut self) {
        self.parent = None;
        self.last_child = None;
        self.prev_sibling = None;
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            parent: self.parent,
            last_child: self.last_child,
            prev_sibling: self.prev_sibling,
        }
    }
}

#[test]
fn test_node_size() {
    let node = Node::<u32>::default();
    assert_eq!(std::mem::size_of_val(&node), 12);
}
