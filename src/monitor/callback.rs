use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use crate::{prelude::ChildMonitor, work::Work, CloseError};

use super::{ProgressMonitor, ProgressMonitorDivision};

pub struct CallbackProgressMonitor<'n, W: Work, C: Fn(&W, &W)> {
    name: Cow<'n, str>,
    work: W,
    work_done: W,
    callback: C,
    closed: Option<Result<(), CloseError>>, // TODO: Box error, as error is unlikely?
}

impl<'n, W, C> Debug for CallbackProgressMonitor<'n, W, C>
where
    W: Work,
    C: Fn(&W, &W),
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackProgressMonitor")
            .field("name", &self.name)
            .field("work", &self.work)
            .field("work_done", &self.work_done)
            .finish()
    }
}

impl<'n, W, C> CallbackProgressMonitor<'n, W, C>
where
    W: Work,
    C: Fn(&W, &W),
{
    pub fn new<N: Into<Cow<'n, str>>, A: Into<W>>(name: N, work: A, callback: C) -> Self {
        Self {
            name: name.into(),
            work: work.into(),
            work_done: W::zero(),
            callback,
            closed: None,
        }
    }
}

impl<'n, W, C> ProgressMonitor<W> for CallbackProgressMonitor<'n, W, C>
where
    W: Work,
    C: Fn(&W, &W),
{
    fn worked<A: Into<W>>(&mut self, amount: A) {
        let now = (self.work_done.clone() + amount.into()).unwrap(); // TODO: Handle error!
        if now > self.work {
            // TOOD: Handle overshoot: e.g. require Min trait implementation, return min(possibly_overshootet, max_work)
            println!("now: {now:?}");
            println!("work: {:?}", self.work);
            panic!("");
        }
        self.work_done = now;
        (self.callback)(&self.work, &self.work_done);
    }

    /// Get the total amount of work.
    fn total(&self) -> &W {
        &self.work
    }

    /// Get the amount of work completed.
    fn completed(&self) -> &W {
        &self.work_done
    }

    /// Get the amount of work remaining.
    fn remaining(&self) -> Cow<W> {
        Cow::Owned(self.work.clone() - self.work_done.clone())
    }

    /// Retruns a percentage value ranging from 0.0 to 1.0.
    fn get_relative_amount_of_work_completed(&self) -> f64 {
        if self.work_done.is_zero() {
            0.0
        } else {
            self.work.clone().div_f64(self.work_done.clone())
        }
    }

    fn close(&mut self) -> Result<(), crate::CloseError> {
        let work_left = self.remaining();
        let result = if work_left.as_ref() == &W::zero() {
            Ok(())
        } else {
            Err(crate::CloseError {
                msg: format!(
                    "Must not drop progress monitor {self:#?} when work left is {work_left}."
                ),
            })
        };
        self.closed = Some(result.clone());
        result
    }
}

impl<'p, 'n, N, W, A1, A2, C> ProgressMonitorDivision<'p, 'n, N, W, A1, A2>
    for CallbackProgressMonitor<'n, W, C>
where
    N: Into<Cow<'n, str>>,
    W: Work,
    A1: Into<W>,
    A2: Into<W>,
    C: Fn(&W, &W),
{
    fn new_child(
        &'p mut self,
        name: N,
        parent_work: A1,
        child_work: A2,
    ) -> ChildMonitor<'n, 'p, W, Self> {
        let parent_work: W = parent_work.into();
        let total_child_work: W = child_work.into();

        // TODO: As Result?
        assert!(&parent_work <= self.remaining().as_ref());

        ChildMonitor::new(name.into(), self, parent_work, total_child_work)
    }
}

impl<'n, W: Work, C: Fn(&W, &W)> Display for CallbackProgressMonitor<'n, W, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.work_done, self.work))
    }
}

impl<'n, W: Work, C: Fn(&W, &W)> Drop for CallbackProgressMonitor<'n, W, C> {
    fn drop(&mut self) {
        match &self.closed {
            // TODO: Replace closed with None variant, taking ownership of close result
            Some(result) => {
                assert!(result.is_ok());
            }
            None => {
                tracing::warn!("close() was not called on {self:?}!");
                self.close().expect("Successful close");
            }
        }
    }
}
