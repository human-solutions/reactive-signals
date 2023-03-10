use std::{mem, num::NonZeroU16, rc::Rc};

use crate::{create_data_signal, create_func_signal, scope::Scope, tests::StringStore, Signal};

#[test]
fn test_signal_dep() {
    let cx = Scope::new_root();

    let num_sig = create_data_signal(cx, 5);

    let output = Rc::new(StringStore::new());
    let out = output.clone();

    let str_sig = create_func_signal(cx, move || out.push(format!("val: {}", num_sig.get())));

    str_sig.subscribe(num_sig);

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[test]
fn test_signal_func_val() {
    let cx = Scope::new_root();

    let num_sig = create_data_signal(cx, 5);

    let output = Rc::new(StringStore::new());

    let a_sig = create_func_signal(cx, move || format!("a{}", num_sig.get()));
    let b_sig = create_func_signal(cx, move || format!("b{}", num_sig.get()));

    a_sig.subscribe(num_sig);
    b_sig.subscribe(num_sig);

    let out = output.clone();
    let str_sig = create_func_signal(cx, move || {
        out.push(format!("{}-{}", a_sig.get(), b_sig.get()))
    });

    str_sig.subscribe(a_sig);
    str_sig.subscribe(b_sig);

    num_sig.set(4);

    assert_eq!(output.values(), "a5-b5, a5-b4, a4-b4");
}

#[test]
fn test_sizes() {
    assert_eq!(12, mem::size_of::<Signal<String>>());
    assert_eq!(12, mem::size_of::<Signal<usize>>());
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
