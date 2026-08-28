#![doc(hidden)]

mod client;
mod data;
mod func;
mod server;

// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization
//
// The `signal!` macro dispatches with `(&&tuple).signal_kind()`. Method resolution
// probes each deref step of the `&&(Scope, T)` receiver, by value before autoref,
// so with the `&self` kind methods the priority order of impl targets is:
//   1. `&(Scope, T)`  — receiver `&&(Scope, T)` matches with zero adjustment
//   2. `&&(Scope, T)` — one autoref
//   3. `(Scope, T)`   — only reached after a deref step
// Kind impls must be arranged on those targets in priority order (Hash+Eq > Eq >
// fallback for data; Eq > fallback for funcs). The dispatch tests in macros.rs
// pin the resolved signal type for every case.

pub use func::{EqFuncKind, TrueFuncKind};

pub use data::{EqDataKind, HashEqDataKind, TrueDataKind};

pub use server::{ServerEqFuncKind, ServerTrueFuncKind};

pub use client::{ClientEqFuncKind, ClientTrueFuncKind};
