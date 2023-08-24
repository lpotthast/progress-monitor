use error::CloseError;

pub mod error;
pub mod monitor;
pub mod work;

pub mod prelude {
    pub use crate::error::CloseError;
    pub use crate::monitor::callback::CallbackProgressMonitor;
    pub use crate::monitor::sub::ChildMonitor;
    pub use crate::monitor::ProgressMonitor;
    pub use crate::monitor::ProgressMonitorDivision;
    pub use crate::work::numeric::NumericWork;
    pub use crate::work::set::SetWork;
    pub use crate::work::Work;
}

#[cfg(test)]
mod test {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    use crate::prelude::*;

    #[test]
    fn test() {
        // a builder for `FmtSubscriber`.
        let subscriber = FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(Level::TRACE)
            // completes the builder.
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let mut mon =
            CallbackProgressMonitor::new("root", 300, |a: &NumericWork, w: &NumericWork| {
                tracing::info!("{}/{}", w, a)
            });
        mon.worked(100);

        let mut sub = mon.new_child("a", 100, 5000);
        sub.worked(1000);
        sub.worked(1000);
        sub.worked(1000);
        sub.worked(1000);

        //let mut subsub = sub.new_child("b", 1000, 10);
        //subsub.worked(2);
        //subsub.worked(2);
        //subsub.worked(2);
        //subsub.worked(2);
        ////subsub.worked(2);
        //drop(subsub);
        tracing::info!("drop sub");
        sub.close().unwrap();
        drop(sub);
        tracing::info!("dropped sub");

        mon.worked(120);
    }
}
