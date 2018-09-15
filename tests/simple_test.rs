use std::{thread, time};

#[macro_use]
extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::fs;
use std::path::Path;

#[test]
fn simmple_tracing_test() {
    let file_name = "file_name.htdump";
    fs::remove_file(file_name);
    let _instance = create_hawktracer_instance(file_name, 4096);

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

    assert!(std::path::Path::new(file_name).exists());
}
