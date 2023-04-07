#![doc(hidden)]

mod client;
mod data;
mod func;
mod server;

// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

pub use func::{EqFuncKind, TrueFuncKind};

pub use data::{EqDataKind, HashEqDataKind, TrueDataKind};

pub use server::{ServerEqFuncKind, ServerTrueFuncKind};

pub use client::{ClientEqFuncKind, ClientTrueFuncKind};
