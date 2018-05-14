use std::{thread, time};
extern crate rust_hawktracer;
use rust_hawktracer::*;

fn main() {
    unsafe {
        let c_name = std::ffi::CString::new("BLABLA").unwrap();
        ht_feature_callstack_start_string(ht_global_timeline_get(), c_name.as_ptr());
        thread::sleep(time::Duration::from_millis(1000));
        ht_feature_callstack_stop(ht_global_timeline_get());
        ht_timeline_flush(ht_global_timeline_get());
    }
}