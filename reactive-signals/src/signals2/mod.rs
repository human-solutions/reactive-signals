// #[cfg(test)]
// mod tests;

pub(crate) mod kinds;
mod signal_accessors;
mod signal_id;
mod signal_inner;
mod signal_new;
pub mod types;
mod updater;

use std::marker::PhantomData;

pub(crate) use signal_id::SignalId;
pub(crate) use signal_inner::SignalInner;
pub(crate) use types::*;

#[doc(hidden)]
pub use kinds::*;

/// A [Signal] is a reactive value or a function that produces a value,
/// with subscribers that are automatically notified when the value changes.
///
/// When it is a function, the function automatically subscribes to all the other
/// signals it is using and automatically re-runs when any of those signals change.
///
/// If the value implements [PartialEq] then the subscribers are notified only if
/// the value changed.
///
/// A [Signal] is created in a reactive [Scope](crate::Scope) using the [signal!](crate::signal!) macro.
/// It can only be deleted by discarding that [Scope](crate::Scope).
///
/// ## Accessors
///
/// Only data signals can be manually changed. Func signals that only runs on `server` or `client`
/// always return optional values which are some only when runninig on their side.
///
/// | Value implements | Data signal          | Func signal | Func signal with<br>`server` or `client` |
/// | ---              | --                   | ---         | ---                                      |
/// | -                | .set, .update, .with | .with       | .opt_with                                |
/// | [Clone]          | .cloned              | .cloned     | .opt_cloned                              |
/// | [Copy]           | .get                 | .get        | .opt_get                                 |
///
///
/// ## Example
///
/// ```rust
/// use reactive_signals::{runtimes::ClientRuntime, signal};
///
/// // signals are created in scopes
/// let sx = ClientRuntime::new_root_scope();
///
/// // a simple data value
/// let count = signal!(sx, 5);
///
/// // a simple string value
/// let name = signal!(sx, "kiwi");
///
/// // is_plural will update when count changes
/// let is_plural = signal!(sx, move || count.get() != 1);
///
/// // we'll keep a history of all changes
/// let history = signal!(sx, Vec::<String>::new());
///
/// let text = signal!(sx, move || {
///     let ending = if is_plural.get() { "s" } else { "" };
///     let txt = format!("{} {}{ending}", count.get(), name.get());
///     // using .update we can add the text to the vec without cloning the vec
///     history.update(|hist| hist.push(txt.clone()));
///     txt
/// });
///
/// assert_eq!(text.cloned(), "5 kiwis");
///
/// // when setting to same value the subscribers are not notified.
/// name.set("kiwi");
/// assert_eq!(history.with(|h| h.join(", ")), "5 kiwis");
///
/// // when changing the count the name and is_plural are updated automatically.
/// count.set(1);
/// assert_eq!(text.cloned(), "1 kiwi");
///
/// // you can update the name
/// name.update(|t| *t = "fig");
/// assert_eq!(text.cloned(), "1 fig");
///
/// // 1 kiwi is repated because when changing count, is_plural changes as well
/// // triggering a second update of the text. This will be detected in
/// // future versions and only notified once.
/// assert_eq!(
///     history.with(|h| h.join(", ")),
///     "5 kiwis, 1 kiwi, 1 kiwi, 1 fig"
/// );
///
/// with_signal_arg(count);
///
/// // when declaring functions some additional imports are necessary
/// use reactive_signals::{runtimes::Runtime, Signal, types::*};
///
/// fn with_signal_arg<RT: Runtime>(count: Signal<EqData<i32>, RT>) {
/// }
///
/// ```
pub struct Signal<'rt, T: SignalType> {
    id: SignalId<'rt>,
    ty: PhantomData<T>,
}

impl<'rt, T: SignalType> Clone for Signal<'rt, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            ty: self.ty,
        }
    }
}

impl<'rt, T: SignalType> Copy for Signal<'rt, T> {}

// #[test]
// fn test_example() {
//     use crate::{runtimes2::ClientRuntime, signal};

//     // signals are created in scopes
//     let sx = ClientRuntime::new_root_scope();

//     // a simple data value
//     let count = signal!(sx, 5);

//     // a simple string value
//     let name = signal!(sx, "kiwi");

//     // is_plural will update when count changes
//     let is_plural = signal!(sx, move || count.get() != 1);

//     // we'll keep a history of all changes
//     let history = signal!(sx, Vec::<String>::new());

//     let text = signal!(sx, move || {
//         let ending = if is_plural.get() { "s" } else { "" };
//         let txt = format!("{} {}{ending}", count.get(), name.get());
//         // using .update we can add the text to the vec without cloning the vec
//         history.update(|hist| hist.push(txt.clone()));
//         txt
//     });

//     assert_eq!(text.cloned(), "5 kiwis");

//     // when setting to same value the subscribers are not notified.
//     name.set("kiwi");
//     assert_eq!(history.with(|h| h.join(", ")), "5 kiwis");

//     // when changing the count the name and is_plural are updated automatically.
//     count.set(1);
//     assert_eq!(text.cloned(), "1 kiwi");

//     // you can update the name
//     name.update(|t| *t = "fig");
//     assert_eq!(text.cloned(), "1 fig");

//     // 1 kiwi is repated because when changing count, is_plural changes as well
//     // triggering a second update of the text. This will be detected in
//     // future versions and only notified once.
//     assert_eq!(
//         history.with(|h| h.join(", ")),
//         "5 kiwis, 1 kiwi, 1 kiwi, 1 fig"
//     );
// }
