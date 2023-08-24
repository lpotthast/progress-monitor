use std::{fmt::Debug, thread, time::Duration};

use error::CloseError;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod error;
pub mod monitor;
pub mod work;

pub mod prelude {
    pub use crate::error::CloseError;
    pub use crate::monitor::callback::CallbackProgressMonitor;
    pub use crate::monitor::sub::SubMonitor;
    pub use crate::monitor::ProgressMonitor;
    pub use crate::monitor::ProgressMonitorDivision;
    pub use crate::work::numeric::NumericWork;
    pub use crate::work::set::SetWork;
    pub use crate::work::Work;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MyWork {
    ResourcesLoaded,
    ConnectionEstablished,
    SystemInitialized,
}

fn main() {
    use prelude::*;

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut mon = CallbackProgressMonitor::new(
        "root",
        &[MyWork::ResourcesLoaded, MyWork::SystemInitialized],
        |a: &SetWork<MyWork>, w: &SetWork<MyWork>| println!("{}/{}", w, a),
    );

    thread::sleep(Duration::from_secs(1));
    mon.worked(MyWork::ResourcesLoaded);
    thread::sleep(Duration::from_secs(1));
    mon.worked(MyWork::ConnectionEstablished);
    //thread::sleep(Duration::from_secs(1));
    //mon.worked(MyWork::ConnectionEstablished);
    thread::sleep(Duration::from_secs(1));
    mon.worked(MyWork::SystemInitialized);
    //let c = mon.new_child("sub", MyWork::SystemInitialized, MyWork::SystemInitialized);

    mon.close().unwrap();

    let mut mon = CallbackProgressMonitor::new("root", 300, |a: &NumericWork, w: &NumericWork| {
        println!("{}/{}", w, a)
    });
    mon.worked(1);
    thread::sleep(Duration::from_secs(1));
    mon.worked(99);

    {
        let mut sub = mon.new_child("a", 100, 5000);
        thread::sleep(Duration::from_millis(500));
        sub.worked(1000);
        thread::sleep(Duration::from_millis(500));
        sub.worked(1000);
        thread::sleep(Duration::from_millis(500));
        sub.worked(1000);
        thread::sleep(Duration::from_millis(500));
        sub.worked(1000);
        {
            let mut subsub = sub.new_child("b".to_string(), 1000, 10);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(2);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(2);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(2);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(2);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(2);
            subsub.close().unwrap();
        }
        sub.close().unwrap();
    }

    thread::sleep(Duration::from_secs(1));
    mon.worked(100);
    mon.close().unwrap();
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test() {
        let mut mon =
            CallbackProgressMonitor::new("root", 300, |a: &NumericWork, w: &NumericWork| {
                println!("{}/{}", w, a)
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
        drop(sub);

        mon.worked(120);
    }
}
