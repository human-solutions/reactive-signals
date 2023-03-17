#![allow(dead_code)]

use crate::{Scope, Signal};

// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

pub(crate) trait EqFuncKind {
    #[inline]
    fn func_kind(&self) -> EqFunc {
        EqFunc
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<F, T> EqFuncKind for (Scope, F)
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
impl<F, T> TrueFuncKind for &(Scope, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub(crate) struct EqFunc;

impl EqFunc {
    #[inline]
    pub(crate) fn new<F, T>(self, tuple: (Scope, F)) -> Signal<T>
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let (sx, func) = tuple;
        crate::create_func_signal_eq(sx, func)
    }
}
pub(crate) struct TrueFunc;

impl TrueFunc {
    #[inline]
    pub(crate) fn new<F, T>(self, tuple: (Scope, F)) -> Signal<T>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        crate::create_func_signal(sx, func)
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
impl<T> EqDataKind for (Scope, T) where T: PartialEq + 'static {}

pub(crate) trait TrueDataKind {
    #[inline]
    fn data_kind(&self) -> TrueData {
        TrueData
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<T> TrueDataKind for &(Scope, T) where T: 'static {}

pub(crate) struct EqData;

impl EqData {
    #[inline]
    pub(crate) fn new<T>(self, tuple: (Scope, T)) -> Signal<T>
    where
        T: PartialEq + 'static,
    {
        let (sx, value) = tuple;
        crate::create_data_signal(sx, value)
    }
}
pub(crate) struct TrueData;

impl TrueData {
    #[inline]
    pub(crate) fn new<T>(self, tuple: (Scope, T)) -> Signal<T>
    where
        T: 'static,
    {
        let (sx, value) = tuple;
        crate::create_data_signal(sx, value)
    }
}
