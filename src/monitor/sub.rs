use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use crate::{work::Work, CloseError};

use super::{ProgressMonitor, ProgressMonitorDivision};

#[derive(Debug)]
pub struct SubMonitor<'n, 'p, W: Work, P: ProgressMonitor<W>> {
    name: Cow<'n, str>,
    /// A reference to the parent progress monitor. It must be mutable. When we do some work, our parent must also advance.
    parent: &'p mut P,
    /// Tells how much work of the parent is handled by this child.
    parent_work: W,
    /// Provides a new scale to work with. If this work is completed, the parent_work is completed.
    sub_work: W,
    /// How much sub_work was completed.
    sub_work_completed: W,
    closed: Option<Result<(), CloseError>>, // TODO: Box error, as erorr is unlikely?
}

impl<'n, 'p, W: Work, P: ProgressMonitor<W>> SubMonitor<'n, 'p, W, P> {
    pub fn new(name: Cow<'n, str>, parent: &'p mut P, parent_work: W, sub_work: W) -> Self {
        Self {
            name,
            parent,
            parent_work,
            sub_work,
            sub_work_completed: W::zero(),
            closed: None,
        }
    }
}

impl<'n, 'p, W: Work, P: ProgressMonitor<W>> ProgressMonitor<W> for SubMonitor<'n, 'p, W, P> {
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

        // We have to advance out parent work.
        // There is the possibility that the given work was more than our total sub work. We have to catch that and clamp.
        let amount_of_work: W = W::min(&amount_of_work, &self.sub_work).clone();
        let parent_worked: W = W::parent_work_done_when(
            amount_of_work,
            self.sub_work.clone(),
            self.parent_work.clone(),
        );
        self.parent.worked(parent_worked);
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

    fn get_relative_amount_of_work_completed(&self) -> f64 {
        if self.sub_work_completed.is_zero() {
            0.0
        } else {
            self.sub_work
                .clone()
                .div_f64(self.sub_work_completed.clone())
        }
    }

    fn close(&mut self) -> Result<(), crate::CloseError> {
        let work_left = self.remaining();
        let result = if work_left.as_ref() == &W::zero() {
            Ok(())
        } else {
            Err(crate::CloseError { msg: format!("Must not drop progress monitor {self:#?} when work left is {work_left} which is != 0.") })
        };
        self.closed = Some(result.clone()); // Clone is ok, as our happy path is Copy.
        result
    }
}

impl<'p2, 'n2, 'p, 'n, N, W, A1, A2, P> ProgressMonitorDivision<'p, 'n, N, W, A1, A2>
    for SubMonitor<'n2, 'p2, W, P>
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
    ) -> SubMonitor<'n, 'p, W, Self> {
        let amount_of_parent_work: W = amount_of_parent_work.into();
        let amount_of_child_work: W = amount_of_child_work.into();

        // TODO: As Result?
        assert!(&amount_of_parent_work <= self.remaining().as_ref());

        SubMonitor::new(
            name.into(),
            self,
            amount_of_parent_work,
            amount_of_child_work,
        )
    }
}

impl<'n, 'p, W: Work, T: ProgressMonitor<W>> Display for SubMonitor<'n, 'p, W, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}/{}",
            self.sub_work_completed, self.sub_work
        ))
    }
}

impl<'n, 'p, W: Work, T: ProgressMonitor<W>> Drop for SubMonitor<'n, 'p, W, T> {
    fn drop(&mut self) {
        match &self.closed {
            Some(result) => result.clone().expect("Successful close"),
            None => {
                tracing::warn!("close() was not called on {self:?}!");
                self.close().expect("Successful close");
            }
        }
    }
}
