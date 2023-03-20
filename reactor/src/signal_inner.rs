use std::fmt::Debug;

use crate::{
    primitives::{AnyData, DynFunc, SignalSet},
    runtimes::{Runtime, RuntimeInner},
    signal_id::SignalId,
};

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub enum SignalValue {
    Data(AnyData),
    Func(DynFunc),
    #[cfg(debug_assertions)]
    Reuse,
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub(crate) struct SignalInner<RT: Runtime> {
    pub(crate) value: SignalValue,
    pub(crate) listeners: SignalSet<3, SignalId<RT>>,
}

impl<RT: Runtime> SignalInner<RT> {
    fn value(&self) -> &AnyData {
        match self.value {
            SignalValue::Data(ref value) | SignalValue::Func(DynFunc { ref value, .. }) => value,
            #[cfg(debug_assertions)]
            SignalValue::Reuse => panic!("BUG: using a reused signal"),
        }
    }

    pub fn get<T: 'static + Clone>(&self) -> T {
        self.value().cloned()
    }

    pub(crate) fn set<T: 'static>(&mut self, new_value: T) {
        match self.value {
            SignalValue::Data(ref mut value) | SignalValue::Func(DynFunc { ref mut value, .. }) => {
                *value = AnyData::new(new_value)
            }
            #[cfg(debug_assertions)]
            SignalValue::Reuse => panic!("BUG: using a reused signal"),
        }
    }

    pub(crate) fn run(&self, rt: &RuntimeInner<RT>, id: SignalId<RT>) -> bool {
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
