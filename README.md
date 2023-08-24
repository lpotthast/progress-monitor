# Progress-Monitor

Track progress of any type in your Rust application.

Allows you to track part of your work with individual scales.

## Usage

```rust
use std::{fmt::Debug, thread, time::Duration};
use error::CloseError;
use progress_monitor::prelude::*;

fn main() {
    // Numeric work
    let mut mon = CallbackProgressMonitor::new("root", 300, |a: &NumericWork<u64>, w: &NumericWork<u64>| {
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

    // Set work
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum MyWork {
        ResourcesLoaded,
        ConnectionEstablished,
        SystemInitialized,
    }
    let mut mon = CallbackProgressMonitor::new(
        "root",
        &[MyWork::ResourcesLoaded, MyWork::SystemInitialized],
        |a: &SetWork<MyWork>, w: &SetWork<MyWork>| println!("{}/{}", w, a),
    );
    thread::sleep(Duration::from_secs(1));
    mon.worked(MyWork::ResourcesLoaded);
    thread::sleep(Duration::from_secs(1));
    mon.worked(MyWork::ConnectionEstablished);
    thread::sleep(Duration::from_secs(1));
    mon.worked(MyWork::SystemInitialized);
    mon.close().unwrap();
}
```
