#[cfg(test)]
mod tests;

mod any_func;
mod runtime;
// mod scope;
mod signal;
mod signal_inner;

// use scope::{Scope, ScopeId};
use signal::SignalId;
use signal_inner::SignalInner;

pub use runtime::{Runtime, RuntimeId};
pub use signal::{create_data_signal, create_func_signal, Signal};
