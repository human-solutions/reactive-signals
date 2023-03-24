//!
//! ## Performance
//!
//! These measurements has been produced using [criterion](https://crates.io/crates/criterion) by measuring on
//! 1000 instances and calculating the time for one. They have been rounded for ease of reading and reasoning.
//! It has been measured on a Macbook M1.
//!
//! | What                | Time  | With `unsafe-cell`
//! | ---                 | ---   | ---
//! | Create a Scope      | 10 ns |  8 ns
//! | Create a Signal     | 55 ns | 50 ns
//! | Notify a subscriber | 25 ns | 15 ns
//!
//! The leptos_reactive profiling example "Leptos create 1000 signals" measures 245 µs.
//! The same measures 70 µs using reactor. That makes for a 3,5 times improvement.
//!
//! ## Memory use
//!
//! These measurements has been produced using [dhat](https://crates.io/crates/dhat) by creating
//! 1000 instances and calculating the size of one. They have been rounded for ease of reading and reasoning.
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
//!
//! Please see the benches, examples and tests for full details.
//!

#[cfg(any(test, feature = "profile"))]
pub mod tests;

mod iter;
mod macros;
mod primitives;
pub mod runtimes;
mod scope;
mod signals;

#[doc(hidden)]
pub use signals::kinds::*;

pub use crate::signals::Signal;
use runtimes::Runtime;
pub use scope::Scope;
use scope::ScopeInner;
use std::cell;

#[cfg(not(feature = "unsafe-cell"))]
type CellType<T> = cell::RefCell<T>;
#[cfg(feature = "unsafe-cell")]
type CellType<T> = cell::UnsafeCell<T>;
