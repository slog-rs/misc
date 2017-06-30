#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate slog_atomic;
extern crate slog_async;

use slog::*;
use slog_atomic::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;

use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

fn slow_fib(n: u64) -> u64 {
    match n {
        0 | 1 | 2 => 1,
        n => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

fn main() {
    // Create a new drain hierarchy, for the need of your program.
    // Choose from collection of existing drains, or write your own
    // `struct`-s implementing `Drain` trait.
    let decorator = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();


    // `AtomicSwitch` is a drain that wraps other drain and allows to change
    // it atomically in runtime.
    let drain = AtomicSwitch::new(drain);
    let ctrl = drain.ctrl();

    // Get a root logger that will log into a given drain.
    //
    // Note `o!` macro for more natural `OwnedKeyValue` sequence building.
    let root = Logger::root(
        drain.fuse(),
        o!("version" => env!("CARGO_PKG_VERSION"), "build-id" => "8dfljdf"),
    );

    // Build logging context as data becomes available.
    //
    // Create child loggers from existing ones. Children clone `key: value`
    // pairs from their parents.
    let log = root.new(o!("child" => 1));

    // Closures can be used for values that change at runtime.
    // Data captured by the closure needs to be `Send+Sync`.
    let counter = Arc::new(AtomicUsize::new(0));
    let log = log.new(o!("counter" => {
        let counter = counter.clone();
        // Note the `move` to capture `counter`,
        // and unfortunate `|_ : &_|` that helps
        // current `rustc` limitations. In the future,
        // a `|_|` could work.
        slog::FnValue(
            move |_ : &Record| { counter.load(SeqCst) }
                )
            }));

    // Loggers  can be cloned, passed between threads and stored without hassle.
    let join = thread::spawn({
        let log = log.clone();
        move || {

            info!(log, "before-fetch-add"); // counter == 0
            counter.fetch_add(1, SeqCst);
            info!(log, "after-fetch-add"); // counter == 1

            let drain = Mutex::new(slog_json::Json::default(std::io::stderr()));

            // `AtomicSwitch` drain can swap it's interior atomically (race-free).
            ctrl.set(
                // drains are composable and reusable
                slog::LevelFilter::new(drain, Level::Info)
                .map(slog::Fuse)
            );

            // Closures can be used for lazy evaluation:
            // This `slow_fib` won't be evaluated, as the current drain discards
            // "trace" level logging records.
            debug!(log, "debug"; "lazy-closure" => FnValue(|_ : &Record| slow_fib(40)));

            info!(log, "subthread"; "stage" => "start");
            thread::sleep(Duration::new(1, 0));
            info!(log, "subthread"; "stage" => "end");
        }
    });

    join.join().unwrap();
}
