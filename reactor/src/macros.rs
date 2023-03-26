#[macro_export]
macro_rules! signal {
    ($sx:ident, $data:expr) => {{
        #[allow(unused_imports)]
        use $crate::{EqFuncKind, TrueFuncKind, EqDataKind, TrueDataKind, HashEqDataKind};
        match ($sx, $data) {
            tuple => (&&tuple).signal_kind().new(tuple),
        }
    }};
    ($sx:ident, server, $data:expr) => {{
        #[allow(unused_imports)]
        use $crate::{ServerEqFuncKind, ServerTrueFuncKind};
        match ($sx, $data) {
            tuple => (&&tuple).server_kind().new(tuple),
        }
    }};
    ($sx:ident, server, clone: $($clone:ident) +, $data:expr) => {{
        $(let $clone = $clone.clone();)*
        #[allow(unused_imports)]
        use $crate::{ServerEqFuncKind, ServerTrueFuncKind};
        match ($sx, $data) {
            tuple => (&&tuple).server_kind().new(tuple),
        }
    }};

    ($sx:ident, client, $data:expr) => {{
        #[allow(unused_imports)]
        use $crate::{ClientEqFuncKind, ClientTrueFuncKind};
        match ($sx, $data) {
            tuple => (&&tuple).client_kind().new(tuple),
        }
    }};
    ($sx:ident, client, clone: $($clone:ident) +, $data:expr) => {{
        $(let $clone = $clone.clone();)*
        #[allow(unused_imports)]
        use $crate::{ClientEqFuncKind, ClientTrueFuncKind};
        match ($sx, $data) {
            tuple => (&&tuple).client_kind().new(tuple),
        }
    }};
    ($sx:ident, clone: $($clone:ident) +, $data:expr) => {{
        $(let $clone = $clone.clone();)*
        #[allow(unused_imports)]
        use $crate::{EqFuncKind, TrueFuncKind};
        match ($sx, $data) {
            tuple => (&&tuple).signal_kind().new(tuple),
        }
    }};
}

#[test]
fn test() {
    use crate::runtimes::ServerRuntime;
    let sx = ServerRuntime::new_root_scope();
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
