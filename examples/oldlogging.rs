#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_scope;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

use std::io;

use slog::Drain;

fn main() {
    let decorator = slog_term::PlainSyncDecorator::new(io::stderr());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let logger = slog::Logger::root(drain, o!("version" => "0.5"));

    // slog_stdlog uses the logger from slog_scope, so set a logger there
    let _guard = slog_scope::set_global_logger(logger);

    // register slog_stdlog as the log handler with the log crate
    slog_stdlog::init().unwrap();

    info!("standard logging redirected to slog");
}
