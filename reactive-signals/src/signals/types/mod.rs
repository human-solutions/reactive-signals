//!
//! Simple zero-cost abstractions that classifies signals based on the values they produce
//!
mod client;
mod data;
mod func;
mod server;

pub use client::*;
pub use data::*;
pub use func::*;
pub use server::*;

#[doc(hidden)]
pub trait SignalType: 'static {
    type Inner;

    fn is_eq(&self, _other: &Self::Inner) -> bool {
        false
    }
    fn opt_hash(&self) -> Option<u64> {
        None
    }

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
    fn new(value: Self::Inner) -> Self;
}

#[doc(hidden)]
pub trait Modifiable {}

#[doc(hidden)]
pub trait Readable {}

#[doc(hidden)]
pub trait OptReadable {
    const RUN_ON_SERVER: bool = true;
    const RUN_ON_CLIENT: bool = true;

    fn should_run(client_side: bool) -> bool {
        client_side && Self::RUN_ON_CLIENT || !client_side && Self::RUN_ON_SERVER
    }
}

#[cfg(test)]
fn set<T: 'static + SignalType>(val1: &T, val2: &T::Inner) -> bool {
    val1.is_eq(&val2)
}

#[test]
fn cmp_test() {
    use crate::signals::{Data, EqData};

    let d1 = Data(3);
    let d2 = Data(2);

    assert_eq!(set(&d1, &d2.inner()), false);
    assert_eq!(set(&d1, &d1.inner()), false);

    let d1 = EqData(3);
    let d2 = EqData(2);

    assert_eq!(set(&d1, &d2.inner()), false);
    assert_eq!(set(&d1, &d1.inner()), true);
}
