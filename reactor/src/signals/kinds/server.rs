use crate::primitives::DynFunc;
use crate::{runtimes::Runtime, Scope};

use crate::signals::{ServerEqFunc, ServerFunc, Signal};

pub trait ServerEqFuncKind {
    #[inline]
    fn server_kind(&self) -> ServerEqFuncSignal {
        ServerEqFuncSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<F, T, RT: Runtime> ServerEqFuncKind for (Scope<RT>, F)
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
impl<F, T, RT: Runtime> ServerTrueFuncKind for &(Scope<RT>, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct ServerEqFuncSignal;

impl ServerEqFuncSignal {
    #[inline]
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<ServerEqFunc<T>, RT>
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
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<ServerFunc<T>, RT>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, ServerFunc<T>>(func))
    }
}
