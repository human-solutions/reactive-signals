#![allow(dead_code)]

use std::hash::Hash;

use crate::{
    primitives::{Data, EqData},
    runtimes::Runtime,
    Scope, Signal,
};

// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

pub trait EqFuncKind {
    #[inline]
    fn signal_kind(&self) -> EqFunc {
        EqFunc
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<F, T, RT: Runtime> EqFuncKind for (Scope<RT>, F)
where
    F: Fn() -> T + 'static,
    T: PartialEq + 'static,
{
}

pub trait TrueFuncKind {
    #[inline]
    fn signal_kind(&self) -> TrueFunc {
        TrueFunc
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<F, T, RT: Runtime> TrueFuncKind for &(Scope<RT>, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct EqFunc;

impl EqFunc {
    #[inline]
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<EqData<T>, RT>
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let (sx, func) = tuple;
        Signal::new_func_eq(sx, func)
    }
}
pub struct TrueFunc;

impl TrueFunc {
    #[inline]
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<Data<T>, RT>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        crate::Signal::new_func(sx, func)
    }
}

// ====== DATA =======

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
    pub fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<EqData<T>, RT>
    where
        T: Hash + PartialEq + 'static,
    {
        let (sx, value) = tuple;
        Signal::new_data_hash_eq(sx, value)
    }
}

pub struct EqSignal;

impl EqSignal {
    #[inline]
    pub fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<EqData<T>, RT>
    where
        T: PartialEq + 'static,
    {
        let (sx, value) = tuple;
        Signal::new_data_eq(sx, value)
    }
}
pub struct TrueSignal;

impl TrueSignal {
    #[inline]
    pub fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<Data<T>, RT>
    where
        T: 'static,
    {
        let (sx, value) = tuple;
        Signal::new_data(sx, value)
    }
}
