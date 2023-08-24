use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

pub mod numeric;
pub mod set;

#[derive(Debug)]
pub struct AddError {
    pub msg: String,
}

pub trait Work:
    Sized
    + Debug
    + Display
    + Add<Output = Result<Self, AddError>>
    + Sub<Output = Self>
    + PartialEq
    + PartialOrd
    + Clone
{
    type Type;

    fn new<A: Into<Self::Type>>(value: A) -> Self;
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn min<'a>(a: &'a Self, b: &'a Self) -> &'a Self;

    fn parent_work_done_when(
        sub_work_done: Self,
        of_total_sub_work: Self,
        of_parent_work: Self,
    ) -> Self;

    fn div_f64(self, rhs: Self) -> f64;
}
