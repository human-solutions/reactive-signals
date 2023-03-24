use crate::primitives::DynFunc;
use crate::{runtimes::Runtime, Scope};

use crate::signals::{ClientEqFunc, ClientFunc, Signal};

pub trait ClientEqFuncKind {
    #[inline]
    fn client_kind(&self) -> ClientEqFuncSignal {
        ClientEqFuncSignal
    }
}

// Does not require any autoref if called as (&error).datakind().
impl<F, T, RT: Runtime> ClientEqFuncKind for (Scope<RT>, F)
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
impl<F, T, RT: Runtime> ClientTrueFuncKind for &(Scope<RT>, F)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
}

pub struct ClientEqFuncSignal;

impl ClientEqFuncSignal {
    #[inline]
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<ClientEqFunc<T>, RT>
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
    pub fn new<F, T, RT: Runtime>(self, tuple: (Scope<RT>, F)) -> Signal<ClientFunc<T>, RT>
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let (sx, func) = tuple;
        Signal::func(sx, || DynFunc::new::<F, T, ClientFunc<T>>(func))
    }
}
