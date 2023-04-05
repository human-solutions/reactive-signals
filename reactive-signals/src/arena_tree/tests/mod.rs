use std::{cell::RefCell, fmt::Display};

use super::Tree;

mod deep;
mod iter;
mod reuse_ids;
mod reuse_tree;
mod sub_tree;
mod three_children;
mod wide;

impl<T: Display> Tree<T> {
    pub fn dump_used(&self) -> String {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(i, n)| n.parent.is_some() || *i == 0)
            .map(|(i, n)| format!("[{i}] {}", n.data.to_string()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

struct StringStore(RefCell<Vec<String>>);

impl StringStore {
    fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    fn push(&self, value: String) {
        self.0.borrow_mut().push(value);
    }

    fn values(&self) -> String {
        self.0
            .borrow()
            .iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
