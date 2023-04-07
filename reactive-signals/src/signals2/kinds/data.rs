use std::hash::Hash;

use crate::{
    primitives::any_data2::AnyData,
    scope2::Scope,
    signals2::{Data, EqData, HashEqData, Signal},
};

pub trait HashEqDataKind {
    #[inline]
    fn signal_kind(&self) -> HashEqSignal {
        HashEqSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<'rt, T> HashEqDataKind for (Scope<'rt>, T) where T: Hash + PartialEq + 'static {}

pub trait EqDataKind {
    #[inline]
    fn signal_kind(&self) -> EqSignal {
        EqSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<'rt, T> EqDataKind for &(Scope<'rt>, T) where T: PartialEq + 'static {}

pub trait TrueDataKind {
    #[inline]
    fn signal_kind(&self) -> TrueSignal {
        TrueSignal
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<'rt, T> TrueDataKind for &&(Scope<'rt>, T) where T: 'static {}

pub struct HashEqSignal;

impl HashEqSignal {
    #[inline]
    pub fn new<'rt, T>(self, tuple: (Scope<'rt>, T)) -> Signal<HashEqData<T>>
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
    pub fn new<'rt, T>(self, tuple: (Scope<'rt>, T)) -> Signal<EqData<T>>
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
    pub fn new<'rt, T>(self, tuple: (Scope<'rt>, T)) -> Signal<Data<T>>
    where
        T: 'static,
    {
        let (sx, data) = tuple;
        Signal::data(sx, AnyData::new(Data(data)))
    }
}
