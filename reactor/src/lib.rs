#[cfg(test)]
mod tests;

#[cfg(feature = "profile")]
pub mod profile;

mod iter;
mod macros;
mod primitives;
pub mod runtimes;
mod scope;
mod signal;
mod updater;

use runtimes::Runtime;
pub use scope::Scope;
pub use signal::*;

#[cfg(not(feature = "unsafe-cell"))]
mod scope_inner;
#[cfg(not(feature = "unsafe-cell"))]
use scope_inner::ScopeInner;

#[cfg(feature = "unsafe-cell")]
mod scope_inner_unsafe;
#[cfg(feature = "unsafe-cell")]
use scope_inner_unsafe::ScopeInner;
