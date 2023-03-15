#![allow(dead_code)]

use std::num::NonZeroU16;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct u15Bool(NonZeroU16);

const LAST_BIT_ONLY: u16 = 0b0000_0000_0000_0001;
const NONE_LAST_BIT: u16 = 0b1111_1111_1111_1110;
/// 2^15 - 2
const MAX: u16 = 32_766;

impl u15Bool {
    pub fn new(num: usize, val: bool) -> Self {
        Self(base_one(set_bool(shift_one(num as u16), val)))
    }

    pub fn set_bool(&mut self, val: bool) {
        self.0 = base_one(set_bool(base_zero(self.0), val));
    }

    pub fn set_u15(&mut self, num: u16) {
        let val = self.bool();
        self.0 = base_one(set_bool(num, val))
    }

    pub fn bool(&self) -> bool {
        get_bool(base_zero(self.0))
    }
    pub fn usize(&self) -> usize {
        self.u15() as usize
    }

    pub fn u15(&self) -> u16 {
        unshift_one(base_zero(self.0))
    }
}

#[inline]
fn shift_one(num: u16) -> u16 {
    num << 1
}

#[inline]
fn unshift_one(base_zero: u16) -> u16 {
    base_zero >> 1
}

#[inline]
fn set_bool(base_zero: u16, val: bool) -> u16 {
    if val {
        base_zero | LAST_BIT_ONLY
    } else {
        base_zero & NONE_LAST_BIT
    }
}

#[inline]
fn get_bool(base_zero: u16) -> bool {
    (base_zero & LAST_BIT_ONLY) != 0
}

#[inline]
fn base_one(val: u16) -> NonZeroU16 {
    NonZeroU16::new(val + 1).unwrap()
}

#[inline]
fn base_zero(val: NonZeroU16) -> u16 {
    val.get() - 1
}

// pub struct u15Bool(NonZeroU16);

#[test]
fn test_0_bool() {
    let val = u15Bool::new(0, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u15(), 0);

    let val = u15Bool::new(0, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u15(), 0);
}

#[test]
fn test_1_bool() {
    let val = u15Bool::new(1, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u15(), 1);

    let val = u15Bool::new(1, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u15(), 1);
}

#[test]
fn test_max_bool() {
    let mut val = u15Bool::new(MAX as usize, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u15(), MAX);

    val.set_bool(true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u15(), MAX);

    val.set_u15(0);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u15(), 0);

    let mut val = u15Bool::new(MAX as usize, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u15(), MAX);

    val.set_bool(false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u15(), MAX);

    val.set_u15(0);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u15(), 0);
}
