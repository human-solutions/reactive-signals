use crate::primitives::DynFunc;

use crate::scope::Scope;
use crate::signals::{ClientEqFunc, ClientFunc, Signal};

pub trait ClientEqFuncKind {
    #[inline]
    fn client_kind(&self) -> ClientEqFuncSignal {
        ClientEqFuncSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<F, T> ClientEqFuncKind for (Scope, F)
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
impl<'rt, F, T> ClientTrueFuncKind for &(Scope, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct ClientEqFuncSignal;

impl ClientEqFuncSignal {
    #[inline]
    pub fn new<'rt, F, T>(self, tuple: (Scope, F)) -> Signal<ClientEqFunc<T>>
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
    pub fn new<'rt, F, T>(self, tuple: (Scope, F)) -> Signal<ClientFunc<T>>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, ClientFunc<T>>(func))
    }
}
