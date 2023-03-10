#![allow(dead_code)]

use std::num::NonZeroU32;

#[allow(non_camel_case_types)]
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct u31Bool(NonZeroU32);

const LAST_BIT_ONLY: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001;
const NONE_LAST_BIT: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1110;
/// 2^31 - 2
const MAX: u32 = 2_147_483_646;

impl u31Bool {
    pub fn new(num: usize, val: bool) -> Self {
        Self(base_one(set_bool(shift_one(num as u32), val)))
    }

    pub fn set_bool(&mut self, val: bool) {
        self.0 = base_one(set_bool(base_zero(self.0), val));
    }

    pub fn set_u31(&mut self, num: u32) {
        let val = self.bool();
        self.0 = base_one(set_bool(num, val))
    }

    pub fn bool(&self) -> bool {
        get_bool(base_zero(self.0))
    }
    pub fn usize(&self) -> usize {
        self.u31() as usize
    }

    pub fn u31(&self) -> u32 {
        unshift_one(base_zero(self.0))
    }
}

#[inline]
fn shift_one(num: u32) -> u32 {
    num << 1
}

#[inline]
fn unshift_one(base_zero: u32) -> u32 {
    base_zero >> 1
}

#[inline]
fn set_bool(base_zero: u32, val: bool) -> u32 {
    if val {
        base_zero | LAST_BIT_ONLY
    } else {
        base_zero & NONE_LAST_BIT
    }
}

#[inline]
fn get_bool(base_zero: u32) -> bool {
    (base_zero & LAST_BIT_ONLY) != 0
}

#[inline]
fn base_one(val: u32) -> NonZeroU32 {
    NonZeroU32::new(val + 1).unwrap()
}

#[inline]
fn base_zero(val: NonZeroU32) -> u32 {
    val.get() - 1
}

// pub struct u15Bool(NonZeroU16);

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
    let mut val = u31Bool::new(MAX as usize, false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u31(), MAX);

    val.set_bool(true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u31(), MAX);

    val.set_u31(0);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u31(), 0);

    let mut val = u31Bool::new(MAX as usize, true);
    assert_eq!(val.bool(), true);
    assert_eq!(val.u31(), MAX);

    val.set_bool(false);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u31(), MAX);

    val.set_u31(0);
    assert_eq!(val.bool(), false);
    assert_eq!(val.u31(), 0);
}
