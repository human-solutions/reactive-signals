#![allow(unused_imports)]

mod any_data;
mod dyn_func;
mod u15_bool;
mod u31_bool;

pub(crate) use any_data::AnyData;
pub(crate) use dyn_func::DynFunc;
pub(crate) use u15_bool::u15Bool;
pub(crate) use u31_bool::u31Bool;

#[cfg(not(feature = "use-unsafe"))]
mod signal_set;
#[cfg(not(feature = "use-unsafe"))]
pub(crate) use signal_set_unsafe::SignalSet;

#[cfg(feature = "use-unsafe")]
mod signal_set_unsafe;
#[cfg(feature = "use-unsafe")]
pub(crate) use signal_set_unsafe::SignalSet;
