#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! scoped_tracepoint {
    ($name:ident) => {
        let tracepoint_name = concat!(stringify!($name), "\0");
        unsafe {
            ht_feature_callstack_start_string(ht_global_timeline_get(), tracepoint_name.as_ptr() as _);
        }
        let $name = ScopedTracepoint{}
    };
}


pub struct ScopedTracepoint;

impl Drop for ScopedTracepoint {
    fn drop(&mut self) {
        unsafe {
            ht_feature_callstack_stop(ht_global_timeline_get());
        }
    }
}