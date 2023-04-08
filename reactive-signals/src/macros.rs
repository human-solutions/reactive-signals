/// The `signal!` macro is used to create signals of all types. It automatically detects
/// the type of the provided data or function and if it implements [PartialEq] or [Hash](std::hash::Hash)
///
/// Arguments:
/// - `scope`: mandatory. The [Scope](crate::Scope) to use when creating the [Signal](crate::Signal)
/// - `clone:`: optional. A space-separated list of data to clone and provide to the function.
/// - `server` | `client`: optional. Whether the signal should run only on the server or the client.
/// - `inner`: the data or function the signal handles.
///
/// Examples:
///
/// - [reactive data signals](Self#Example_of_reactive_data_signals)
/// - [functional reactive signals](Self#Example_of_functional_reactive_signals)
/// - [async functional reactive signals](Self#Example_of_async_functional_reactive_signals)
///
/// # Example of reactive data signals
///
/// ```rust
/// # use reactive_signals::types::*;
/// use reactive_signals::{Scope, Signal, signal};
///
/// let (_guard, sc) = Scope::new_client_side_root_scope();
///
/// // Create a string signal. Since string implements PartialEq the
/// // signal will only notify it's subscribers if it's value change.
/// let string_sig = signal!(sc, "hi".to_string());
///
/// struct MyNoEqData;
/// // Create a signal from data that doesn't implement equality.
/// // it will always notify the subscribers when it changes.
/// let no_eq_sig = signal!(sc, MyNoEqData);
/// ```
///
/// # Example of functional reactive signals
///
/// ```rust
/// # use reactive_signals::types::*;
/// # use std::cell::RefCell;
/// # use std::rc::Rc;
/// #
/// use reactive_signals::{Scope, Signal, signal};
///
/// let (_guard, sc) = Scope::new_client_side_root_scope();
///
/// struct MyNoEqData;
///
/// let count_sig = signal!(sc, 4);
///
/// // create a simple functional signal
/// let func_sig = signal!(sc, move || count_sig.get() + 1);
///
/// ///////////// the clone argument /////////////
///
/// let counter = Rc::new(RefCell::new(0));
///
/// // using the clone argument you can provide a space-separated
/// // list of data to clone and provide to the function.
/// let counter_upd = signal!(sc, clone: counter, move || *counter.borrow_mut() += 1);
///
/// // the above is equivalent to:
/// let counter_upd = {
///     let counter = counter.clone();
///     signal!(sc, move ||  *counter.borrow_mut() += 1)
/// };
///
/// ///////////// client and server only signals /////////////
///
/// // create a signal that only runs on the server
/// let server_func  = signal!(sc, server, move || count_sig.get() + 1);
///
/// // create a signal that only runs on the client
/// let client_func = signal!(sc, client, move || count_sig.get() + 1);
/// ```
///
/// # Example of async functional reactive signals
///
/// Note that this has not yet been implemented and the exact details of the API
/// has not been ironed out.
///
/// ```rust ignore
/// // creating an async closure/function is basically the same as normal one
/// let remote_count = signal!(sc, move async || {
///     // some async stuff
///     fetch_usize_count().await
/// });
///
/// // an async signal works just like any other signal, it just waits until
/// // the async closure finishes before notifiying subscribers.
/// signal!(sc, move || println!("Remote count is now: {}", remote_count.get()));
///
/// let async_timer = signal!(sc, 500 ms, move async |&mut interval| {
///     // runs after 500 ms
///     // you can change the interval or stop it by:
///     *interval = None;
/// });
/// ```
#[macro_export]
macro_rules! signal {
    ($scope:ident, $inner:expr) => {{
        #[allow(unused_imports)]
        use $crate::{EqFuncKind, TrueFuncKind, EqDataKind, TrueDataKind, HashEqDataKind};
        match ($scope, $inner) {
            tuple => (&&tuple).signal_kind().new(tuple),
        }
    }};
    ($scope:ident, server, $inner:expr) => {{
        #[allow(unused_imports)]
        use $crate::{ServerEqFuncKind, ServerTrueFuncKind};
        match ($scope, $inner) {
            tuple => (&&tuple).server_kind().new(tuple),
        }
    }};
    ($scope:ident, server, clone: $($clone:ident) +, $inner:expr) => {{
        $(let $clone = $clone.clone();)*
        #[allow(unused_imports)]
        use $crate::{ServerEqFuncKind, ServerTrueFuncKind};
        match ($scope, $inner) {
            tuple => (&&tuple).server_kind().new(tuple),
        }
    }};

    ($scope:ident, client, $inner:expr) => {{
        #[allow(unused_imports)]
        use $crate::{ClientEqFuncKind, ClientTrueFuncKind};
        match ($scope, $inner) {
            tuple => (&&tuple).client_kind().new(tuple),
        }
    }};
    ($scope:ident, client, clone: $($clone:ident) +, $inner:expr) => {{
        $(let $clone = $clone.clone();)*
        #[allow(unused_imports)]
        use $crate::{ClientEqFuncKind, ClientTrueFuncKind};
        match ($scope, $inner) {
            tuple => (&&tuple).client_kind().new(tuple),
        }
    }};
    ($scope:ident, clone: $($clone:ident) +, $data:expr) => {{
        $(let $clone = $clone.clone();)*
        #[allow(unused_imports)]
        use $crate::{EqFuncKind, TrueFuncKind};
        match ($scope, $data) {
            tuple => (&&tuple).signal_kind().new(tuple),
        }
    }};
}

#[test]
fn test() {
    use crate::Scope;
    let (_guard, sx) = Scope::new_server_side_root_scope();
    let _sig = signal!(sx, 32);
    // assert!(!sig.eq);

    let x = 5;
    let _sig = signal!(sx, x);
    // assert!(!sig.eq);

    let _sig = signal!(sx, || 32);
    // assert!(sig.eq);

    #[derive(Clone)]
    struct NonEq;
    let _sig = signal!(sx, || NonEq);
    // assert!(!sig.eq);

    let ne = NonEq;
    let _sig = signal!(sx, move || ne.clone());
    // assert!(!sig.eq);

    let ne = NonEq;
    let _sig = signal!(sx, clone: ne, move || ne.clone());

    // assert!(!sig.eq);

    let _sit = signal!(sx, server, move || ne.clone());

    let srv = signal!(sx, server, move || 1);
    assert_eq!(srv.opt_get(), Some(1));

    let clnt = signal!(sx, client, move || 1);

    assert_eq!(clnt.opt_get(), None);
}
