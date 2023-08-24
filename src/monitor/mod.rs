use std::{borrow::Cow, fmt::Debug, fmt::Display};

use crate::{prelude::CloseError, work::Work};

use self::sub::ChildMonitor;

pub mod callback;
pub mod sub;

// TODO: Impl addition / mult / div / min
// TODO: Make get_relative_amount_of_work_completed a method of an additional trait

pub trait ProgressMonitor<W: Work>: Debug + Display {
    fn worked<A: Into<W>>(&mut self, amount_of_work: A);

    fn total(&self) -> &W;

    fn completed(&self) -> &W;

    fn remaining(&self) -> Cow<W>;

    // TODO: remove this or move to own trait?
    fn get_relative_amount_of_work_completed(&self) -> f64;

    fn close(&mut self) -> Result<(), CloseError>;
}

pub trait ProgressMonitorDivision<'p, 'n, N, W, A1, A2>
where
    Self: ProgressMonitor<W> + Sized,
    N: Into<Cow<'n, str>>,
    W: Work,
    A1: Into<W>,
    A2: Into<W>,
{
    fn new_child(
        &'p mut self,
        name: N,
        parent_work: A1,
        total_child_work: A2,
    ) -> ChildMonitor<'n, 'p, W, Self>; // TODO: Return trait
}
