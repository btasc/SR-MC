use std::{
    ops::{Mul, Add, Sub},
    cmp::{Ordering}
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApInt {
    width: usize,
    core: i64,
}

impl ApInt {
    const MAX_WIDTH: usize = 63;

    pub fn new(width: usize) -> Self {
        debug_assert!(width <= Self::MAX_WIDTH);
        Self { width, core: 0 }
    }

    pub fn reduce(new_width: usize) {

    }
}

impl Mul for ApInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let width = self.width + rhs.width;
        debug_assert!(width <= Self::MAX_WIDTH);

        let core = self.core * rhs.core;

        Self { width, core }
    }
}

impl Add for ApInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.width, rhs.width);

        let width = self.width + 1;
        debug_assert!(width <= Self::MAX_WIDTH);

        let core = self.core + rhs.core;

        Self { width, core }
    }
}

impl Sub for ApInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.width, rhs.width);

        let width = self.width + 1;
        debug_assert!(width <= Self::MAX_WIDTH);

        let core = self.core - rhs.core;

        Self { width, core }
    }
}

impl PartialOrd for ApInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.core.cmp(&other.core).then(self.core.cmp(&other.core)))
    }
}