#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
#[cfg(feature = "profiling_enabled")]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        let tracepoint_name = concat!(stringify!($name), "\0");
        ScopedTracepoint::start_trace(tracepoint_name.as_ptr() as _);
        let $name = ScopedTracepoint{};
    };
}

#[macro_export]
#[cfg(not(feature = "profiling_enabled"))]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        ()
    };
}

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
impl Drop for ScopedTracepoint {
    fn drop(&mut self) {
        unsafe {
            ht_feature_callstack_stop(ht_global_timeline_get());
        }
    }
}

#[cfg(feature = "profiling_enabled")]
pub fn start_hawktracer<S: Into<String>>(file_name: S, buffer_size: usize) -> *mut _HT_FileDumpListener {
    let file_name = std::ffi::CString::new(file_name.into()).unwrap();
    
    unsafe {
        use std::os::raw::c_char;
        let p: *mut *mut c_char = std::ptr::null_mut();
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
        ht_registry_push_all_klass_info_events(ht_global_timeline_get());
        listener 
    }
}

#[cfg(feature = "profiling_enabled")]
pub fn stop_hawktracer(listener: *mut _HT_FileDumpListener) {
    unsafe {
        ht_timeline_flush(ht_global_timeline_get());
        ht_file_dump_listener_destroy(listener);
        ht_deinit();
    }
}

#[cfg(not(feature = "profiling_enabled"))]
pub fn stop_hawktracer(listener: *mut _HT_FileDumpListener) {
}

#[cfg(not(feature = "profiling_enabled"))]
pub fn start_hawktracer<S: Into<String>>(_file_name: S, _buffer_size: usize) -> *mut _HT_FileDumpListener {
    std::ptr::null_mut() as _
}