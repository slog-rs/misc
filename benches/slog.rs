#![feature(test)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_std;

extern crate test;

use std::io;
use test::Bencher;
use slog::*;
use slog_std::Async;
use std::sync::Mutex;

const LONG_STRING : &'static str = "A long string that would take some time to allocate";

struct BlackBoxDrain;

fn o_10() -> slog::OwnedKV {
    o!(
        "u8" => 0u8,
        "u16" => 0u16,
        "u32" => 0u32,
        "u64" => 0u64,
        "bool" => false,
        "str" => "",
        "f32" => 0f32,
        "f64" => 0f64,
        "option" => Some(0),
        "unit" => (),
        )
}

impl Drain for BlackBoxDrain {
    type Ok = ();
    type Err= ();
    fn log(&self, ri: &Record, o : &OwnedKVList) -> std::result::Result<(), ()> {

        test::black_box((ri, o));
        Ok(())
    }
}

struct BlackBoxWriter;

impl io::Write for BlackBoxWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        test::black_box(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn async_json_blackbox() -> impl Drain<Err=(), Ok=()> {
    let json = slog_json::default(BlackBoxWriter).fuse();
    let async : Async = Async::custom(json).chan_size(1024 * 1024 * 16).build();
    async.ignore_res()
}

fn empty_json_blackbox() -> impl Drain<Err=(), Ok=()> {
    Mutex::new(slog_json::custom(BlackBoxWriter).build().fuse()).ignore_res()
}

fn json_blackbox() -> impl Drain<Err=(), Ok=()> {
    Mutex::new(slog_json::default(BlackBoxWriter).fuse()).ignore_res()
}

#[bench]
fn log_filter_out_empty_x100(b: &mut Bencher) {
    let log = Logger::root(LevelFilter::new(BlackBoxDrain, Level::Warning).ignore_res(), o!());

    b.iter(|| {
        for _ in 0u32..100 {
            info!(log, "");
        }
    });
}


#[bench]
fn log_discard_00br_10ow_x100(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o_10());

    b.iter(|| {
        for _ in 0u32..100 {
            info!(log, "");
        }
    });
}

#[bench]
fn log_discard_10br_00ow_x100(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        for i in 0u32..100 {
            info!(log,
                  "";
                  "u8" => 0u8,
                  "u16" => 0u16,
                  "u32" => i,
                  "u64" => 0u64,
                  "bool" => false,
                  "str" => "",
                  "f32" => 0f32,
                  "f64" => 0f64,
                  "option" => Some(0),
                  "unit" => (),
                  );
        }
    });
}
#[bench]
fn log_discard_00br_00ow_x100(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        for _ in 0u32..100 {
            info!(log, "");
        }
    });
}



#[bench]
fn log_discard_u32val_x100(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        for i in 0u32..100 {
            info!(log, ""; "u32" => i);
        }
    });
}

#[bench]
fn log_discard_u32closure_x100(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        for i in 0u32..100 {
            info!(log, ""; "i32" => move |_:&Record|{i});
        }
    });
}

#[bench]
fn logger_clone(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn logger_clone_10prev(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o_10());

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn logger_child_00prev_00new(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        log.new(o!())
    });
}

#[bench]
fn logger_child_00prev_10new(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        log.new(o_10());
    });
}

#[bench]
fn logger_child_10prev_00new(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o_10());

    b.iter(|| {
        log.new(o!());
    });
}
#[bench]
fn logger_child_10prev_10new(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o_10());

    b.iter(|| {
        log.new(o_10());
    });
}


#[bench]
fn log_empty_json_blackbox_i32val(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";  "i32" => 5);
    });
}

#[bench]
fn log_empty_json_blackbox_i32closure(b: &mut Bencher) {

    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => |_:&Record|{5});
    });
}

#[bench]
fn log_empty_json_blackbox_i32pushclosure(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => PushFnValue(|_:&Record, ser|{
            ser.serialize(5)
        }));
    });
}



#[bench]
fn log_empty_json_blackbox_strclosure(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => |_:&Record| {
            String::from(LONG_STRING)
        });
    });
}

#[bench]
fn log_empty_json_blackbox_strpushclosure(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => PushFnValue(|_:&Record, ser|{
            ser.serialize(LONG_STRING)
        }));
    });
}

#[bench]
fn log_json_blackbox_i32val(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";  "i32" => 5);
    });
}

#[bench]
fn log_json_blackbox_10br_10ow(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(),  o_10());

    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn log_empty_json_blackbox_10br_10ow(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o_10());

    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn log_empty_json_blackbox_00br_10ow(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o_10());


    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_json_blackbox_10br_00ow(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());
    let log = log.new(o!(
            "u8" => 0u8,
            "u16" => 0u16,
            "u32" => 0u32,
            "u64" => 0u64,
            "bool" => false,
            "str" => "",
            "f32" => 0f32,
            "f64" => 0f64,
            "option" => Some(0),
            "unit" => (),
            ));

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_json_blackbox_i32closure(b: &mut Bencher) {

    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => |_:&Record|{5});
    });
}

#[bench]
fn log_json_blackbox_i32pushclosure(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => PushFnValue(|_:&Record, ser|{
            ser.serialize(5)
        }));
    });
}

#[bench]
fn log_json_blackbox_strclosure(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => |_:&Record| {
            String::from(LONG_STRING)
        });
    });
}

#[bench]
fn log_json_blackbox_strpushclosure(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => PushFnValue(|_:&Record, ser|{
            ser.serialize(LONG_STRING)
        }));
    });
}

#[bench]
fn log_async_json_blackbox_00br_00_ow(b: &mut Bencher) {
    let log = Logger::root(async_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_async_json_blackbox_00br_10_ow(b: &mut Bencher) {
    let log = Logger::root(async_json_blackbox(), o_10());

    b.iter(|| {
        info!(log, "");
    });
}


#[bench]
fn log_async_json_blackbox_10br_10ow(b: &mut Bencher) {
    let log = Logger::root(async_json_blackbox(), o_10());

    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn log_async_json_blackbox_10br_00ow(b: &mut Bencher) {
    let log = Logger::root(async_json_blackbox(), o!());


    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn tmp_file_write_1b(b: &mut Bencher) {
    use std::io::Write;

    let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/slog-test-1b").unwrap();

    b.iter(|| {
        f.write_all(&[0]).unwrap();
    });
}


#[bench]
fn tmp_file_write_1kib(b: &mut Bencher) {
    use std::io::Write;

    let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/slog-test-1k").unwrap();

    let buf = vec!(0u8; 1024);
    b.iter(|| {
        f.write_all(&buf).unwrap();
    });
}
