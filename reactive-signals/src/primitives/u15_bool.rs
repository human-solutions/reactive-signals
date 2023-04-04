#![allow(dead_code)]

use std::num::NonZeroU16;

/// A u15 (unsigned integer) with one bit used for representing a boolean.
/// It's main purpose is to transparently add on a boolean to a number.
/// The boolean is typically used for indicated if the value is "dirty"
/// "initialized".
///
/// A `u15Bool` is optimized so that the memory size of a `u15Bool`
/// and an `Option<u15Bool>` is the same.
///
/// ## **IMPORTANT**
/// The boolean is transparent, i.e. it is not used in equality or
/// ordering operations.
///
/// ```ignore
/// let v1 = u15Bool::new(0, false);
/// let v2 = u15Bool::new(0, true);
/// assert_eq!(v1, v2);
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone)]
pub struct u15Bool(NonZeroU16);

const LAST_BIT_ONLY: u16 = 0b0000_0000_0000_0001;
const NONE_LAST_BIT: u16 = 0b1111_1111_1111_1110;

// A number and a bool is stored as:
//
// - add one to the number
// - shift number bitwise once (leaving the last bit 0)
// - set the last bit according to the bool
// - store in a NonZeroU16
impl u15Bool {
    /// The maximum value the u15 accepts:
    ///
    /// `2^15 - 2 = 32_766`
    pub const MAX: u16 = 32_766;

    /// ## Warning
    /// Panics if the num (usize) is bigger than `u15Bool::MAX`
    pub fn new(num: usize, val: bool) -> Self {
        Self(set_last_bit_bool(shift_one(num as u16 + 1), val))
    }

    pub fn set_bool(&mut self, val: bool) {
        self.0 = set_last_bit_bool(self.0.get(), val);
    }

    pub fn bool(&self) -> bool {
        get_bool(self.0)
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.as_u15() as usize
    }

    #[inline]
    pub fn as_u15(&self) -> u16 {
        unshift_one(self.0) - 1
    }
}

impl PartialEq for u15Bool {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unshift_one(self.0).eq(&unshift_one(other.0))
    }
}

impl Eq for u15Bool {}

impl PartialOrd for u15Bool {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unshift_one(self.0).partial_cmp(&unshift_one(other.0))
    }
}

impl Ord for u15Bool {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unshift_one(self.0).cmp(&unshift_one(other.0))
    }
}

#[inline]
fn shift_one(base_one: u16) -> u16 {
    base_one << 1
}

#[inline]
fn unshift_one(base_one: NonZeroU16) -> u16 {
    base_one.get() >> 1
}

#[inline]
fn set_last_bit_bool(base_one: u16, val: bool) -> NonZeroU16 {
    NonZeroU16::new(if val {
        base_one | LAST_BIT_ONLY
    } else {
        base_one & NONE_LAST_BIT
    })
    .unwrap()
}

#[inline]
fn get_bool(non_zero: NonZeroU16) -> bool {
    (non_zero.get() & LAST_BIT_ONLY) != 0
}

#[test]
fn test_0_bool() {
    let val = u15Bool::new(0, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.as_u15(), 0);

    let val = u15Bool::new(0, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.as_u15(), 0);
}

#[test]
fn test_1_bool() {
    let val = u15Bool::new(1, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.as_u15(), 1);

    let val = u15Bool::new(1, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.as_u15(), 1);
}

#[test]
fn test_max_bool() {
    let mut val = u15Bool::new(u15Bool::MAX as usize, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.as_u15(), u15Bool::MAX);

    val.set_bool(true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.as_u15(), u15Bool::MAX);

    let mut val = u15Bool::new(u15Bool::MAX as usize, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.as_u15(), u15Bool::MAX);

    val.set_bool(false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.as_u15(), u15Bool::MAX);
}

#[test]
fn test_eq() {
    let max_true = u15Bool::new(u15Bool::MAX as usize, true);
    let max_false = u15Bool::new(u15Bool::MAX as usize, false);
    let zero_true = u15Bool::new(0 as usize, true);
    let zero_false = u15Bool::new(0 as usize, false);
    let one_true = u15Bool::new(1 as usize, true);
    let one_false = u15Bool::new(1 as usize, false);

    assert_eq!(max_true, max_false);
    assert_eq!(zero_true, zero_false);
    assert_eq!(one_true, one_false);
    assert_ne!(one_true, zero_true);
    assert_ne!(one_false, zero_true);
    assert_ne!(max_true, zero_false);
}
