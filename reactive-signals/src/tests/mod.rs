pub mod profile;
#[cfg(test)]
mod size_test;

use std::cell::RefCell;

pub struct StringStore(RefCell<Vec<String>>);

impl Default for StringStore {
    fn default() -> Self {
        Self::new()
    }
}

impl StringStore {
    pub fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    pub fn push(&self, value: String) {
        self.0.borrow_mut().push(value);
    }

    pub fn values(&self) -> String {
        self.0
            .borrow()
            .iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
