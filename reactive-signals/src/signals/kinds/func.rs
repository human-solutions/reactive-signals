// `fn new(self, ..) -> Signal` is the `signal!` macro's autoref-specialization dispatch
// (`(&&tuple).signal_kind().new(tuple)`), not a conventional constructor — the `self`
// receiver and non-`Self` return are deliberate.
#![allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]

use crate::{
    primitives::DynFunc,
    runtimes::Runtime,
    signals::{EqFunc, Func},
    Scope, Signal,
};

pub trait EqFuncKind {
    #[inline]
    fn signal_kind(&self) -> EqFuncSignal {
        EqFuncSignal
    }
}

// Highest priority: with a `&&tuple` receiver, `&self` on `&(Scope, F)` gives a
// receiver type of `&&(Scope, F)` — an exact match needing no adjustment.
impl<F, T, RT: Runtime> EqFuncKind for &(Scope<RT>, F)
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

// Lower priority than EqFuncKind: needs one autoref of the `&&tuple` receiver.
impl<F, T, RT: Runtime> TrueFuncKind for &&(Scope<RT>, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct EqFuncSignal;

impl EqFuncSignal {
    #[inline]
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<EqFunc<T>, RT>
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
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<Func<T>, RT>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, Func<T>>(func))
    }
}
