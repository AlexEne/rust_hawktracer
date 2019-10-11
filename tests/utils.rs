extern crate rust_hawktracer;
use std::{thread, time};
use rust_hawktracer::*;

#[hawktracer(interesting)]
fn method_to_profile() {
    thread::sleep(time::Duration::from_millis(10));
}

pub fn do_work() { 
    scoped_tracepoint!(_work);

    for _ in 0..3 {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(30));
    }

    {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(10));
    }

    method_to_profile();
}
