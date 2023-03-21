mod arr_vec;
mod dyn_func;
mod u15_bool;

pub(crate) use arr_vec::ArrVec;
pub(crate) use dyn_func::DynFunc;
pub(crate) use u15_bool::u15Bool;

// kept for future use if a feature "large-indexes" is added
// mod u31_bool;
// pub(crate) use u31_bool::u31Bool;

///// any_data ////

#[cfg(not(feature = "use-unsafe"))]
mod any_data;
#[cfg(not(feature = "use-unsafe"))]
pub(crate) use any_data::AnyData;

#[cfg(feature = "use-unsafe")]
mod any_data_unsafe;
#[cfg(feature = "use-unsafe")]
pub(crate) use any_data_unsafe::AnyData;

///// signal_set ////

#[cfg(not(feature = "use-unsafe"))]
mod signal_set;
#[cfg(not(feature = "use-unsafe"))]
pub(crate) use signal_set::SignalSet;

#[cfg(feature = "use-unsafe")]
mod signal_set_unsafe;
#[cfg(feature = "use-unsafe")]
pub(crate) use signal_set_unsafe::SignalSet;
