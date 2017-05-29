#[macro_use]
extern crate slog;
extern crate slog_term;

use std::sync::Mutex;

use slog::Drain;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
    let log = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

    trace!(log, "logging a trace message");
    debug!(log, "debug values"; "x" => 1, "y" => -1);
    info!(log, "some interesting info"; "where" => "right here");
    warn!(log, "be cautious!"; "why" => "you never know...");
    error!(log, "wrong {}", "foobar"; "type" => "unknown");
    crit!(log, "abandoning test");
}
