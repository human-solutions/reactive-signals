use std::hash::Hash;

use crate::{
    primitives::AnyData,
    scope::Scope,
    signals::{Data, EqData, HashEqData, Signal},
};

pub trait HashEqDataKind {
    #[inline]
    fn signal_kind(&self) -> HashEqSignal {
        HashEqSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<T> HashEqDataKind for (Scope, T) where T: Hash + PartialEq + 'static {}

pub trait EqDataKind {
    #[inline]
    fn signal_kind(&self) -> EqSignal {
        EqSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<T> EqDataKind for &(Scope, T) where T: PartialEq + 'static {}

pub trait TrueDataKind {
    #[inline]
    fn signal_kind(&self) -> TrueSignal {
        TrueSignal
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<T> TrueDataKind for &&(Scope, T) where T: 'static {}

pub struct HashEqSignal;

impl HashEqSignal {
    #[inline]
    pub fn new<T>(self, tuple: (Scope, T)) -> Signal<HashEqData<T>>
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
    pub fn new<T>(self, tuple: (Scope, T)) -> Signal<EqData<T>>
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
    pub fn new<T>(self, tuple: (Scope, T)) -> Signal<Data<T>>
    where
        T: 'static,
    {
        let (sx, data) = tuple;
        Signal::data(sx, AnyData::new(Data(data)))
    }
}
