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

// Does not require any autoref if called as (&error).datakind().
impl<T, RT: Runtime> HashEqDataKind for (Scope<RT>, T) where T: Hash + PartialEq + 'static {}

pub trait EqDataKind {
    #[inline]
    fn signal_kind(&self) -> EqSignal {
        EqSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<T, RT: Runtime> EqDataKind for &(Scope<RT>, T) where T: PartialEq + 'static {}

pub trait TrueDataKind {
    #[inline]
    fn signal_kind(&self) -> TrueSignal {
        TrueSignal
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<T, RT: Runtime> TrueDataKind for &&(Scope<RT>, T) where T: 'static {}

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
