use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

use crate::work::{AddError, Work};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SetWork<T>(BTreeSet<T>)
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy;

impl<T> Display for SetWork<T>
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl<T> From<SetWork<T>> for f64
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy,
{
    fn from(val: SetWork<T>) -> Self {
        val.0.len() as f64
    }
}

impl<T> From<T> for SetWork<T>
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy,
{
    fn from(value: T) -> Self {
        let mut set = BTreeSet::new();
        set.insert(value);
        Self(set)
    }
}

impl<T, const N: usize> From<&[T; N]> for SetWork<T>
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy,
{
    fn from(value: &[T; N]) -> Self {
        let mut set = BTreeSet::new();
        for v in value {
            set.insert(*v);
        }
        Self(set)
    }
}

impl<T> Work for SetWork<T>
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy,
{
    type Type = BTreeSet<T>;

    fn new<A: Into<Self::Type>>(value: A) -> Self {
        SetWork(value.into())
    }

    fn zero() -> Self {
        SetWork(BTreeSet::new())
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    fn min<'a>(a: &'a Self, b: &'a Self) -> &'a Self {
        if a.0.len() < b.0.len() {
            a
        } else {
            b
        }
    }

    fn parent_work_done_when(
        _sub_work_done: Self,
        _of_total_sub_work: Self,
        _of_parent_work: Self,
    ) -> Self {
        todo!()
    }
}

impl<T> Add for SetWork<T>
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy,
{
    type Output = Result<Self, AddError>;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.0.append(&mut rhs.0);

        for r in rhs.0 {
            if self.0.contains(&r) {
                return Err(AddError {
                    msg: format!("Element {r:?} is already present."),
                });
            }
        }

        Ok(SetWork(self.0))
    }
}

impl<T> Sub for SetWork<T>
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Clone + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.0.difference(&rhs.0).map(|it| *it).collect();
        SetWork(diff)
    }
}
