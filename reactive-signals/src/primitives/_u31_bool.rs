#![allow(dead_code)]

use std::num::NonZeroU32;

/// A u15 (unsigned integer) with one bit used for representing a boolean.
/// It's main purpose is to transparently add on a boolean to a number.
/// The boolean is typically used for indicated if the value is "dirty"
/// "initialized".
///
/// A `u31Bool` is optimized so that the memory size of a `u31Bool`
/// and an `Option<u31Bool>` is the same.
///
/// ## **IMPORTANT**
/// The boolean is transparent, i.e. it is not used in equality or
/// ordering operations.
///
/// ```ignore
/// let v1 = u31Bool::new(0, false);
/// let v2 = u31Bool::new(0, true);
/// assert_eq!(v1, v2);
/// ```
///

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone)]
pub(crate) struct u31Bool(NonZeroU32);

const LAST_BIT_ONLY: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001;
const NONE_LAST_BIT: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1110;

// A number and a bool is stored as:
//
// - add one to the number
// - shift number bitwise once (leaving the last bit 0)
// - set the last bit according to the bool
// - store in a NonZeroU32
impl u31Bool {
    /// The maximum value the u15 accepts:
    ///
    /// `2^31 - 2 = 2_147_483_646`

    pub const MAX: u32 = 2_147_483_646;

    /// ## Warning
    /// Panics if the num (usize) is bigger than `u15Bool::MAX`
    pub fn new(num: usize, val: bool) -> Self {
        Self(set_last_bit_bool(shift_one(num as u32 + 1), val))
    }

    pub fn set_bool(&mut self, val: bool) {
        self.0 = set_last_bit_bool(self.0.get(), val);
    }

    pub fn bool(&self) -> bool {
        get_bool(self.0)
    }

    #[inline]
    pub fn usize(&self) -> usize {
        self.u31() as usize
    }

    #[inline]
    pub fn u31(&self) -> u32 {
        unshift_one(self.0) - 1
    }
}

impl PartialEq for u31Bool {
    fn eq(&self, other: &Self) -> bool {
        unshift_one(self.0).eq(&unshift_one(other.0))
    }
}

impl Eq for u31Bool {}

impl PartialOrd for u31Bool {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unshift_one(self.0).partial_cmp(&unshift_one(other.0))
    }
}

impl Ord for u31Bool {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unshift_one(self.0).cmp(&unshift_one(other.0))
    }
}

#[inline]
fn shift_one(base_one: u32) -> u32 {
    base_one << 1
}

#[inline]
fn unshift_one(base_one: NonZeroU32) -> u32 {
    base_one.get() >> 1
}

#[inline]
fn set_last_bit_bool(base_one: u32, val: bool) -> NonZeroU32 {
    NonZeroU32::new(if val {
        base_one | LAST_BIT_ONLY
    } else {
        base_one & NONE_LAST_BIT
    })
    .unwrap()
}

#[inline]
fn get_bool(non_zero: NonZeroU32) -> bool {
    (non_zero.get() & LAST_BIT_ONLY) != 0
}

#[test]
fn test_0_bool() {
    let val = u31Bool::new(0, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u31(), 0);

    let val = u31Bool::new(0, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u31(), 0);
}

#[test]
fn test_1_bool() {
    let val = u31Bool::new(1, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u31(), 1);

    let val = u31Bool::new(1, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u31(), 1);
}

#[test]
fn test_max_bool() {
    let mut val = u31Bool::new(u31Bool::MAX as usize, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u31(), u31Bool::MAX);

    val.set_bool(true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u31(), u31Bool::MAX);

    let mut val = u31Bool::new(u31Bool::MAX as usize, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u31(), u31Bool::MAX);

    val.set_bool(false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u31(), u31Bool::MAX);
}
