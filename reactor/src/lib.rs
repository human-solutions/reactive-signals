#[cfg(test)]
mod tests;

#[cfg(feature = "profile")]
pub mod profile;

mod iter;
mod macros;
mod primitives;
pub mod runtimes;
mod scope;
mod scope_inner;
mod signal;
mod updater;

use runtimes::Runtime;
pub use scope::Scope;
use scope_inner::ScopeInner;
pub use signal::*;
use std::cell;

#[cfg(not(feature = "unsafe-cell"))]
type CellType<T> = cell::RefCell<T>;
#[cfg(feature = "unsafe-cell")]
type CellType<T> = cell::UnsafeCell<T>;
