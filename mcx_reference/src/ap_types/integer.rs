use std::{
    ops::{Mul, Add, Sub},
    cmp::{Ordering}
};

use super::rounding::{Round, RoundMethod};

#[derive(Debug, Clone, Copy)]
pub struct ApInt {
    pub width: u32,
    pub core: i64,
}

impl ApInt {
    const MAX_WIDTH: u32 = 63;

    pub fn new(core: i64, width: u32) -> Self {
        debug_assert!(width <= Self::MAX_WIDTH);
        let ret = Self { width, core };

        ret.dbg_assert_core_width();
        ret
    }

    pub fn reduce(new_width: usize) {

    }

    #[inline]
    fn dbg_assert_core_width(&self) {
        debug_assert!(
            self.core_width() <= self.width,
            "Core width exceeds num width. Core width: {}, Num width: {}",
            self.core_width(), self.width,
        );
    }

    #[inline]
    fn dbg_assert_max_width(&self) {
        debug_assert!(
            self.width <= Self::MAX_WIDTH,
            "Width exceeded 63 bits of precision. Width: {}",
            self.width,
        )
    }

    fn core_width(&self) -> u32 {
        let us_core: u64 = self.core.unsigned_abs();
        u64::BITS - us_core.leading_zeros()
    }
}

impl Round for ApInt {
    const ROUND_METHOD: RoundMethod = RoundMethod::ToNearest;
    type Width = u32;

    fn get_fract_dist(&self, new_width: Self::Width) -> u32 {
        0
    }
}

impl Mul for ApInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let ret = Self {
            width: self.width + rhs.width,
            core: self.core * rhs.core,
        };

        ret.dbg_assert_max_width();
        ret.dbg_assert_core_width();

        ret
    }
}

impl Add for ApInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.width, rhs.width);

        let ret = Self {
            width: self.width + 1,
            core: self.core + rhs.core,
        };

        ret.dbg_assert_max_width();
        ret.dbg_assert_core_width();
        ret
    }
}

impl Sub for ApInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.width, rhs.width);

        let ret = Self {
            width: self.width + 1,
            core: self.core - rhs.core,
        };

        ret.dbg_assert_max_width();
        ret.dbg_assert_core_width();
        ret
    }
}

impl PartialOrd for ApInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.core.cmp(&other.core))
    }
}

impl PartialEq for ApInt {
    fn eq(&self, other: &Self) -> bool {
        self.core == other.core
    }
}