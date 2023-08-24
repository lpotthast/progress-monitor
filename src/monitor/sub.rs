use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use crate::{work::Work, CloseError};

use super::{ProgressMonitor, ProgressMonitorDivision};

/// A child monitor references a parent monitor.
/// It monitors a subset of it's parent's total work, named `parent_work`.
/// This monitors own scale is declared by `sub_work` and can be arbitrary.
/// A child monitor, as a main monitor, tracks its progress. See `sub_work_completed`.
/// Whenever work is being made in a child monitor, the parents progress is increased relative amount.
///
/// Example:
/// Given a ChildMonitor with
/// - parent_work == 10
/// - sub_work == 1000
/// When a work(500) is submitted
/// Then the parent.work(5) is submitted.
#[derive(Debug)]
pub struct ChildMonitor<'n, 'p, W: Work, P: ProgressMonitor<W>> {
    name: Cow<'n, str>,
    /// A reference to the parent progress monitor. It must be mutable. When we do some work, our parent must also advance.
    parent: &'p mut P,
    /// Tells how much work of the parent is handled by this child.
    parent_work: W,
    /// Provides a new scale to work with. If this work is completed, the parent_work is completed.
    sub_work: W,
    /// How much sub_work was completed.
    sub_work_completed: W,
    /// Tracks thr amount of work submitted to the parent. Must equal `parent_work` when closing this child monitor!
    parent_work_submitted: W,
    closed: Option<Result<(), CloseError>>, // TODO: Box error, as erorr is unlikely?
}

impl<'n, 'p, W: Work, P: ProgressMonitor<W>> ChildMonitor<'n, 'p, W, P> {
    pub fn new(name: Cow<'n, str>, parent: &'p mut P, parent_work: W, sub_work: W) -> Self {
        Self {
            name,
            parent,
            parent_work,
            sub_work,
            sub_work_completed: W::zero(),
            parent_work_submitted: W::zero(),
            closed: None,
        }
    }

    pub fn name(&self) -> Cow<'n, str> {
        self.name.clone()
    }
}

impl<'n, 'p, W: Work, P: ProgressMonitor<W>> ProgressMonitor<W> for ChildMonitor<'n, 'p, W, P> {
    fn worked<A: Into<W>>(&mut self, amount_of_work: A) {
        let amount_of_work: W = amount_of_work.into();

        // Advance the work we have done, while preventing overshooting.
        let now: W = (self.sub_work_completed.clone() + amount_of_work.clone()).unwrap();
        if now > self.sub_work {
            // Would overshoot! Just clamp to maximum work possible.
            self.sub_work_completed = self.sub_work.clone();
        } else {
            self.sub_work_completed = now;
        }

        let finished = self.sub_work_completed == self.sub_work;

        // We have to advance our parent work.
        if !finished {
            // If this child monitor is not yet finished, we can dispatch parent work normally.
            // There is the possibility that the given work was more than our total sub work. We have to catch that and clamp.
            let amount_of_work: W = W::min(&amount_of_work, &self.sub_work).clone();
            let parent_worked: W = W::parent_work_done_when(
                amount_of_work,
                self.sub_work.clone(),
                self.parent_work.clone(),
            );
            self.parent.worked(parent_worked.clone());
            let new_parent_work_submitted =
                W::add(self.parent_work_submitted.clone(), parent_worked)
                    .unwrap()
                    .clone();
            self.parent_work_submitted = new_parent_work_submitted;
        } else {
            // If this child monitor did all its work, we dispatch all the remaining parent work.
            // Why? We advance the parent work with relative work done.
            // Based on the actual work type W, this might only be computable with a loss of precision.
            // For example by truncating floating point data.
            // This may result in us not advancing the parent progress enough, so we simply push the remaining work.
            let remaining_parent_work =
                self.parent_work.clone() - self.parent_work_submitted.clone();
            self.parent.worked(remaining_parent_work.clone());
            let new_parent_work_submitted =
                W::add(self.parent_work_submitted.clone(), remaining_parent_work)
                    .unwrap()
                    .clone();
            self.parent_work_submitted = new_parent_work_submitted;
        }
    }

    fn total(&self) -> &W {
        &self.sub_work
    }

    fn completed(&self) -> &W {
        &self.sub_work_completed
    }

    fn remaining(&self) -> Cow<W> {
        Cow::Owned(self.sub_work.clone() - self.sub_work_completed.clone())
    }

    fn close(&mut self) -> Result<(), crate::CloseError> {
        if self.closed.is_none() {
            let work_left = self.remaining();
            let result = if work_left.as_ref() == &W::zero() {
                Ok(())
            } else {
                Err(crate::CloseError { msg: format!("Must not close progress monitor {self:#?} when work left is {work_left} which is != 0.") })
            };
            self.closed = Some(result.clone()); // Clone is ok, as our happy path is Copy.
            result
        } else {
            // TODO: Forbid multiple closes?
            self.closed.clone().unwrap()
        }
    }
}

impl<'p2, 'n2, 'p, 'n, N, W, A1, A2, P> ProgressMonitorDivision<'p, 'n, N, W, A1, A2>
    for ChildMonitor<'n2, 'p2, W, P>
where
    Self: ProgressMonitor<W> + Sized,
    N: Into<Cow<'n, str>>,
    W: Work,
    A1: Into<W>,
    A2: Into<W>,
    P: ProgressMonitor<W>,
{
    fn new_child(
        &'p mut self,
        name: N,
        amount_of_parent_work: A1,
        amount_of_child_work: A2,
    ) -> ChildMonitor<'n, 'p, W, Self> {
        let amount_of_parent_work: W = amount_of_parent_work.into();
        let amount_of_child_work: W = amount_of_child_work.into();

        // TODO: As Result?
        assert!(&amount_of_parent_work <= self.remaining().as_ref());

        ChildMonitor::new(
            name.into(),
            self,
            amount_of_parent_work,
            amount_of_child_work,
        )
    }
}

impl<'n, 'p, W: Work, T: ProgressMonitor<W>> Display for ChildMonitor<'n, 'p, W, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}/{}",
            self.sub_work_completed, self.sub_work
        ))
    }
}

impl<'n, 'p, W: Work, T: ProgressMonitor<W>> Drop for ChildMonitor<'n, 'p, W, T> {
    fn drop(&mut self) {
        match &self.closed {
            Some(result) => match result {
                Ok(()) => { /* do nothing */ }
                Err(err) => {
                    tracing::error!(
                        "SubMonitor was not successfully closed. Reason: {}",
                        err.msg
                    );
                }
            },
            None => {
                tracing::warn!("close() was not called on {self:?}!");
                self.close().expect("Successful close");
            }
        }
    }
}
