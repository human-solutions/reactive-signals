use crate::{runtimes::Runtime, updater::propagate_change, Signal};

impl<T: Clone + 'static, RT: Runtime> Signal<T, RT> {
    pub fn get(&self) -> T {
        self.id.rt_ref(|rt| {
            if let Some(listener) = rt.get_running_signal() {
                rt[self.id].with_signal(self.id, |signal| {
                    signal.listeners.insert(listener);
                    signal.get()
                })
            } else {
                rt[self.id].with_signal(self.id, |signal| signal.get())
            }
        })
    }

    pub fn set(&self, val: T) {
        self.id.rt_ref(|rt| {
            rt[self.id].with_signal_mut(self.id, |sig| sig.set(val));

            propagate_change(rt, self.id);
        });
    }
}
