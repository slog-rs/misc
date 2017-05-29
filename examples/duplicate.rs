#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_json;

use slog::Drain;

use std::io;
use std::sync::Mutex;

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let d1 = slog_term::FullFormat::new(plain).build().fuse();
    let d2 = Mutex::new(slog_json::Json::default(io::stdout())).fuse();
    let log = slog::Logger::root(slog::Duplicate::new(d1, d2).fuse(), o!("version" => env!("CARGO_PKG_VERSION")));

    trace!(log, "logging a trace message");
    debug!(log, "debug values"; "x" => 1, "y" => -1);
    info!(log, "some interesting info"; "where" => "right here");
    warn!(log, "be cautious!"; "why" => "you never know...");
    error!(log, "wrong {}", "foobar"; "type" => "unknown");
    crit!(log, "abandoning test");
}
