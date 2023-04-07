use std::fmt::Debug;

use super::SignalId;
use crate::{
    primitives::{any_data2::AnyData, dyn_func2::DynFunc, SignalSet},
    runtimes2::RuntimeInner,
};

#[derive(Debug)]
pub enum SignalValue {
    Data(AnyData),
    Func(DynFunc),
    #[cfg(debug_assertions)]
    Reuse,
}

#[derive(Debug)]
pub(crate) struct SignalInner<'a> {
    pub(super) value: SignalValue,
    pub(crate) listeners: SignalSet<3, SignalId<'a>>,
}

impl<'a> SignalInner<'a> {
    pub(crate) fn value(&self) -> &AnyData {
        match self.value {
            SignalValue::Data(ref value) | SignalValue::Func(DynFunc { ref value, .. }) => value,
            #[cfg(debug_assertions)]
            SignalValue::Reuse => panic!("BUG: using a reused signal"),
        }
    }

    pub(crate) fn run(&self, rt: &RuntimeInner<'a>, id: SignalId<'a>) -> bool {
        if let SignalValue::Func(func) = &self.value {
            let previous = rt.set_running_signal(Some(id));
            let changed = func.run();
            // println!("run: {id:?} - {changed}");
            rt.set_running_signal(previous);
            changed
        } else {
            // println!("NOT: {id:?}");
            false
        }
    }

    pub(crate) fn reuse(&mut self) {
        self.listeners.clear();
        #[cfg(debug_assertions)]
        {
            self.value = SignalValue::Reuse;
        }
    }
}
