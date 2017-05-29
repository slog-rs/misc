#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;

fn main() {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();

    let log = Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION"), "build-id" => "8dfljdf"));

    let log = log.new(o!("owned-fast-lazy" => {
        PushFnValue(move |info, ser| {
            // no need for new allocations
            ser.emit(info.file())
        })
    }));

    debug!(log, "debug"; "fast-lazy" =>
           PushFnValue(move |info, ser| {
               // no need for new allocations
               ser.emit(info.msg())
           })
    );

    trace!(log, "debug"; "drop-fast-lazy" =>
        PushFnValue(move |_, _| {
            // drop of `ser` will emit unit (`()`/`void`) value
            Ok(())
        })
    );

}
