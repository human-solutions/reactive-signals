#[cfg(test)]
mod tests;

mod any_func;
mod runtime;
// mod scope;
mod signal;

// use scope::{Scope, ScopeId};

pub use runtime::{Runtime, RuntimeId};
pub use signal::{create_data_signal, create_func_signal, Signal};
