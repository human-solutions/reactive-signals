use crate::{
    primitives::DynFunc,
    scope::Scope,
    signals::{EqFunc, Func, Signal},
};

pub trait EqFuncKind {
    #[inline]
    fn signal_kind(&self) -> EqFuncSignal {
        EqFuncSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<'rt, F, T> EqFuncKind for (Scope<'rt>, F)
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
impl<'rt, F, T> TrueFuncKind for &(Scope<'rt>, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct EqFuncSignal;

impl EqFuncSignal {
    #[inline]
    pub fn new<'rt, F, T>(self, tuple: (Scope<'rt>, F)) -> Signal<EqFunc<T>>
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, EqFunc<T>>(func))
    }
}
pub struct TrueFunc;

impl TrueFunc {
    #[inline]
    pub fn new<'rt, F, T>(self, tuple: (Scope<'rt>, F)) -> Signal<Func<T>>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, Func<T>>(func))
    }
}
