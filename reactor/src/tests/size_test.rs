///
/// You need the wasm-bindgen-cli installed:
///  - cargo install wasm-bindgen-cli --vers "0.2.84"
///
use std::{mem, num::NonZeroU16};
use wasm_bindgen_test::*;

use crate::{
    primitives::{AnyData, DynFunc, SignalSet},
    runtimes::ClientRuntime,
    signal::SignalId,
    signal::{SignalInner, SignalValue},
};

/// Run with both:
///  - cargo test --target wasm32-unknown-unknown --profile=release
///  - cargo test --target wasm32-unknown-unknown
#[cfg(not(feature = "unsafe-cell"))]
#[wasm_bindgen_test]
fn wasm_sizes() {
    // Box & RefCell
    assert_eq!(mem::size_of::<AnyData>(), 8);
    // Box & dyn Fn = 2 words + AnyData
    assert_eq!(mem::size_of::<DynFunc>(), 16);

    // SignalValue: max of DynFunc & AnyData
    // In --release there's an item (word) less
    let size = if cfg!(debug_assertions) { 20 } else { 16 };
    assert_eq!(mem::size_of::<SignalValue>(), size);

    // SignalSet: RefCell & Vec
    assert_eq!(mem::size_of::<SignalSet<3, SignalId<ClientRuntime>>>(), 20);

    // SignalInner: SignalValue + SignalSet
    let size = if cfg!(debug_assertions) { 40 } else { 36 };
    assert_eq!(mem::size_of::<SignalInner<ClientRuntime>>(), size);
}

/// Run with both:
///  - cargo test --target wasm32-unknown-unknown --features=unsafe-cell --profile=release
///  - cargo test --target wasm32-unknown-unknown --features=unsafe-cell
#[cfg(feature = "unsafe-cell")]
#[wasm_bindgen_test]
fn wasm_sizes() {
    // Box & RefCell
    assert_eq!(mem::size_of::<AnyData>(), 8);
    // Box & dyn Fn = 2 words + AnyData
    assert_eq!(mem::size_of::<DynFunc>(), 16);

    // SignalValue: max of DynFunc & AnyData
    // In --release there's an item (word) less
    let size = if cfg!(debug_assertions) { 20 } else { 16 };
    assert_eq!(mem::size_of::<SignalValue>(), size);

    // SignalSet: UnsafeCell & Vec
    assert_eq!(mem::size_of::<SignalSet<3, SignalId<ClientRuntime>>>(), 16);

    // SignalInner: SignalValue + SignalSet
    let size = if cfg!(debug_assertions) { 36 } else { 32 };
    assert_eq!(mem::size_of::<SignalInner<ClientRuntime>>(), size);
}

/// Test sizes of ArrVec enum
#[allow(dead_code, non_camel_case_types)]
#[wasm_bindgen_test]
fn test_subscriber_size() {
    // vec: a pointer, size and capacity
    assert_eq!(mem::size_of::<Vec<SignalId<ClientRuntime>>>(), 12);
    assert_eq!(mem::size_of::<[(u16, u16); 1]>(), 4);

    enum Store_u15_NoRT_big {
        // uses more space if the NonZeroU16 is replaced with an ordinary u16
        Arr([(u16, NonZeroU16); 8]),
        Vec(Vec<(u16, u16)>),
    }
    assert_eq!(mem::size_of::<Store_u15_NoRT_big>(), 32);

    enum Store_u15_NoRT_small {
        Arr([(u16, u16); 2]),
        Vec(Vec<(u16, u16)>),
    }
    assert_eq!(mem::size_of::<Store_u15_NoRT_small>(), 16);

    // u15 NoRT
    // When arr has 2 elements, then the enum size 24, the same as for a vec.
    // When the arr has 3-8 element the size jumps to 32 bytes

    enum Store_u15_RT_big {
        Arr([(u16, u16, u16); 5]),
        Vec(Vec<(u16, u16, u16)>),
    }
    assert_eq!(mem::size_of::<Store_u15_RT_big>(), 32);

    enum Store_u15_RT_small {
        Arr([(u16, u16, u16); 1]),
        Vec(Vec<(u16, u16, u16)>),
    }
    assert_eq!(mem::size_of::<Store_u15_RT_small>(), 16);
}
