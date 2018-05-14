use std::{thread, time};
extern crate rust_hawktracer;
use std::os::raw::c_char;
use rust_hawktracer::*;

fn main() {
    unsafe {
        let c_name = std::ffi::CString::new("BLABLA").unwrap();
        let p: *mut *mut c_char = std::ptr::null_mut();
        ht_init(0, p);

        ht_feature_callstack_start_string(ht_global_timeline_get(), c_name.as_ptr());
        thread::sleep(time::Duration::from_millis(1000));
        ht_feature_callstack_stop(ht_global_timeline_get());
        ht_timeline_flush(ht_global_timeline_get());
        ht_deinit();
    }
}