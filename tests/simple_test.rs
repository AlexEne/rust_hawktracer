use std::{thread, time};

#[macro_use]
extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::fs;

#[test]
fn simple_tracing_test() {
    let file_name = "file_name.htdump";
    fs::remove_file(file_name);
    let instance = start_hawktracer(file_name, 4096);
    {
        for _ in 0..3 {
            scoped_tracepoint!(_test);
            thread::sleep(time::Duration::from_millis(30));
        }
    }
    {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(10));
    }

    stop_hawktracer(instance);

    let metadata = fs::metadata(file_name).unwrap();
    assert!(metadata.len() > 0);
}


