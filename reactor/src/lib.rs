//!
//! ## Performance and memory use
//!
//! The measurments below comes from exact measurements that has
//! been rounded for ease of reading and reasoning.
//!
//! | What                     | Heap use | With `unsafe-cell`
//! | ---                      | ---      | ---
//! | Scope                    | 40 bytes | 32 bytes
//! | Signal                   | 80 bytes | 70 bytes
//! | Subscription<sup>*</sup> | 8 bytes  | 12 bytes
//!
//! <sup>*</sup> The memory use for each signal subscription.
//!
//! In leptos_reactive, 1000 signals and one memo uses 400kb and
//! in reactor creating 1000 function signals each with a subscription
//! uses 100kb. In other words reactor uses 4 times less memory than
//! leptos_reactive

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
