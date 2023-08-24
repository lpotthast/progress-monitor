use progress_monitor::prelude::*;
use std::{fmt::Debug, thread, time::Duration};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MyWork {
    ResourcesLoaded,
    ConnectionEstablished,
    SystemInitialized,
}

fn main() {
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

    thread::sleep(Duration::from_millis(500));
    mon.worked(MyWork::ResourcesLoaded);
    thread::sleep(Duration::from_millis(500));
    mon.worked(MyWork::ConnectionEstablished);
    thread::sleep(Duration::from_millis(500));
    // Submitting the same work twice is not a problem when working with sets. Could be an opt-in though!
    mon.worked(MyWork::ConnectionEstablished);
    thread::sleep(Duration::from_millis(500));
    mon.worked(MyWork::SystemInitialized);
    //let c = mon.new_child("sub", MyWork::SystemInitialized, MyWork::SystemInitialized);

    mon.close().unwrap();

    let mut mon =
        CallbackProgressMonitor::new("root", 300, |a: &NumericWork<u64>, w: &NumericWork<u64>| {
            println!("{}/{}", w, a)
        });
    mon.worked(1);
    thread::sleep(Duration::from_millis(500));
    mon.worked(99);

    {
        let mut sub = mon.new_child("a", 100, 5000);
        thread::sleep(Duration::from_millis(250));
        sub.worked(1000);
        thread::sleep(Duration::from_millis(250));
        sub.worked(1000);
        thread::sleep(Duration::from_millis(250));
        sub.worked(1000);
        thread::sleep(Duration::from_millis(250));
        sub.worked(1000);
        {
            let mut subsub = sub.new_child("b".to_string(), 1000, 11);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(3);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(3);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(2);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(2);
            thread::sleep(Duration::from_millis(100));
            subsub.worked(1);
            subsub.close().unwrap();
        }
        sub.close().unwrap();
    }

    thread::sleep(Duration::from_millis(500));
    mon.worked(100);
    mon.close().unwrap();
}
