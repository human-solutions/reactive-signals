use std::fmt::Display;

use crate::Tree;

mod deep;
mod reuse_ids;
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
