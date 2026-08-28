// `fn new(self, ..) -> Signal` is the `signal!` macro's autoref-specialization dispatch
// (`(&&tuple).signal_kind().new(tuple)`), not a conventional constructor — the `self`
// receiver and non-`Self` return are deliberate.
#![allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]

use std::hash::Hash;

use crate::{
    primitives::AnyData,
    runtimes::Runtime,
    signals::{Data, EqData, HashEqData},
    Scope, Signal,
};

pub trait HashEqDataKind {
    #[inline]
    fn signal_kind(&self) -> HashEqSignal {
        HashEqSignal
    }
}

// Highest priority: with a `&&tuple` receiver, `&self` on `&(Scope, T)` gives a
// receiver type of `&&(Scope, T)` — an exact match needing no adjustment.
impl<T, RT: Runtime> HashEqDataKind for &(Scope<RT>, T) where T: Hash + PartialEq + 'static {}

pub trait EqDataKind {
    #[inline]
    fn signal_kind(&self) -> EqSignal {
        EqSignal
    }
}

// Second priority: needs one autoref of the `&&tuple` receiver.
impl<T, RT: Runtime> EqDataKind for &&(Scope<RT>, T) where T: PartialEq + 'static {}

pub trait TrueDataKind {
    #[inline]
    fn signal_kind(&self) -> TrueSignal {
        TrueSignal
    }
}

// Lowest priority: only reachable after a deref step of the `&&tuple` receiver.
impl<T, RT: Runtime> TrueDataKind for (Scope<RT>, T) where T: 'static {}

pub struct HashEqSignal;

impl HashEqSignal {
    #[inline]
    pub fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<HashEqData<T>, RT>
    where
        T: Hash + PartialEq + 'static,
    {
        let (sx, data) = tuple;
        Signal::data(sx, AnyData::new(HashEqData(data)))
    }
}

pub struct EqSignal;

impl EqSignal {
    #[inline]
    pub fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<EqData<T>, RT>
    where
        T: PartialEq + 'static,
    {
        let (sx, data) = tuple;
        Signal::data(sx, AnyData::new(EqData(data)))
    }
}
pub struct TrueSignal;

impl TrueSignal {
    #[inline]
    pub fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<Data<T>, RT>
    where
        T: 'static,
    {
        let (sx, data) = tuple;
        Signal::data(sx, AnyData::new(Data(data)))
    }
}
