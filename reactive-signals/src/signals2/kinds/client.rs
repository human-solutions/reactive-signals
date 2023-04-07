use crate::primitives::dyn_func2::DynFunc;

use crate::scope2::Scope;
use crate::signals2::{ClientEqFunc, ClientFunc, Signal};

pub trait ClientEqFuncKind {
    #[inline]
    fn client_kind(&self) -> ClientEqFuncSignal {
        ClientEqFuncSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<'a, F, T> ClientEqFuncKind for (Scope<'a>, F)
where
    F: Fn() -> T + 'static,
    T: PartialEq + 'static,
{
}

pub trait ClientTrueFuncKind {
    #[inline]
    fn client_kind(&self) -> ClientTrueFuncSignal {
        ClientTrueFuncSignal
    }
}

// Requires one extra autoref to call! Lower priority than EqKind.
impl<'rt, F, T> ClientTrueFuncKind for &(Scope<'rt>, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct ClientEqFuncSignal;

impl ClientEqFuncSignal {
    #[inline]
    pub fn new<'rt, F, T>(self, tuple: (Scope<'rt>, F)) -> Signal<ClientEqFunc<T>>
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, ClientEqFunc<T>>(func))
    }
}
pub struct ClientTrueFuncSignal;

impl ClientTrueFuncSignal {
    #[inline]
    pub fn new<'rt, F, T>(self, tuple: (Scope<'rt>, F)) -> Signal<ClientFunc<T>>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, ClientFunc<T>>(func))
    }
}
