use crate::NodeId;

#[derive(Default)]
pub struct Node<T> {
    pub data: T,
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
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
            first_child: None,
            next_sibling: None,
        }
    }

    pub fn reset(&mut self) {
        self.parent = None;
        self.first_child = None;
        self.next_sibling = None;
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            parent: self.parent,
            first_child: self.first_child,
            next_sibling: self.next_sibling,
        }
    }
}

#[test]
fn test_node_size() {
    let node = Node::<u32>::default();
    assert_eq!(std::mem::size_of_val(&node), 16);
}
