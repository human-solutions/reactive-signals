use std::cell::RefCell;

mod size_test;

pub struct StringStore(RefCell<Vec<String>>);

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
