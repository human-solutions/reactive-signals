use crate::primitives::dyn_func2::DynFunc;

use crate::scope2::Scope;
use crate::signals2::{ServerEqFunc, ServerFunc, Signal};

pub trait ServerEqFuncKind {
    #[inline]
    fn server_kind(&self) -> ServerEqFuncSignal {
        ServerEqFuncSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<'rt, F, T> ServerEqFuncKind for (Scope<'rt>, F)
where
    F: Fn() -> T + 'static,
    T: PartialEq + 'static,
{
}

pub trait ServerTrueFuncKind {
    #[inline]
    fn server_kind(&self) -> ServerTrueFunc {
        ServerTrueFunc
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<'rt, F, T> ServerTrueFuncKind for &(Scope<'rt>, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct ServerEqFuncSignal;

impl ServerEqFuncSignal {
    #[inline]
    pub fn new<'rt, F, T>(self, tuple: (Scope<'rt>, F)) -> Signal<ServerEqFunc<T>>
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, ServerEqFunc<T>>(func))
    }
}
pub struct ServerTrueFunc;

impl ServerTrueFunc {
    #[inline]
    pub fn new<'rt, F, T>(self, tuple: (Scope<'rt>, F)) -> Signal<ServerFunc<T>>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, ServerFunc<T>>(func))
    }
}
