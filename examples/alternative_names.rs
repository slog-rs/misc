#[macro_use(slog_o,slog_b,slog_record,slog_record_static,slog_log,slog_trace,slog_debug,slog_info,slog_warn,slog_error,slog_crit,slog_kv)]
extern crate slog;

extern crate slog_term;

use slog::Drain;

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = slog::Logger::root(slog_term::FullFormat::new(plain).build().fuse(), slog_o!());

    slog_trace!(log, "logging a trace message");
    slog_debug!(log, "debug values"; "x" => 1, "y" => -1);
    slog_info!(log, "some interesting info"; "where" => "right here");
    slog_warn!(log, "be cautious!"; "why" => "you never know...");
    slog_error!(log, "wrong {}", "foobar"; "type" => "unknown");
    slog_crit!(log, "abandoning test");
}
