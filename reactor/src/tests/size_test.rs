use std::{cell::RefCell, mem, num::NonZeroU16};

use crate::{
    primitives::{AnyData, DynFunc, SignalSet},
    runtimes::SingleRuntimeId,
    signal_id::SignalId,
    signal_inner::{SignalInner, SignalValue},
    Signal,
};

#[test]
fn test_signal_sizes() {
    assert_eq!(4, mem::size_of::<Signal<String, SingleRuntimeId>>());
    assert_eq!(4, mem::size_of::<Signal<usize, SingleRuntimeId>>());
}

#[test]
fn size_of_ref_cell_box() {
    assert_eq!(mem::size_of::<RefCell<Box<usize>>>(), 16);
    assert_eq!(mem::size_of::<Box<usize>>(), 8);
}

#[test]
fn size_of_signal_inner() {
    // Box & RefCell = 2 words
    assert_eq!(mem::size_of::<AnyData>(), 16);
    // Box & dyn Fn = 2 words + AnyData
    assert_eq!(mem::size_of::<DynFunc>(), 32);
    // SignalValue: max of DynFunc & AnyData
    // Is 32 in --release but in debug there's an extra item
    // in the enum which makes it 40
    assert_eq!(mem::size_of::<SignalValue>(), 40);

    #[cfg(feature = "use-unsafe")]
    // SignalSet: RefCell & Vec = 3 words
    assert_eq!(
        mem::size_of::<SignalSet<3, SignalId<SingleRuntimeId>>>(),
        32
    );
    #[cfg(not(feature = "use-unsafe"))]
    // SignalSet: RefCell & Vec = 4 words
    assert_eq!(
        mem::size_of::<SignalSet<3, SignalId<SingleRuntimeId>>>(),
        40
    );

    #[cfg(feature = "use-unsafe")]
    // SignalInner: SignalValue + SignalSet = 7 words (8 words when not in --release)
    assert_eq!(mem::size_of::<SignalInner<SingleRuntimeId>>(), 72);
    #[cfg(not(feature = "use-unsafe"))]
    // SignalInner: SignalValue + SignalSet = 8 words (9 words when not in --release)
    assert_eq!(mem::size_of::<SignalInner<SingleRuntimeId>>(), 80);

    // unsafe:
    // 64-bit arch: 7 words = 56 bytes
    // 32-bit arch: 7 words = 28 bytes

    // safe:
    // 64-bit arch: 8 words = 64 bytes
    // 32-bit arch: 8 words = 32 bytes
}

#[allow(dead_code, non_camel_case_types)]
#[test]
fn test_subscriber_size() {
    // vec: a pointer, size and capacity
    assert_eq!(mem::size_of::<Vec<SignalId<SingleRuntimeId>>>(), 24);
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
    assert_eq!(mem::size_of::<Store_u15_NoRT_small>(), 24);

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
    assert_eq!(mem::size_of::<Store_u15_RT_small>(), 24);
}

// Copy-paste into wasm and run

// use std::mem::size_of;
// use std::num::NonZeroU16;

// // vec: a pointer, size and capacity: 12 bytes
// log!("Vec<SignalId>: {}", size_of::<Vec<SignalId>>());
// // 4 bytes
// log!("[(u16, u16); 1]: {}", size_of::<[(u16, u16); 1]>());

// enum Store_u15_NoRT_big {
//     // uses more space if the NonZeroU16 is replaced with an ordinary u16
//     Arr([(u16, NonZeroU16); 4]),
//     Vec(Vec<(u16, u16)>),
// }
// log!("Store_u15_NoRT_big: {}", size_of::<Store_u15_NoRT_big>());

// enum Store_u15_NoRT_small {
//     Arr([(u16, u16); 1]),
//     Vec(Vec<(u16, u16)>),
// }
// log!("Store_u15_NoRT_small: {}", size_of::<Store_u15_NoRT_small>());

// // u15 NoRT
// // When arr has 1 elements, then the enum size 12, the same as for a vec.
// // When the arr has 2-4 element the size jumps to 16 bytes

// enum Store_u15_RT_big {
//     Arr([(u16, u16, u16); 3]),
//     Vec(Vec<(u16, u16, u16)>),
// }
// log!("Store_u15_RT_big: {}", size_of::<Store_u15_RT_big>());

// enum Store_u15_RT_small {
//     Arr([(u16, u16, u16); 2]),
//     Vec(Vec<(u16, u16, u16)>),
// }
// log!("Store_u15_RT_small: {}", size_of::<Store_u15_RT_small>());

// // u15 RT
// // When arr has 1-2 elements, then the enum size is 16
// // for higher counts it grows with 4 bytes per
