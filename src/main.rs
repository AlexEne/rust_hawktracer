use std::{thread, time};
extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::os::raw::c_char;

fn main() {
    unsafe {
        let c_name = std::ffi::CString::new("BLABLA").unwrap();
        let p: *mut *mut c_char = std::ptr::null_mut();
        ht_init(0, p);

        let buffer_size = 4096; // size of internal listener's buffer
        let file_name = std::ffi::CString::new("file_name.htdump").unwrap();
        let listener = ht_file_dump_listener_create(
            file_name.as_ptr(),
            buffer_size,
            std::ptr::null_mut() as _,
        ); // initialize listener

        println!("Listener: {:?}", listener);

        let timeline = ht_global_timeline_get();
        ht_timeline_register_listener(
            timeline,
            Some(ht_file_dump_listener_callback),
            listener as _,
        );

        ht_feature_callstack_start_string(timeline, c_name.as_ptr());
        thread::sleep(time::Duration::from_millis(10));
        ht_feature_callstack_stop(timeline);
        ht_timeline_flush(timeline);

        ht_file_dump_listener_destroy(listener);
        ht_deinit();
    }
}
