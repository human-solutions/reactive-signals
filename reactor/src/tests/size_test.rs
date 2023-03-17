use std::{mem, num::NonZeroU16};

use crate::Signal;

#[test]
fn test_sizes() {
    assert_eq!(8, mem::size_of::<Signal<String>>());
    assert_eq!(8, mem::size_of::<Signal<usize>>());
}

#[test]
fn size_of_ref_cell_box() {
    use std::cell::RefCell;
    assert_eq!(std::mem::size_of::<RefCell<Box<usize>>>(), 16);
    assert_eq!(std::mem::size_of::<Box<usize>>(), 8);
}

#[allow(dead_code, non_camel_case_types)]
#[test]
fn test_subscriber_size() {
    use crate::signal_id::SignalId;
    use std::mem::size_of;

    // vec: a pointer, size and capacity
    assert_eq!(size_of::<Vec<SignalId>>(), 24);
    assert_eq!(size_of::<[(u16, u16); 1]>(), 4);

    enum Store_u15_NoRT_big {
        // uses more space if the NonZeroU16 is replaced with an ordinary u16
        Arr([(u16, NonZeroU16); 8]),
        Vec(Vec<(u16, u16)>),
    }
    assert_eq!(size_of::<Store_u15_NoRT_big>(), 32);

    enum Store_u15_NoRT_small {
        Arr([(u16, u16); 2]),
        Vec(Vec<(u16, u16)>),
    }
    assert_eq!(size_of::<Store_u15_NoRT_small>(), 24);

    // u15 NoRT
    // When arr has 2 elements, then the enum size 24, the same as for a vec.
    // When the arr has 3-8 element the size jumps to 32 bytes

    enum Store_u15_RT_big {
        Arr([(u16, u16, u16); 5]),
        Vec(Vec<(u16, u16, u16)>),
    }
    assert_eq!(size_of::<Store_u15_RT_big>(), 32);

    enum Store_u15_RT_small {
        Arr([(u16, u16, u16); 1]),
        Vec(Vec<(u16, u16, u16)>),
    }
    assert_eq!(size_of::<Store_u15_RT_small>(), 24);
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
