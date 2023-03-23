use crate::{primitives::Compare, runtimes::Runtime, updater::propagate_change, Signal};

use super::{SignalId, SignalInner};

impl<T, RT> Signal<T, RT>
where
    T: 'static + Compare,
    RT: Runtime,
{
    /// Set the signal's value and notifies subscribers
    /// if the value changed when it implements `PartialEq`
    /// otherwise it always notifies.
    pub fn set(&self, val: T::Inner) {
        self.id.rt_ref(|rt| {
            let is_equal = rt[self.id].with_signal(self.id, |sig| sig.value().set::<T>(val));
            if !is_equal {
                propagate_change(rt, self.id);
            }
        });
    }

    /// Applies a function to the current value to mutate it in place and returns
    /// whatever that function returns.
    ///
    /// Subscribers are notified if the value changed when it implements [PartialEq]
    /// otherwise it always notifies.
    ///
    /// **Example of using the return value**
    ///
    /// ```rust
    /// # use reactor::{signal, runtimes::SingleRuntime};
    /// # let cx = SingleRuntime::new_root_scope();
    /// let count = signal!(cx, 2);
    /// let is_even = count.update(|val| {
    ///     *val += 1;
    ///     *val % 2 == 0
    /// });
    /// ```
    ///
    pub fn update<R: 'static>(&self, f: impl Fn(&mut T::Inner) -> R) -> R {
        self.id.rt_ref(|rt| {
            let r = rt[self.id].with_signal(self.id, |sig| sig.value().update::<T, R>(f));
            propagate_change(rt, self.id);
            r
        })
    }
}

impl<T, RT> Signal<T, RT>
where
    T: 'static + Compare,
    T::Inner: Copy,
    RT: Runtime,
{
    /// Get a copy of the signal value (if the value implements [Copy])
    pub fn get(&self) -> T::Inner {
        register_and_run(self.id, |sig| sig.value().get::<T>())
    }
}

impl<T: 'static + Compare, RT: Runtime> Signal<T, RT>
where
    T: 'static + Compare,
    T::Inner: Clone,
    RT: Runtime,
{
    /// Get a clone of the signal value (if the value implements [Clone])
    ///
    /// Use the `.with()` function if you can in order to avoid the clone.
    pub fn cloned(&self) -> T::Inner {
        register_and_run(self.id, |sig| sig.value().cloned::<T>())
    }
}

impl<T: 'static + Compare, RT: Runtime> Signal<T, RT> {
    /// Applies a function to the current value to mutate it in place and returns
    /// whatever that function returns.
    ///
    /// Subscribers are notified if the value changed when it implements `PartialEq`
    /// otherwise it always notifies.
    ///
    /// **Example of using the return value**
    ///
    /// ```rust
    /// # use reactor::{signal, runtimes::SingleRuntime};
    /// # let cx = SingleRuntime::new_root_scope();
    /// let count = signal!(cx, 2);
    /// let is_even = count.with(|val| *val % 2 == 0);
    /// ```
    ///
    pub fn with<R: 'static>(&self, f: impl Fn(&T::Inner) -> R) -> R {
        register_and_run(self.id, |sig| sig.value().with::<T, R>(f))
    }
}

#[inline]
fn register_and_run<RT: Runtime, T: 'static, F: FnOnce(&SignalInner<RT>) -> T>(
    id: SignalId<RT>,
    f: F,
) -> T {
    id.rt_ref(|rt| {
        if let Some(listener) = rt.get_running_signal() {
            rt[id].with_signal(id, |signal| {
                signal.listeners.insert(listener);
                f(signal)
            })
        } else {
            rt[id].with_signal(id, |signal| f(signal))
        }
    })
}
