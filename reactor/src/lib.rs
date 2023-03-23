#[cfg(any(test, feature = "profile"))]
pub mod tests;

mod iter;
mod macros;
mod primitives;
pub mod runtimes;
mod scope;
mod signal;

use runtimes::Runtime;
pub use scope::Scope;
use scope::ScopeInner;
pub use signal::*;
use std::cell;

#[cfg(not(feature = "unsafe-cell"))]
type CellType<T> = cell::RefCell<T>;
#[cfg(feature = "unsafe-cell")]
type CellType<T> = cell::UnsafeCell<T>;
