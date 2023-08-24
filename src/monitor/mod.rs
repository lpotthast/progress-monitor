use std::{borrow::Cow, fmt::Debug, fmt::Display};

use crate::{prelude::CloseError, work::Work};

use self::sub::ChildMonitor;

pub mod callback;
pub mod sub;

/// A ProgressMonitor tracks an amount of work which must be completed.
pub trait ProgressMonitor<W: Work>: Debug + Display {
    fn worked<A: Into<W>>(&mut self, amount_of_work: A);

    fn total(&self) -> &W;

    fn completed(&self) -> &W;

    fn remaining(&self) -> Cow<W>;

    /// If you are done with your work, close this monitor.
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
        // A reference to the parent monitor.
        &'p mut self,
        // The name of the child monitor. For debug purposes only.
        name: N,
        // The amount of parent work this child monitor is responsible for. Must be <= the remaining work of the given parent!
        parent_work: A1,
        // The child monitors scale for the work taken from the parent monitor. Can be arbitrary.
        child_work: A2,
    ) -> ChildMonitor<'n, 'p, W, Self>;
}
