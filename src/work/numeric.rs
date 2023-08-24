use std::{
    fmt::Debug,
    fmt::Display,
    ops::{Add, Sub},
};

use num::{FromPrimitive, Num, ToPrimitive};

use crate::work::{AddError, Work};

pub trait NumReq: Num + ToPrimitive + FromPrimitive + PartialOrd + Debug + Display + Clone {}

impl<T: Num + ToPrimitive + FromPrimitive + PartialOrd + Debug + Display + Clone> NumReq for T {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NumericWork<N: NumReq>(N);

impl<N: NumReq> Work for NumericWork<N> {
    type Type = N;

    fn new<A: Into<Self::Type>>(value: A) -> Self {
        Self(value.into())
    }

    fn zero() -> Self {
        Self::new(N::zero())
    }

    fn is_zero(&self) -> bool {
        N::is_zero(&self.0)
    }

    fn min<'a>(a: &'a Self, b: &'a Self) -> &'a Self {
        if a.0 < b.0 {
            a
        } else {
            b
        }
    }

    fn parent_work_done_when(
        sub_work_done: Self,
        of_total_sub_work: Self,
        of_parent_work: Self,
    ) -> Self {
        let sub_work_done = sub_work_done.0.to_f64().expect("representable as f64");
        let of_total_sub_work = of_total_sub_work.0.to_f64().expect("representable as f64");
        let of_parent_work = of_parent_work.0.to_f64().expect("representable as f64");

        let rel = sub_work_done / of_total_sub_work * of_parent_work;

        Self::new(N::from_f64(rel).expect("cast from f64 to N"))
    }
}

impl<N: NumReq> Display for NumericWork<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl<N: NumReq> Add for NumericWork<N> {
    type Output = Result<Self, AddError>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(Self::new(self.0 + rhs.0))
    }
}

impl<N: NumReq> Sub for NumericWork<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl<N: NumReq> From<N> for NumericWork<N> {
    fn from(value: N) -> Self {
        Self::new(value)
    }
}
