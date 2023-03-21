#![allow(dead_code)]

use crate::{runtimes::Runtime, Scope, Signal};

// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

pub(crate) trait EqFuncKind {
    #[inline]
    fn func_kind(&self) -> EqFunc {
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

pub(crate) trait TrueFuncKind {
    #[inline]
    fn func_kind(&self) -> TrueFunc {
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

pub(crate) struct EqFunc;

impl EqFunc {
    #[inline]
    pub(crate) fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<T, RT>
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let (sx, func) = tuple;
        Signal::new_func_eq(sx, func)
    }
}
pub(crate) struct TrueFunc;

impl TrueFunc {
    #[inline]
    pub(crate) fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<T, RT>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        crate::Signal::new_func(sx, func)
    }
}

// ====== DATA =======

pub(crate) trait EqDataKind {
    #[inline]
    fn data_kind(&self) -> EqData {
        EqData
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<T, RT: Runtime> EqDataKind for (Scope<RT>, T) where T: PartialEq + 'static {}

pub(crate) trait TrueDataKind {
    #[inline]
    fn func_kind(&self) -> TrueData {
        TrueData
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<T, RT: Runtime> TrueDataKind for &(Scope<RT>, T) where T: 'static {}

pub(crate) struct EqData;

impl EqData {
    #[inline]
    pub(crate) fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<T, RT>
    where
        T: PartialEq + 'static,
    {
        let (sx, value) = tuple;
        Signal::new_data_eq(sx, value)
    }
}
pub(crate) struct TrueData;

impl TrueData {
    #[inline]
    pub(crate) fn new<T, RT: Runtime>(self, tuple: (Scope<RT>, T)) -> Signal<T, RT>
    where
        T: 'static,
    {
        let (sx, value) = tuple;
        Signal::new_data(sx, value)
    }
}
