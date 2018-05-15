use std::{thread, time};
extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::os::raw::c_char;
use std::os::raw::c_void;

fn main() {
    unsafe {
        let c_name = std::ffi::CString::new("BLABLA").unwrap();
        let p: *mut *mut c_char = std::ptr::null_mut();
        ht_init(0, p);

        //TODO, have ** HT_FileDumpListener in the api instead of reaquiring a pointer to listener
        let mut listener = HT_FileDumpListener {
            buffer: HT_ListenerBuffer {
                data: std::ptr::null_mut() as _,
                max_size: 0,
                usage: 0,
            },
            p_file: std::ptr::null_mut(),
            mtx: std::ptr::null_mut(),
        };
        let buffer_size = 4096; // size of internal listener's buffer
        println!("Listener: {:?}", listener);
        let file_name = std::ffi::CString::new("file_name.htdump").unwrap();
        ht_file_dump_listener_init(&mut listener as _, file_name.as_ptr(), buffer_size); // initialize listener

        println!("Listener: {:?}", listener);

        let timeline = ht_global_timeline_get();
        ht_timeline_register_listener(
            timeline,
            Some(ht_file_dump_listener_callback),
            &mut listener as *mut _ as *mut c_void,
        );

        ht_feature_callstack_start_string(timeline, c_name.as_ptr());
        thread::sleep(time::Duration::from_millis(10000));
        ht_feature_callstack_stop(timeline);
        ht_timeline_flush(timeline);

        ht_file_dump_listener_deinit(&mut listener as _);
        ht_deinit();
    }
}
