mod signal_accessors;
mod signal_id;
mod signal_inner;
mod signal_kind;
mod signal_new;

use std::marker::PhantomData;

use crate::runtimes::Runtime;
pub(crate) use signal_id::SignalId;
pub(crate) use signal_inner::{SignalInner, SignalValue};
#[allow(unused_imports)] // allowed because these are used by the signal! macro.
pub(crate) use signal_kind::{EqDataKind, EqFuncKind, TrueDataKind, TrueFuncKind};

pub struct Signal<T, RT: Runtime> {
    id: SignalId<RT>,
    ty: PhantomData<T>,
}

impl<T, RT: Runtime> Clone for Signal<T, RT> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            ty: self.ty,
        }
    }
}
impl<T, RT: Runtime> Copy for Signal<T, RT> {}
