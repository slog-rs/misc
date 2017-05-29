#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;

fn main() {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();

    let log = Logger::root(
        drain,
        o!("place" =>
           FnValue(move |info| {
               format!("{}:{} {}",
                       info.file(),
                       info.line(),
                       info.module(),
                       )
           })
          )
        );

    debug!(log, "HERE");
}
