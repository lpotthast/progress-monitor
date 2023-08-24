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
        |a: &SetWork<MyWork>, w: &SetWork<MyWork>| tracing::info!("{}/{}", w, a),
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
}
