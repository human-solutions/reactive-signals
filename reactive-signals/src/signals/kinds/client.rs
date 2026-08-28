// `fn new(self, ..) -> Signal` is the `signal!` macro's autoref-specialization dispatch
// (`(&&tuple).client_kind().new(tuple)`), not a conventional constructor — the `self`
// receiver and non-`Self` return are deliberate.
#![allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]

use crate::primitives::DynFunc;
use crate::{runtimes::Runtime, Scope};

use crate::signals::{ClientEqFunc, ClientFunc, Signal};

pub trait ClientEqFuncKind {
    #[inline]
    fn client_kind(&self) -> ClientEqFuncSignal {
        ClientEqFuncSignal
    }
}

// Highest priority: with a `&&tuple` receiver, `&self` on `&(Scope, F)` gives a
// receiver type of `&&(Scope, F)` — an exact match needing no adjustment.
impl<F, T, RT: Runtime> ClientEqFuncKind for &(Scope<RT>, F)
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

// Lower priority than ClientEqFuncKind: needs one autoref of the `&&tuple` receiver.
impl<F, T, RT: Runtime> ClientTrueFuncKind for &&(Scope<RT>, F)
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
