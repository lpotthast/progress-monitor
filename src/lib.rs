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
        tracing::subscriber::set_global_default(
            FmtSubscriber::builder()
                .with_max_level(Level::TRACE)
                .finish(),
        )
        .expect("setting default subscriber failed");

        let mut mon =
            CallbackProgressMonitor::new("root", 300, |a: &NumericWork<u64>, w: &NumericWork<u64>| {
                tracing::info!("{}/{}", w, a)
            });
        mon.worked(100);

        let mut sub = mon.new_child("a", 100, 3000);
        sub.worked(1000);
        sub.worked(1000);
        sub.worked(1000);
        sub.close().unwrap();
        drop(sub);

        mon.worked(120);
        mon.close().unwrap();
    }
}
