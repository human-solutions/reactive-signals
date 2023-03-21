#[macro_export]
macro_rules! signal {
    ($sx:ident, $data:literal) => {{
        #[allow(unused_imports)]
        use $crate::signal::{EqDataKind, TrueDataKind};
        match ($sx, $data) {
            tuple => (&tuple).data_kind().new(tuple),
        }
    }};
    ($sx:ident, $data:ident) => {{
        #[allow(unused_imports)]
        use $crate::signal::{EqDataKind, TrueDataKind};
        match ($sx, $data) {
            tuple => (&tuple).data_kind().new(tuple),
        }
    }};
    ($sx:ident, $data:expr) => {{
        #[allow(unused_imports)]
        use $crate::signal::{EqFuncKind, TrueFuncKind};
        match ($sx, $data) {
            tuple => (&tuple).func_kind().new(tuple),
        }
    }};
    ($sx:ident, clone: $($clone:ident) +, $data:expr) => {{
        $(let $clone = $clone.clone();)*
        #[allow(unused_imports)]
        use $crate::signal::{EqFuncKind, TrueFuncKind};
        match ($sx, $data) {
            tuple => (&tuple).func_kind().new(tuple),
        }
    }};
}

#[test]
fn test() {
    use crate::runtimes::RuntimePool;
    let sx = RuntimePool::new_root_scope();
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
}
