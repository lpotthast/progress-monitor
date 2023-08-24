use progress_monitor::prelude::*;
use std::{thread, time::Duration};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
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
