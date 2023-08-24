use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::work::{AddError, Work};

// TODO: Make generic
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NumericWork(u64);

impl Work for NumericWork {
    type Type = u64;

    fn new<A: Into<Self::Type>>(value: A) -> NumericWork {
        NumericWork(value.into())
    }

    fn zero() -> NumericWork {
        NumericWork(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn min<'a>(a: &'a NumericWork, b: &'a NumericWork) -> &'a NumericWork {
        if a.0 < b.0 {
            a
        } else {
            b
        }
    }

    fn mul_f64(self, rhs: Self) -> f64 {
        self.0 as f64 * rhs.0 as f64
    }

    fn div_f64(self, rhs: Self) -> f64 {
        self.0 as f64 / rhs.0 as f64
    }

    fn parent_work_done_when(sub_work_done: Self, of_total_sub_work: Self, of_parent_work: Self) -> Self {
        NumericWork((sub_work_done.0 as f64 / of_total_sub_work.0  as f64 * of_parent_work.0 as f64) as u64)
    }
}

impl Display for NumericWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Add for NumericWork {
    type Output = Result<Self, AddError>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(Self(self.0 + rhs.0))
    }
}

impl Sub for NumericWork {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        NumericWork(self.0 - rhs.0)
    }
}

impl Into<NumericWork> for i32 {
    fn into(self) -> NumericWork {
        NumericWork(self as u64)
    }
}

impl Into<NumericWork> for u32 {
    fn into(self) -> NumericWork {
        NumericWork(self as u64)
    }
}

impl Into<NumericWork> for u64 {
    fn into(self) -> NumericWork {
        NumericWork(self)
    }
}

impl From<f64> for NumericWork {
    fn from(val: f64) -> Self {
        NumericWork(val as u64)
    }
}

impl From<NumericWork> for f64 {
    fn from(val: NumericWork) -> Self {
        val.0 as f64
    }
}
