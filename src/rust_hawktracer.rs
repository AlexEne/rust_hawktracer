#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "profiling_enabled")]
pub struct ScopedTracepoint;

#[cfg(feature = "profiling_enabled")]
impl ScopedTracepoint {
    pub fn start_trace(name: *mut i8) {
        unsafe {
            ht_feature_callstack_start_string(ht_global_timeline_get(), name);
        }
    }
}

#[cfg(feature = "profiling_enabled")]
pub struct HawktracerInstance {
    listener: *mut _HT_FileDumpListener,
}

#[cfg(feature = "profiling_enabled")]
impl HawktracerInstance {
    pub fn new<S: Into<String>>(file_name: S, buffer_size: usize) -> HawktracerInstance {
        let file_name = std::ffi::CString::new(file_name.into()).unwrap();
        use std::os::raw::c_char;
        let p: *mut *mut c_char = std::ptr::null_mut();
        let listener = unsafe {
            ht_init(0, p);
            let listener = ht_file_dump_listener_create(
                file_name.as_ptr(),
                buffer_size,
                std::ptr::null_mut() as _,
            );

            ht_timeline_register_listener(
                ht_global_timeline_get(),
                Some(ht_file_dump_listener_callback),
                listener as _,
            );

            listener
        };

        HawktracerInstance { listener: listener }
    }
}

#[cfg(feature = "profiling_enabled")]
impl Drop for HawktracerInstance {
    fn drop(&mut self) {
        unsafe {
            ht_timeline_flush(ht_global_timeline_get());
            ht_timeline_unregister_all_listeners(ht_global_timeline_get());
            ht_file_dump_listener_destroy(self.listener);
            ht_deinit();
        }
    }
}

#[cfg(feature = "profiling_enabled")]
impl Drop for ScopedTracepoint {
    fn drop(&mut self) {
        unsafe {
            ht_feature_callstack_stop(ht_global_timeline_get());
        }
    }
}

#[cfg(feature = "profiling_enabled")]
pub fn create_hawktracer_instance<S: Into<String>>(
    file_name: S,
    buffer_size: usize,
) -> HawktracerInstance {
    let file_name = std::ffi::CString::new(file_name.into()).unwrap();
    use std::os::raw::c_char;
    let p: *mut *mut c_char = std::ptr::null_mut();
    let listener = unsafe {
        ht_init(0, p);
        let listener = ht_file_dump_listener_create(
            file_name.as_ptr(),
            buffer_size,
            std::ptr::null_mut() as _,
        );

        ht_timeline_register_listener(
            ht_global_timeline_get(),
            Some(ht_file_dump_listener_callback),
            listener as _,
        );

        listener
    };

    HawktracerInstance { listener: listener }
}

#[cfg(not(feature = "profiling_enabled"))]
pub fn create_hawktracer_instance<S: Into<String>>(_file_name: S, _buffer_size: usize) {}

#[cfg(not(feature = "profiling_enabled"))]
pub struct ScopedTracepoint;
