//!
//! Runtimes are the starting point of a reactive-signals based application. Internally, the runtimes presents a
//! simple boolean constant to let the [Scope](crate::Scope)s and [Signal](crate::Signal)s know
//! where they are running. Like that a signal marked with `server` or `client` knows if it should run.
//!
//! There are two types of runtimes:
//!
//! - Pooled runtimes: Allows for many runtimes in a thread.
//! - Single runtimes: Limitied to one runtime per thread.
//!
//! A runtime presents a single function: `new_root_scope()` which returns a root [Scope](crate::Scope).
//! When the root scope is discarded, using it's [discard()](crate::Scope::discard()) function, the
//! runtime is discarded as well.
//!
//! Single runtimes have no memory overhead, whereas pooled runtimes have an overhead of 2 bytes
//! which is the index in the pool. As a consequence a pool can have at most 65k runtimes.
//!
mod inner;

use std::{borrow::BorrowMut, cell::RefCell};

pub use inner::RuntimeInner;

use crate::{signal, Scope};

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Runtime<'rt> {
    pub(crate) inner: &'rt RefCell<RuntimeInner<'rt>>,
}

impl<'rt> Runtime<'rt> {
    pub fn new(inner: &'rt RefCell<RuntimeInner<'rt>>) -> Self {
        Self { inner }
    }

    pub fn new_root_scope(&'rt self) -> Scope {
        let mut rti = self.inner.borrow_mut();
        let sx = rti.scope_tree.init(Default::default());

        Scope { sx, rt: self }
    }
}

// fn run() {
//     let rti = RuntimeInner::new();
//     let rt = Runtime::new(&rti);
//     let root = rt.new_root_scope();

//     let sc0 = root.clone();

//     let num_sig = signal!(sc0, 5);

//     let sc1 = sc0.new_child();
//     let sc2 = sc1.new_child();
//     let sc3 = sc2.new_child();

//     let output = std::rc::Rc::new(StringStore::new());

//     let _str_sig = signal!(sc3, clone: output, move || output
//         .push(format!("val: {}", num_sig.get())));
// }

// pub struct StringStore(RefCell<Vec<String>>);

// impl StringStore {
//     pub fn new() -> Self {
//         Self(RefCell::new(Vec::new()))
//     }

//     pub fn push(&self, value: String) {
//         self.0.borrow_mut().push(value);
//     }

//     pub fn values(&self) -> String {
//         self.0
//             .borrow()
//             .iter()
//             .map(|s| s.to_owned())
//             .collect::<Vec<String>>()
//             .join(", ")
//     }
// }
