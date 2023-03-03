use std::cell::RefCell;

mod scope_test;
mod signal_test;

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
